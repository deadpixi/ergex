// FIXME - this whole thing needs to be rewritten
//         we should use an iterator, and the
//         cookie thing is just terrible

use std::collections::VecDeque;

#[derive(Debug)]
enum TransitionSet {
    Dense(Vec<Option<usize>>),
    Sparse(Vec<(u8, usize)>),
}

impl TransitionSet {
    fn new_dense() -> Self {
        Self::Dense(vec![None; 256])
    }

    fn new_sparse() -> Self {
        Self::Sparse(vec![])
    }

    fn next_state(&self, byte: u8) -> Option<usize> {
        match self {
            Self::Dense(ref transitions) => transitions[byte as usize],
            Self::Sparse(ref transitions) => {
                for &(value, state) in transitions.iter() {
                    if value == byte {
                        return Some(state);
                    }
                }
                None
            }
        }
    }

    fn set_next_state(&mut self, byte: u8, next_state: usize) {
        match self {
            Self::Dense(ref mut transitions) => transitions[byte as usize] = Some(next_state),
            Self::Sparse(ref mut transitions) => transitions.push((byte, next_state)),
        }
    }

    fn next_states(&self) -> Vec<usize> {
        match self {
            Self::Dense(ref transitions) => transitions
                .iter()
                .filter(|&x| x.is_some())
                .map(|&x| x.unwrap())
                .collect(),
            Self::Sparse(ref transitions) => transitions.iter().map(|&x| x.1).collect(),
        }
    }
}

struct Node {
    value: Option<u8>,
    next_states: TransitionSet,
    fail_state: usize,
    output: Vec<usize>,
}

impl Node {
    fn empty() -> Self {
        Self {
            value: None,
            next_states: TransitionSet::new_dense(),
            fail_state: 0,
            output: vec![],
        }
    }

    fn new(value: u8, next_states: TransitionSet) -> Self {
        Self {
            value: Some(value),
            fail_state: 0,
            output: vec![],
            next_states,
        }
    }
}

#[derive(Clone, Copy)]
pub struct AhoCorasickMatch {
    offset: usize,
    pattern: usize,
}

impl AhoCorasickMatch {
    fn new(offset: usize, pattern: usize) -> Self {
        Self { offset, pattern }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn pattern(&self) -> usize {
        self.pattern
    }
}

pub struct AhoCorasick {
    trie: Vec<Node>,
    keywords: Vec<Vec<u8>>,
}

impl AhoCorasick {
    pub fn new<I, P>(keywords: I, dense_prefix: usize) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<[u8]>,
    {
        let mut ac = Self {
            trie: vec![Node::empty()],
            keywords: vec![],
        };

        for keyword in keywords.into_iter() {
            ac.add_keyword(keyword, dense_prefix);
        }
        ac.build_failure_function();

        for node in ac.trie.iter_mut() {
            node.output.shrink_to_fit();
        }
        ac.trie.shrink_to_fit();

        ac
    }

    pub fn keywords(&self) -> &Vec<Vec<u8>> {
        &self.keywords
    }

    fn find_next_state(&self, current_state: usize, value: u8) -> Option<usize> {
        self.trie[current_state].next_states.next_state(value)
    }

    fn add_keyword<P>(&mut self, keyword: P, dense_prefix: usize)
    where
        P: AsRef<[u8]>,
    {
        let keyword = keyword.as_ref();
        let mut current_state = 0;
        let mut j = 0;
        let mut child = self.find_next_state(current_state, keyword[j]);

        while let Some(c) = child {
            current_state = c;

            j += 1;
            if j < keyword.len() {
                child = self.find_next_state(current_state, keyword[j]);
            } else {
                break;
            }
        }

        for &c in &keyword[j..] {
            let next_states = if self.trie.len() <= dense_prefix {
                TransitionSet::new_dense()
            } else {
                TransitionSet::new_sparse()
            };

            self.trie.push(Node::new(c, next_states));
            let l = self.trie.len() - 1;
            self.trie[current_state].next_states.set_next_state(c, l);
            current_state = self.trie.len() - 1;
        }

        self.trie[current_state].output.push(self.keywords.len());

        let mut keyword = keyword.to_vec();
        keyword.shrink_to_fit();
        self.keywords.push(keyword);
    }

