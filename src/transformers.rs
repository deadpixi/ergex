use regex_syntax::hir::{
    Anchor, Class, ClassBytes, ClassBytesRange, ClassUnicode, ClassUnicodeRange, Group, GroupKind,
    Hir, HirKind, Literal, Repetition, WordBoundary,
};
use regex_syntax::utf8::{Utf8Sequence, Utf8Sequences};

pub trait Transformer {
    fn empty(&self) -> Hir {
        Hir::empty()
    }

    fn literal(&self, node: Literal) -> Hir {
        Hir::literal(node)
    }

    fn class(&self, node: Class) -> Hir {
        Hir::class(node)
    }

    fn anchor(&self, node: Anchor) -> Hir {
        Hir::anchor(node)
    }

    fn word_boundary(&self, node: WordBoundary) -> Hir {
        Hir::word_boundary(node)
    }

    fn repetition(&self, node: Repetition) -> Hir {
        Hir::repetition(Repetition {
            kind: node.kind,
            greedy: node.greedy,
            hir: Box::new(self.transform(*node.hir)),
        })
    }

    fn group(&self, node: Group) -> Hir {
        Hir::group(Group {
            kind: node.kind,
            hir: Box::new(self.transform(*node.hir)),
        })
    }

    fn concat(&self, node: Vec<Hir>) -> Hir {
        Hir::concat(node.into_iter().map(|x| self.transform(x)).collect())
    }

    fn alternation(&self, node: Vec<Hir>) -> Hir {
        Hir::alternation(node.into_iter().map(|x| self.transform(x)).collect())
    }

    fn transform(&self, node: Hir) -> Hir {
        match node.into_kind() {
            HirKind::Empty => self.empty(),
            HirKind::Literal(l) => self.literal(l),
            HirKind::Class(c) => self.class(c),
            HirKind::Anchor(a) => self.anchor(a),
            HirKind::WordBoundary(w) => self.word_boundary(w),
            HirKind::Repetition(r) => self.repetition(r),
            HirKind::Group(g) => self.group(g),
            HirKind::Concat(c) => self.concat(c),
            HirKind::Alternation(a) => self.alternation(a),
        }
    }
}

pub struct GroupEliminator {
    max_capture: u32,
}

impl GroupEliminator {
    pub fn with_max_capture(max_capture: u32) -> Self {
        Self { max_capture }
    }
}

impl Transformer for GroupEliminator {
    fn group(&self, node: Group) -> Hir {
        match node.kind {
            GroupKind::NonCapturing => self.transform(*node.hir),
            GroupKind::CaptureIndex(n) if n > self.max_capture => self.transform(*node.hir),
            GroupKind::CaptureName {
                name: ref _x,
                index: n,
            } if n > self.max_capture => self.transform(*node.hir),
            _ => Hir::group(Group {
                hir: Box::new(self.transform(*node.hir)),
                kind: node.kind,
            }),
        }
    }
}

pub struct UnicodeAlternationSimplifier {}

impl UnicodeAlternationSimplifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transformer for UnicodeAlternationSimplifier {
    fn alternation(&self, node: Vec<Hir>) -> Hir {
        let transformed: Vec<Hir> = node.into_iter().map(|x| self.transform(x)).collect();
        let mut char_class = ClassUnicode::empty();
        let mut range_count = 0;
        let mut branches: Vec<Hir> = Vec::new();

        for x in transformed {
            match *x.kind() {
                HirKind::Literal(Literal::Unicode(u)) => {
                    char_class.push(ClassUnicodeRange::new(u, u));
                    range_count += 1;
                }
                HirKind::Class(Class::Unicode(ref r)) => {
                    for range in r.ranges() {
                        char_class.push(*range);
                        range_count += 1;
                    }
                }
                _ => branches.push(x),
            }
        }

        if range_count > 0 {
            branches.push(Hir::class(Class::Unicode(char_class)));
        }

        if branches.len() == 1 {
            branches.remove(0) // remove the only branch and return it
        } else {
            Hir::alternation(branches)
        }
    }
}

pub struct ByteAlternationSimplifier {}

impl ByteAlternationSimplifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transformer for ByteAlternationSimplifier {
    fn alternation(&self, node: Vec<Hir>) -> Hir {
        let transformed: Vec<Hir> = node.into_iter().map(|x| self.transform(x)).collect();
        let mut char_class = ClassBytes::empty();
        let mut branches: Vec<Hir> = Vec::new();

        for x in transformed {
            match *x.kind() {
                HirKind::Literal(Literal::Unicode(u)) if (u as usize) <= 0x7f => {
                    char_class.push(ClassBytesRange::new(u as u8, u as u8))
                }
                HirKind::Literal(Literal::Byte(b)) => char_class.push(ClassBytesRange::new(b, b)),
                HirKind::Class(Class::Bytes(ref r)) => {
                    for range in r.ranges() {
                        char_class.push(*range);
                    }
                }
                _ => branches.push(x),
            }
        }

        if char_class != ClassBytes::empty() {
            branches.push(Hir::class(Class::Bytes(char_class)));
        }

        if branches.len() == 1 {
            branches.remove(0)
        } else {
            Hir::alternation(branches)
        }
    }
}

