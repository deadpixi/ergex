use crate::queue::{End, Queue};
use crate::transformers::{
    ByteAlternationSimplifier, ByteTransformer, ClassSimplifier, GroupEliminator, Transformer,
    UnicodeAlternationSimplifier,
};
use crate::{ContinueMatching, Encoding, Error, MatchHandler, MatchMode, Submatch};

use crate::aho_corasick::{AhoCorasick, AhoCorasickScratch};
use crate::intset::{GrowSet, ShrinkSet};
use regex_syntax::hir::literal::Literals;
use regex_syntax::hir::{
    Anchor, Class, ClassBytes, Group, GroupKind, Hir, HirKind, Literal, Repetition, RepetitionKind,
    RepetitionRange, WordBoundary,
};
use regex_syntax::ParserBuilder;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::mem::swap;

const CR: u8 = 0x0d;
const NL: u8 = 0x0a;

pub struct Regex {
    // Identify this regular expression. This is passed to match callbacks,
    // and also used to disable this regex at runtime.
    id: usize,

    // The expression itself.
    expression: String,

    // The matching mode.
    mode: MatchMode,

    // Maximum number of instructions in the compiled program.
    max_instructions: usize,

    // Maximum repeat count in a repetition.
    max_repeat: usize,

    // Parser options.
    max_depth: u32,
    case_sensitive: bool,
    allow_whitespace: bool,
    allow_invalid_utf8: bool,
    dot_matches_new_line: bool,
    multi_line: bool,

    // A list of transformers to apply to this regular expression.
    preprocessors: Vec<Box<dyn Transformer>>,
    postprocessors: Vec<Box<dyn Transformer>>,

    // Should this expression default to Unicode-aware.
    encoding: Encoding,
}

impl Regex {
    pub fn new<T: AsRef<str>>(id: usize, expression: T) -> Self {
        Self {
            id,
            expression: expression.as_ref().to_string(),
            mode: MatchMode::All(Submatch::All),
            max_instructions: 1048576,
            max_repeat: 65535,
            max_depth: 250,
            case_sensitive: true,
            allow_whitespace: false,
            allow_invalid_utf8: false,
            dot_matches_new_line: false,
            multi_line: false,
            preprocessors: Vec::new(),
            postprocessors: vec![
                Box::new(UnicodeAlternationSimplifier::new()),
                Box::new(ClassSimplifier::new()),
                Box::new(ByteTransformer::new()),
                Box::new(ByteAlternationSimplifier::new()),
                Box::new(ClassSimplifier::new()),
            ],
            encoding: Encoding::UTF8,
        }
    }

    pub fn preprocess(mut self, processor: Box<dyn Transformer>) -> Self {
        self.preprocessors.push(processor);
        self
    }

    pub fn postprocess(mut self, processor: Box<dyn Transformer>) -> Self {
        self.postprocessors.push(processor);
        self
    }

    pub fn mode(mut self, mode: MatchMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    }

    pub fn allow_whitespace(mut self, allow_whitespace: bool) -> Self {
        self.allow_whitespace = allow_whitespace;
        self
    }

    pub fn allow_invalid_utf8(mut self, allow_invalid_utf8: bool) -> Self {
        self.allow_invalid_utf8 = allow_invalid_utf8;
        self
    }