    fn build_failure_function(&mut self) {
        let mut queue = VecDeque::new();

        for &node in self.trie[0].next_states.next_states().iter() {
            queue.push_back(node);
            self.trie[node].fail_state = 0;
        }

        while !queue.is_empty() {
            let r = queue.pop_front().unwrap();
            for &child in self.trie[r].next_states.next_states().iter() {
                queue.push_back(child);

                let mut state = self.trie[r].fail_state;
                while self
                    .find_next_state(state, self.trie[child].value.unwrap())
                    .is_none()
                    && state != 0
                {
                    state = self.trie[state].fail_state;
                }
                self.trie[child].fail_state = self
                    .find_next_state(state, self.trie[child].value.unwrap())
                    .unwrap_or(0);

                let fail_state = self.trie[child].fail_state;
                let mut states = self.trie[fail_state].output.clone();
                self.trie[child].output.append(&mut states);
            }
        }
    }

    pub fn get_scratch(&self) -> AhoCorasickScratch {
        AhoCorasickScratch {
            ac: self,
            current_state: 0,
            seen: 0,
        }
    }
}

pub struct AhoCorasickScratch<'a> {
    ac: &'a AhoCorasick,
    current_state: usize,
    seen: usize,
}

impl<'a> AhoCorasickScratch<'a> {
    pub fn reset(&mut self) {
        self.current_state = 0;
        self.seen = 0;
    }

    pub fn push(&mut self, text: &'a [u8]) -> AhoCorasickIterator {
        AhoCorasickIterator::new(self.ac, &mut self.current_state, &mut self.seen, text)
    }
}

pub struct AhoCorasickIterator<'a> {
    ac: &'a AhoCorasick,
    offset: usize,
    seen: &'a mut usize,
    current_state: &'a mut usize,
    text: &'a [u8],
    current_output: Option<usize>,
}

impl<'a> AhoCorasickIterator<'a> {
    fn new(
        ac: &'a AhoCorasick,
        current_state: &'a mut usize,
        seen: &'a mut usize,
        text: &'a [u8],
    ) -> Self {
        Self {
            offset: 0,
            current_output: None,
            ac,
            seen,
            current_state,
            text,
        }
    }
}

impl<'a> Iterator for AhoCorasickIterator<'a> {
    type Item = AhoCorasickMatch;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_output) = self.current_output {
            let output = &self.ac.trie[*self.current_state].output;
            if current_output < output.len() {
                let keyword = &self.ac.keywords[output[current_output]];
                let len = keyword.len();
                self.current_output = Some(current_output + 1);
                return Some(AhoCorasickMatch::new(
                    self.offset - (len - 1),
                    output[current_output],
                ));
            }
            self.current_output = None;
        }

        for &byte in self.text[self.offset..].iter() {
            self.offset += 1;

            while *self.current_state != 0
                && self.ac.find_next_state(*self.current_state, byte).is_none()
            {
                *self.current_state = self.ac.trie[*self.current_state].fail_state;
            }

            *self.current_state = self
                .ac
                .find_next_state(*self.current_state, byte)
                .unwrap_or(0);
            if !self.ac.trie[*self.current_state].output.is_empty() {
                let output = self.ac.trie[*self.current_state].output[0];
                let keyword = &self.ac.keywords[output];
                let len = keyword.len();
                self.current_output = Some(1);
                return Some(AhoCorasickMatch::new(
                    *self.seen + self.offset - len,
                    output,
                ));
            }
        }

        *self.seen += self.text.len();
        None
    }
}