pub struct ClassSimplifier {}

impl ClassSimplifier {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transformer for ClassSimplifier {
    fn class(&self, node: Class) -> Hir {
        match node {
            Class::Unicode(u) => {
                let r: Vec<&ClassUnicodeRange> = u.ranges().iter().collect();
                if r.len() == 1 && r[0].start() == r[0].end() {
                    Hir::literal(Literal::Unicode(r[0].start()))
                } else {
                    Hir::class(Class::Unicode(u))
                }
            }

            Class::Bytes(b) => {
                let r: Vec<&ClassBytesRange> = b.ranges().iter().collect();
                if r.len() == 1 && r[0].start() == r[0].end() {
                    if r[0].start() > 0x7f {
                        Hir::literal(Literal::Byte(r[0].start()))
                    } else {
                        Hir::literal(Literal::Unicode(r[0].start() as char))
                    }
                } else {
                    Hir::class(Class::Bytes(b))
                }
            }
        }
    }
}

pub struct ByteTransformer {}

impl ByteTransformer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transformer for ByteTransformer {
    fn literal(&self, node: Literal) -> Hir {
        match node {
            Literal::Unicode(c) => {
                // The Literal::Byte variant is used only for non-ASCII (!) bytes...
                let mut r: Vec<Hir> = Vec::with_capacity(4);
                for b in c.encode_utf8(&mut [0; 4]).bytes() {
                    if b > 0x7f {
                        r.push(Hir::literal(Literal::Byte(b)));
                    } else {
                        r.push(Hir::literal(Literal::Unicode(b as char)));
                    }
                }
                Hir::concat(r)
            }
            Literal::Byte(b) => Hir::literal(Literal::Byte(b)),
        }
    }

    fn class(&self, node: Class) -> Hir {
        match node {
            Class::Bytes(b) => Hir::class(Class::Bytes(b)),
            Class::Unicode(class) => {
                let mut alternations: Vec<Hir> = Vec::new();

                for range in class.iter() {
                    for sequence in Utf8Sequences::new(range.start(), range.end()) {
                        match sequence {
                            Utf8Sequence::One(r) => {
                                alternations.push(Hir::class(Class::Bytes(ClassBytes::new(vec![
                                    ClassBytesRange::new(r.start, r.end),
                                ]))));
                            }
                            Utf8Sequence::Two([a, b]) => {
                                let class_a = Hir::class(Class::Bytes(ClassBytes::new(vec![
                                    ClassBytesRange::new(a.start, a.end),
                                ])));
                                let class_b = Hir::class(Class::Bytes(ClassBytes::new(vec![
                                    ClassBytesRange::new(b.start, b.end),
                                ])));
                                alternations.push(Hir::concat(vec![class_a, class_b]));
                            }
                            Utf8Sequence::Three([a, b, c]) => {
                                let class_a = Hir::class(Class::Bytes(ClassBytes::new(vec![
                                    ClassBytesRange::new(a.start, a.end),
                                ])));
                                let class_b = Hir::class(Class::Bytes(ClassBytes::new(vec![
                                    ClassBytesRange::new(b.start, b.end),
                                ])));
                                let class_c = Hir::class(Class::Bytes(ClassBytes::new(vec![
                                    ClassBytesRange::new(c.start, c.end),
                                ])));
                                alternations.push(Hir::concat(vec![class_a, class_b, class_c]));
                            }
                            Utf8Sequence::Four([a, b, c, d]) => {
                                let class_a = Hir::class(Class::Bytes(ClassBytes::new(vec![
                                    ClassBytesRange::new(a.start, a.end),
                                ])));
                                let class_b = Hir::class(Class::Bytes(ClassBytes::new(vec![
                                    ClassBytesRange::new(b.start, b.end),
                                ])));
                                let class_c = Hir::class(Class::Bytes(ClassBytes::new(vec![
                                    ClassBytesRange::new(c.start, c.end),
                                ])));
                                let class_d = Hir::class(Class::Bytes(ClassBytes::new(vec![
                                    ClassBytesRange::new(d.start, d.end),
                                ])));
                                alternations
                                    .push(Hir::concat(vec![class_a, class_b, class_c, class_d]));
                            }
                        }
                    }
                }

                if alternations.len() == 1 {
                    alternations.remove(0) // remove and return the only element
                } else {
                    Hir::alternation(alternations)
                }
            }
        }
    }
}