    pub fn dot_matches_new_line(mut self, dot_matches_new_line: bool) -> Self {
        self.dot_matches_new_line = dot_matches_new_line;
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn max_instructions(mut self, max_instructions: usize) -> Self {
        self.max_instructions = max_instructions;
        self
    }

    pub fn max_repetition(mut self, max_repeat: usize) -> Self {
        self.max_repeat = max_repeat;
        self
    }

    pub fn multi_line(mut self, multi_line: bool) -> Self {
        self.multi_line = multi_line;
        self
    }

    pub fn encoding(mut self, encoding: Encoding) -> Self {
        self.encoding = encoding;
        self
    }

    pub fn build(mut self) -> Result<CompiledRegex, Error> {
        // Add a GroupEliminator to eliminate all captures that we don't need.
        // This makes things simpler on the compilation and match sides: the code can be
        // generic. This also makes it possible to more aggressively optimize.
        let max_capture = self.get_max_capture();
        self.preprocessors
            .push(Box::new(GroupEliminator::with_max_capture(max_capture)));

        // Perform our transformations.
        let mut hir = self.parse()?;
        for transformer in self.preprocessors {
            hir = transformer.transform(hir);
        }
        for transformer in self.postprocessors {
            hir = transformer.transform(hir);
        }

        // The compiled regex itself. We let a Compiler fill it in.
        let mut compiled = CompiledRegex {
            id: self.id,
            capture_count: 1,
            named_captures: HashMap::new(),
            char_classes: Vec::new(),
            mode: self.mode,
            program: Vec::new(),
            anchored_start: hir.is_anchored_start(),
            prefixes: if hir.is_anchored_start() {
                // Since we can early exit on anchored expressions,
                // there's no need to bloat the AC automaton with its prefixes.
                vec![]
            } else {
                Literals::prefixes(&hir)
                    .literals()
                    .iter()
                    .map(|x| x.to_vec())
                    .collect()
            },
        };

        Compiler::new(&mut compiled, self.max_repeat, self.max_instructions).build(hir.kind())?;
        Ok(compiled)
    }

    fn parse(&self) -> Result<Hir, Error> {
        Ok(ParserBuilder::new()
            .nest_limit(self.max_depth)
            .allow_invalid_utf8(self.allow_invalid_utf8 || self.encoding != Encoding::UTF8)
            .ignore_whitespace(self.allow_whitespace)
            .case_insensitive(!self.case_sensitive)
            .multi_line(self.multi_line)
            .dot_matches_new_line(self.dot_matches_new_line)
            .unicode(self.encoding == Encoding::UTF8)
            .build()
            .parse(&self.expression)?)
    }

    fn get_max_capture(&self) -> u32 {
        match self.mode {
            MatchMode::First(Submatch::All) => u32::MAX,
            MatchMode::First(Submatch::AtMost(n)) => n,
            MatchMode::First(Submatch::Expression) => 0,
            MatchMode::All(Submatch::All) => u32::MAX,
            MatchMode::All(Submatch::AtMost(n)) => n,
            MatchMode::All(Submatch::Expression) => 0,
        }
    }
}

pub struct CompiledRegex {
    id: usize,
    capture_count: usize,
    named_captures: HashMap<String, usize>,
    char_classes: Vec<CharClass>,
    mode: MatchMode,
    program: Vec<Instruction>,
    anchored_start: bool,
    prefixes: Vec<Vec<u8>>,
}

impl CompiledRegex {
    pub fn get_id(&self) -> usize {
        self.id
    }
}

struct Compiler<'a> {
    compiled: &'a mut CompiledRegex,
    classes: HashMap<CharClass, usize>,
    max_repeat: usize,
    max_instructions: usize,
}

impl<'a> Compiler<'a> {
    fn new(compiled: &'a mut CompiledRegex, max_repeat: usize, max_instructions: usize) -> Self {
        Self {
            compiled,
            max_repeat,
            max_instructions,
            classes: HashMap::new(),
        }
    }

    fn build(&mut self, hir: &HirKind) -> Result<(), Error> {
        self.compile(hir)?;
        self.add_instruction(Instruction::Match)?;
        Ok(())
    }

    fn add_instruction(&mut self, instruction: Instruction) -> Result<(), Error> {
        if self.compiled.program.len() >= self.max_instructions {
            Err(Error::ProgramTooLarge)
        } else {
            self.compiled.program.push(instruction);
            Ok(())
        }
    }

    fn add_byte_class(&mut self, byte_class: CharClass) -> usize {
        match self.classes.get(&byte_class) {
            Some(v) => *v,
            None => {
                let v = self.classes.len();
                self.classes.insert(byte_class.clone(), v);
                self.compiled.char_classes.push(byte_class);
                v
            }
        }
    }

    fn build_group(&mut self, group: &Group) -> Result<(), Error> {
        match group.kind {
            GroupKind::NonCapturing => self.compile(group.hir.kind())?,
            GroupKind::CaptureIndex(n) => {
                let n = n as usize;
                self.add_instruction(Instruction::Start(n))?;
                self.compile(group.hir.kind())?;
                self.add_instruction(Instruction::End(n))?;
                self.compiled.capture_count = max(self.compiled.capture_count, n + 1);
            }
            GroupKind::CaptureName { ref name, index } => {
                let index = index as usize;
                self.add_instruction(Instruction::Start(index))?;
                self.compile(group.hir.kind())?;
                self.add_instruction(Instruction::End(index))?;
                self.compiled.capture_count = max(self.compiled.capture_count, index + 1);
                self.compiled.named_captures.insert(name.clone(), index);
            }
        }
        Ok(())
    }

    fn build_repetition_piece(
        &mut self,
        body: &HirKind,
        minimum: usize,
        maximum: usize,
        kleene: bool,
        greedy: bool,
    ) -> Result<(), Error> {
        if minimum > self.max_repeat || maximum > self.max_repeat {
            return Err(Error::TooManyRepetitions);
        }

        for _ in 0..minimum {
            self.compile(body)?;
        }

        for _ in 0..(maximum - minimum) {
            let offset = self.compiled.program.len();
            self.add_instruction(Instruction::NoOp)?;
            self.compile(body)?;
            if greedy {
                self.compiled.program[offset] =
                    Instruction::Split(offset + 1, self.compiled.program.len());
            } else {
                self.compiled.program[offset] =
                    Instruction::Split(self.compiled.program.len(), offset + 1);
            }
        }

        if kleene {
            let offset = self.compiled.program.len();
            self.add_instruction(Instruction::NoOp)?;
            self.compile(body)?;
            self.add_instruction(Instruction::Jump(offset))?;
            if greedy {
                self.compiled.program[offset] =
                    Instruction::Split(offset + 1, self.compiled.program.len());
            } else {
                self.compiled.program[offset] =
                    Instruction::Split(self.compiled.program.len(), offset + 1);
            }
        }

        Ok(())
    }

    fn build_repetition(&mut self, repetition: &Repetition) -> Result<(), Error> {
        let body = repetition.hir.kind();
        let greedy = repetition.greedy;
        match repetition.kind {
            RepetitionKind::ZeroOrOne => self.build_repetition_piece(body, 0, 1, false, greedy)?,
            RepetitionKind::ZeroOrMore => self.build_repetition_piece(body, 0, 0, true, greedy)?,
            RepetitionKind::OneOrMore => self.build_repetition_piece(body, 1, 1, true, greedy)?,
            RepetitionKind::Range(RepetitionRange::Exactly(n)) => {
                self.build_repetition_piece(body, n as usize, n as usize, false, greedy)?
            }
            RepetitionKind::Range(RepetitionRange::AtLeast(n)) => {
                self.build_repetition_piece(body, n as usize, n as usize, true, greedy)?
            }
            RepetitionKind::Range(RepetitionRange::Bounded(lo, hi)) => {
                self.build_repetition_piece(body, lo as usize, hi as usize, false, greedy)?
            }
        }
        Ok(())
    }

    fn compile(&mut self, hir: &HirKind) -> Result<(), Error> {
        match hir {
            HirKind::Empty => {
                self.add_instruction(Instruction::Match)?;
            }
            HirKind::Literal(Literal::Byte(b)) => {
                self.add_instruction(Instruction::Byte(*b))?;
            }
            HirKind::Literal(Literal::Unicode(u)) => {
                for b in u.encode_utf8(&mut [0; 4]).bytes() {
                    self.add_instruction(Instruction::Byte(b))?;
                }
            }
            HirKind::Class(Class::Unicode(_)) => {
                return Err(Error::InvalidExpression(
                    "Unicode character classes are not supported; use ByteTransformer".to_string(),
                ));
            }
            HirKind::Class(Class::Bytes(b)) => {
                let char_class = CharClass::new_from_bytes(b);
                let index = self.add_byte_class(char_class);
                self.add_instruction(Instruction::CharClass(index))?;
            }
            HirKind::Anchor(Anchor::StartLine) => {
                self.add_instruction(Instruction::Assert(Assertion::StartOfLine))?;
            }
            HirKind::Anchor(Anchor::EndLine) => {
                self.add_instruction(Instruction::Assert(Assertion::EndOfLine))?;
            }
            HirKind::Anchor(Anchor::StartText) => {
                self.add_instruction(Instruction::Assert(Assertion::StartOfText))?;
            }
            HirKind::Anchor(Anchor::EndText) => {
                self.add_instruction(Instruction::Assert(Assertion::EndOfText))?;
            }
            HirKind::WordBoundary(WordBoundary::Unicode) => {
                return Err(Error::UnicodeWordBoundaries);
            }
            HirKind::WordBoundary(WordBoundary::UnicodeNegate) => {
                return Err(Error::UnicodeWordBoundaries);
            }
            HirKind::WordBoundary(WordBoundary::Ascii) => {
                self.add_instruction(Instruction::Assert(Assertion::ByteWordBoundary))?;
            }
            HirKind::WordBoundary(WordBoundary::AsciiNegate) => {
                self.add_instruction(Instruction::Assert(Assertion::ByteNonwordBoundary))?;
            }
            HirKind::Repetition(repetition) => self.build_repetition(repetition)?,
            HirKind::Group(group) => self.build_group(group)?,
            HirKind::Concat(members) => {
                for member in members {
                    self.compile(member.kind())?;
                }
            }
            HirKind::Alternation(branches) => {
                let mut fixups: Vec<usize> = Vec::new();

                for (i, branch) in branches.iter().enumerate() {
                    let is_last = i == branches.len() - 1;
                    let start = self.compiled.program.len();
                    if !is_last {
                        self.add_instruction(Instruction::NoOp)?;
                    }
                    self.compile(branch.kind())?;
                    if !is_last {
                        self.compiled.program[start] =
                            Instruction::Split(start + 1, self.compiled.program.len() + 1);
                        fixups.push(self.compiled.program.len());
                        self.add_instruction(Instruction::NoOp)?;
                    }
                }
                for fixup in fixups {
                    self.compiled.program[fixup] = Instruction::Jump(self.compiled.program.len());
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Assertion {
    StartOfText,
    StartOfLine,
    EndOfText,
    EndOfLine,
    ByteWordBoundary,
    ByteNonwordBoundary,
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Assert(Assertion),
    Byte(u8),
    CharClass(usize),
    End(usize),
    Jump(usize),
    Match,
    Split(usize, usize),
    Start(usize),
    NoOp,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct CharClass {
    bytes: [u64; 4],
}

impl CharClass {
    fn new() -> Self {
        Self { bytes: [0; 4] }
    }

    fn new_from_bytes(bytes: &ClassBytes) -> Self {
        let mut byte_class = Self::new();
        for range in bytes.iter() {
            for i in range.start()..=range.end() {
                byte_class.add(i);
            }
        }
        byte_class
    }

    fn add(&mut self, byte: u8) {
        let c = byte as usize;
        self.bytes[c / 64] |= 1 << (c % 64);
    }

    fn contains(&self, byte: u8) -> bool {
        let c = byte as usize;
        self.bytes[c / 64] & 1 << (c % 64) != 0
    }
}

pub struct Database {
    expressions: Vec<CompiledRegex>,
    ids: HashMap<usize, Vec<usize>>,
    ac: AhoCorasick,
    scratch_map: HashMap<usize, Vec<usize>>,
}

impl<'a> Database {
    pub fn make_scratch(&'a self, handler: &'a mut dyn MatchHandler) -> Scratch {
        Scratch {
            database: self,
            enabled: ShrinkSet::new(self.expressions.len()),
            disabled: GrowSet::with_capacity(self.expressions.len()),
            scratch: self.expressions.iter().map(RegexScratch::new).collect(),
            next_check: handler.get_pulse_interval(),
            seen: 0,
            handler,
            last_byte: None,
            ac_scratch: self.ac.get_scratch(),
        }
    }
}

pub struct DatabaseBuilder {
    expressions: Vec<CompiledRegex>,
    ids: HashMap<usize, Vec<usize>>,
}

impl<'a> DatabaseBuilder {
    pub fn new() -> Self {
        Self {
            expressions: Vec::new(),
            ids: HashMap::new(),
        }
    }

    pub fn with_expression(mut self, expression: CompiledRegex) -> Self {
        self.ids.entry(expression.get_id()).or_insert_with(Vec::new);

        let referenced_expressions = self.ids.get_mut(&expression.get_id()).unwrap();
        referenced_expressions.push(self.expressions.len());
        self.expressions.push(expression);
        self
    }

    pub fn with_expressions(mut self, expressions: Vec<CompiledRegex>) -> Self {
        for expression in expressions {
            self = self.with_expression(expression);
        }
        self
    }

    pub fn build(self) -> Database {
        let (scratch_map, ac) = self.build_aho_corasick(6);
        Database {
            expressions: self.expressions,
            ids: self.ids,
            ac,
            scratch_map,
        }
    }

    fn build_aho_corasick(&self, length: usize) -> (HashMap<usize, Vec<usize>>, AhoCorasick) {
        let mut literal_map: HashMap<Vec<u8>, usize> = HashMap::new();
        let mut literals: Vec<Vec<u8>> = vec![];
        let mut scratch_map: HashMap<usize, Vec<usize>> = HashMap::new();

        for (index, expression) in self.expressions.iter().enumerate() {
            for literal in expression.prefixes.iter() {
                let literal = &literal[..min(length, literal.len())];
                if !literal_map.contains_key(literal) {
                    literal_map.insert(literal.to_vec(), literals.len());
                    literals.push(literal.to_vec());
                }

                if !scratch_map.contains_key(literal_map.get(literal).unwrap()) {
                    scratch_map.insert(*literal_map.get(literal).unwrap(), vec![]);
                }
                scratch_map
                    .get_mut(literal_map.get(literal).unwrap())
                    .unwrap()
                    .push(index);
            }
        }

        (scratch_map, AhoCorasick::new(&literals, 2))
    }
}

impl<'a> Default for DatabaseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Eq, PartialEq)]
enum Disabled {
    Yes(ContinueMatching),
    No(ContinueMatching),
}

pub struct Scratch<'a> {
    // The database we represent.
    database: &'a Database,

    // Various sets keeping track of active expressions.
    // enabled - expressions that are enabled, and thus able to run
    enabled: ShrinkSet,

    // disabled - expressions that are disabled; they will be removed
    // from the enabled set at the end of this cycle. We use this temporary
    // storage to ensure that we don't modify the enabled set as we
    // iterate over it.
    disabled: GrowSet,

    scratch: Vec<RegexScratch<'a>>,
    seen: usize,
    next_check: usize,
    handler: &'a mut dyn MatchHandler,
    last_byte: Option<u8>,
    ac_scratch: AhoCorasickScratch<'a>,
}

impl<'a> Scratch<'a> {
    pub fn reset(mut self) -> Self {
        self.enabled.refill();
        self.disabled.clear();
        self.seen = 0;
        self.handler.on_reset();
        self.next_check = self.handler.get_pulse_interval();
        self.last_byte = None;
        self.ac_scratch.reset();
        for scratch in self.scratch.iter_mut() {
            scratch.reset();
        }
        self
    }

    pub fn finish(mut self) -> Self {
        self.push_finish();
        self.reset()
    }

    pub fn disable(&mut self, id: usize) {
        if let Some(ids) = self.database.ids.get(&id) {
            for &id in ids.iter() {
                self.enabled.remove(id);
            }
        }
    }

    pub fn push_finish(&mut self) {
        for &index in self.enabled.iter() {
            self.scratch[index].handle_finish(self.seen, self.last_byte, self.handler);
        }
    }

    // FIXME - this whole thing needs to be cleaned up a lot. Holy hell.
    pub fn push(&mut self, bytes: &'a [u8]) -> ContinueMatching {
        // Nothing to actually do.
        if bytes.is_empty() || self.enabled.is_empty() {
            return ContinueMatching::Yes;
        }

        // Deal with initial setup and picking up where we left off from previous
        // blocks and executing expressions that can run anywhere.
        for &index in self.enabled.iter() {
            let scratch = &mut self.scratch[index];

            // If there is any leftover execution from the previous block, or if these
            // expressions can run anywhere, run them now.
            if !scratch.current_set.is_empty()
                || scratch.regex.anchored_start
                || scratch.regex.prefixes.is_empty()
            {
                match scratch.handle_bytes(self.seen, self.last_byte, bytes, self.handler, false) {
                    Disabled::Yes(ContinueMatching::Yes) => self.disabled.add(index),
                    Disabled::Yes(ContinueMatching::No) | Disabled::No(ContinueMatching::No) => {
                        return ContinueMatching::No
                    }
                    Disabled::No(ContinueMatching::Yes) => {}
                }
            }
        }

        // Walk through all of the possible starts for the regexes with prefixes.
        for m in self.ac_scratch.push(bytes) {
            let keyword = &self.database.ac.keywords()[m.pattern()];

            for &index in self.database.scratch_map.get(&m.pattern()).unwrap() {
                let mut new_offset = m.offset();
                let scratch = &mut self.scratch[index];
                let mut last_byte = None;

                // skip...
                if !self.enabled.contains(index) // disabled expressions
                    || self.disabled.contains(index) // disabled expressions
                    || new_offset < scratch.offset // input we've already examined
                    || !scratch.current_set.is_empty()
                // already started
                {
                    continue;
                }

                // If the keyword starts before the block, push the portion of the keyword that happened
                // before the current block.
                scratch.handle_bytes(
                    new_offset,
                    None, // NOTE - This works iff ZWA break prefixes.
                    keyword,
                    self.handler,
                    true,
                );
                new_offset = scratch.offset;

                // Push the block we've got here, if there's any left.
                if new_offset < self.seen + bytes.len() && !self.disabled.contains(index) {
                    let start = scratch.offset - self.seen;
                    if start > 0 {
                        last_byte = Some(bytes[start - 1]);
                    }

                    scratch.handle_bytes(
                        new_offset,
                        last_byte,
                        &bytes[start..],
                        self.handler,
                        true,
                    );
                }
            }
        }

        // And update the scratch state.
        self.last_byte = Some(bytes[bytes.len() - 1]);
        self.seen += bytes.len();

        // Disable any newly-disabled expressions.
        for &index in self.disabled.iter() {
            self.enabled.remove(index);
        }
        self.disabled.clear();

        // And we're done.
        ContinueMatching::Yes
    }
}

fn is_alphanumeric(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn check_assertion(assertion: Assertion, last_byte: Option<u8>, byte: Option<u8>) -> bool {
    match (assertion, last_byte, byte) {
        (Assertion::StartOfText, None, _) => true,
        (Assertion::StartOfLine, None, _) => true,
        (Assertion::StartOfLine, Some(CR), _) => true,
        (Assertion::StartOfLine, Some(NL), _) => true,
        (Assertion::EndOfText, _, None) => true,
        (Assertion::EndOfLine, _, None) => true,
        (Assertion::EndOfLine, _, Some(CR)) => true,
        (Assertion::EndOfLine, _, Some(NL)) => true,
        (Assertion::ByteWordBoundary, None, Some(b)) => is_alphanumeric(b),
        (Assertion::ByteWordBoundary, Some(b), None) => is_alphanumeric(b),
        (Assertion::ByteWordBoundary, Some(a), Some(b)) => {
            (is_alphanumeric(a) && !is_alphanumeric(b))
                || (!is_alphanumeric(a) && is_alphanumeric(b))
        }
        (Assertion::ByteNonwordBoundary, None, None) => true,
        (Assertion::ByteNonwordBoundary, None, Some(b)) => !is_alphanumeric(b),
        (Assertion::ByteNonwordBoundary, Some(b), None) => !is_alphanumeric(b),
        (Assertion::ByteNonwordBoundary, Some(a), Some(b)) => {
            is_alphanumeric(a) && is_alphanumeric(b)
        }
        (_, _, _) => false,
    }
}

// FIXME - shrink this down as much as we can
pub struct RegexScratch<'a> {
    regex: &'a CompiledRegex,
    current_set: Queue,
    ready_set: Queue,
    offset: usize,
}

impl<'a> RegexScratch<'a> {
    pub fn new(regex: &'a CompiledRegex) -> Self {
        Self {
            regex,
            current_set: Queue::new(regex.program.len(), regex.capture_count),
            ready_set: Queue::new(regex.program.len(), regex.capture_count),
            offset: 0,
        }
    }

    fn reset(&mut self) {
        self.current_set.clear();
        self.ready_set.clear();
    }

    fn handle_finish(
        &mut self,
        offset: usize,
        last_byte: Option<u8>,
        handler: &mut dyn MatchHandler,
    ) {
        let id = self.regex.id;

        // Figure out if we need to exit early, or push a new starting thread.
        if self.current_set.is_empty() || !self.regex.anchored_start {
            self.current_set.push_empty(offset);
        }

        // Execute for as long as there are pending threads.
        while !self.current_set.is_empty() {
            let thread_id = self.current_set.pop();
            let mut pc = thread_id;
            loop {
                match self.regex.program[pc] {
                    Instruction::Assert(a) => {
                        if check_assertion(a, last_byte, None) {
                            pc += 1;
                            continue;
                        }
                    }
                    Instruction::Byte(_) | Instruction::CharClass(_) => {}
                    Instruction::End(index) => {
                        self.current_set.captures[thread_id][index].end = Some(offset);
                        pc += 1;
                        continue;
                    }
                    Instruction::Jump(target) => {
                        pc = target;
                        continue;
                    }
                    Instruction::Match => {
                        self.current_set.captures[thread_id][0].end = Some(offset);
                        handler.on_match(id, &self.current_set.captures[thread_id]);
                    }
                    Instruction::NoOp => {
                        pc += 1;
                        continue;
                    }
                    Instruction::Split(pc_a, pc_b) => {
                        // this is magic, but a bit smelly
                        self.current_set
                            .push_from_current(End::Front, pc_b, thread_id);
                        pc = pc_a;
                        continue;
                    }
                    Instruction::Start(index) => {
                        self.current_set.captures[thread_id][index].start = Some(offset);
                        pc += 1;
                        continue;
                    }
                }
                // different instructions above continue the thread, but if we made it
                // here, we're done with this thread.
                break;
            }
        }
    }

    fn handle_bytes(
        &mut self,
        mut offset: usize,
        mut last_byte: Option<u8>,
        bytes: &[u8],
        handler: &mut dyn MatchHandler,
        mut at_start: bool,
    ) -> Disabled {
        let id = self.regex.id;
        let mode = self.regex.mode;
        let start_anywhere = self.regex.prefixes.is_empty();
        let mut at = 0usize;

        let len = bytes.len();
        'LOOP: while at < len {
            // If we don't have any threads, see if we can be finished.
            if self.current_set.is_empty() {
                if self.regex.anchored_start && offset > 0 {
                    return Disabled::Yes(ContinueMatching::Yes);
                }

                if at_start || start_anywhere {
                    at_start = false;
                    self.current_set.push_empty(offset);
                    continue 'LOOP;
                }

                break 'LOOP;
            }

            // If we're already running and we're not an anchored start, we can
            // start anywhere, so spawn a new thread. Note that this is distinct
            // from the at_start test above: if we have no threads at all, we'll
            // start running at a prefix, but if we're already running, we have to
            // check all of the "internal" start positions as well.
            if !self.regex.anchored_start {
                self.current_set.push_empty(offset);
            }

            // Execute for as long as there are pending threads.
            let byte = bytes[at];
            while !self.current_set.is_empty() {
                let thread_id = self.current_set.pop();
                let mut pc = thread_id;
                loop {
                    match self.regex.program[pc] {
                        Instruction::Assert(a) => {
                            if check_assertion(a, last_byte, Some(byte)) {
                                pc += 1;
                                continue;
                            }
                        }
                        Instruction::Byte(b) => {
                            if Some(b) == Some(byte) {
                                self.ready_set.push(
                                    End::Back,
                                    pc + 1,
                                    &self.current_set.captures[thread_id],
                                );
                            }
                        }
                        Instruction::End(index) => {
                            self.current_set.captures[thread_id][index].end = Some(offset);
                            pc += 1;
                            continue;
                        }
                        Instruction::CharClass(cc) => {
                            if self.regex.char_classes[cc].contains(byte) {
                                self.ready_set.push(
                                    End::Back,
                                    pc + 1,
                                    &self.current_set.captures[thread_id],
                                );
                            }
                        }
                        Instruction::Jump(target) => {
                            pc = target;
                            continue;
                        }
                        Instruction::Match => {
                            self.current_set.captures[thread_id][0].end = Some(offset);
                            match mode {
                                MatchMode::First(_) => {
                                    return Disabled::Yes(
                                        handler.on_match(id, &self.current_set.captures[thread_id]),
                                    );
                                }
                                MatchMode::All(_) => {
                                    if handler.on_match(id, &self.current_set.captures[thread_id])
                                        == ContinueMatching::No
                                    {
                                        return Disabled::No(ContinueMatching::No);
                                    }
                                }
                            }
                        }
                        Instruction::NoOp => {
                            pc += 1;
                            continue;
                        }
                        Instruction::Split(pc_a, pc_b) => {
                            self.current_set
                                .push_from_current(End::Front, pc_b, thread_id); // this is magic, if a bit smelly
                            pc = pc_a;
                            continue;
                        }
                        Instruction::Start(index) => {
                            self.current_set.captures[thread_id][index].start = Some(offset);
                            pc += 1;
                            continue;
                        }
                    }
                    // different instructions above continue the thread, but if we made it
                    // here, we're done with this thread.
                    break;
                }
            }
            last_byte = Some(byte);
            offset += 1;
            at += 1;
            self.offset = offset;
            swap(&mut self.current_set, &mut self.ready_set);
            self.ready_set.clear();
        }
        Disabled::No(ContinueMatching::Yes)
    }
}
