use crate::*;

struct ShouldNotMatchHandler {}

impl ShouldNotMatchHandler {
    fn new(_expected: Vec<Capture>) -> Self {
        Self {}
    }
}

impl MatchHandler for ShouldNotMatchHandler {
    fn on_match(&mut self, _id: usize, _matches: &[Capture]) -> ContinueMatching {
        assert!(false);
        ContinueMatching::No
    }
}

struct TestHandler {
    expected: Vec<Capture>,
    saw_match: bool,
}

impl TestHandler {
    fn new(expected: Vec<Capture>) -> Self {
        Self {
            expected,
            saw_match: false,
        }
    }
}

impl MatchHandler for TestHandler {
    fn on_match(&mut self, _id: usize, matches: &[Capture]) -> ContinueMatching {
        assert_ne!(matches.len(), 0);
        assert_eq!(self.expected[0], matches[0]);
        self.expected.remove(0);
        self.saw_match = true;
        ContinueMatching::Yes
    }

    fn on_reset(&mut self) {
        assert!(self.saw_match);
    }
}

#[test]
fn test_boundary_test_wb1() -> Result<(), Error> {
    let regex: &'static str = r##"\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb2() -> Result<(), Error> {
    let regex: &'static str = r##"\b"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(0)),
        Capture::new(Some(1), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb3() -> Result<(), Error> {
    let regex: &'static str = r##"\b"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(0)),
        Capture::new(Some(2), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb4() -> Result<(), Error> {
    let regex: &'static str = r##"^\b"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb5() -> Result<(), Error> {
    let regex: &'static str = r##"\b$"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb6() -> Result<(), Error> {
    let regex: &'static str = r##"^\b$"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb7() -> Result<(), Error> {
    let regex: &'static str = r##"\bbar\b"##;
    let text: &'static str = r##"nobar bar foo bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(6), Some(9)),
        Capture::new(Some(14), Some(17)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb8() -> Result<(), Error> {
    let regex: &'static str = r##"a\b"##;
    let text: &'static str = r##"faoa x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb9() -> Result<(), Error> {
    let regex: &'static str = r##"\bbar"##;
    let text: &'static str = r##"bar x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb10() -> Result<(), Error> {
    let regex: &'static str = r##"\bbar"##;
    let text: &'static str = r##"foo
bar x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(4), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb11() -> Result<(), Error> {
    let regex: &'static str = r##"bar\b"##;
    let text: &'static str = r##"foobar"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb12() -> Result<(), Error> {
    let regex: &'static str = r##"bar\b"##;
    let text: &'static str = r##"foobar
xxx"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb13() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb14() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"foo
"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb15() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb16() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"X"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb17() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"XY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb18() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"bar"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb19() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb20() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"foo
"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb21() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"ffoo bbar N x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(10), Some(11))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb22() -> Result<(), Error> {
    let regex: &'static str = r##"\b(fo|foo)\b"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb23() -> Result<(), Error> {
    let regex: &'static str = r##"\b(fo|foo)\b"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb24() -> Result<(), Error> {
    let regex: &'static str = r##"\b\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb25() -> Result<(), Error> {
    let regex: &'static str = r##"\b\b"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(0)),
        Capture::new(Some(1), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb26() -> Result<(), Error> {
    let regex: &'static str = r##"\b$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb27() -> Result<(), Error> {
    let regex: &'static str = r##"\b$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb28() -> Result<(), Error> {
    let regex: &'static str = r##"\b$"##;
    let text: &'static str = r##"y x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb29() -> Result<(), Error> {
    let regex: &'static str = r##"\b.$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb30() -> Result<(), Error> {
    let regex: &'static str = r##"^\b(fo|foo)\b"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb31() -> Result<(), Error> {
    let regex: &'static str = r##"^\b(fo|foo)\b"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb32() -> Result<(), Error> {
    let regex: &'static str = r##"^\b$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb33() -> Result<(), Error> {
    let regex: &'static str = r##"^\b$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb34() -> Result<(), Error> {
    let regex: &'static str = r##"^\b.$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb35() -> Result<(), Error> {
    let regex: &'static str = r##"^\b.\b$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb36() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^\b$$$$$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb37() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^\b.$$$$$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb38() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^\b$$$$$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb39() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^\b\b\b.\b\b\b$$$$$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb40() -> Result<(), Error> {
    let regex: &'static str = r##"\b.+\b"##;
    let text: &'static str = r##"$$abc$$"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(5))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_wb41() -> Result<(), Error> {
    let regex: &'static str = r##"\b"##;
    let text: &'static str = r##"a b c"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(0)),
        Capture::new(Some(1), Some(1)),
        Capture::new(Some(2), Some(2)),
        Capture::new(Some(3), Some(3)),
        Capture::new(Some(4), Some(4)),
        Capture::new(Some(5), Some(5)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb1() -> Result<(), Error> {
    let regex: &'static str = r##"\Bfoo\B"##;
    let text: &'static str = r##"n foo xfoox that"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(7), Some(10))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb2() -> Result<(), Error> {
    let regex: &'static str = r##"a\B"##;
    let text: &'static str = r##"faoa x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb3() -> Result<(), Error> {
    let regex: &'static str = r##"\Bbar"##;
    let text: &'static str = r##"bar x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb4() -> Result<(), Error> {
    let regex: &'static str = r##"\Bbar"##;
    let text: &'static str = r##"foo
bar x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb5() -> Result<(), Error> {
    let regex: &'static str = r##"bar\B"##;
    let text: &'static str = r##"foobar"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb6() -> Result<(), Error> {
    let regex: &'static str = r##"bar\B"##;
    let text: &'static str = r##"foobar
xxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb7() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar|[A-Z])\B"##;
    let text: &'static str = r##"foox"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb8() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar|[A-Z])\B"##;
    let text: &'static str = r##"foo
"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb9() -> Result<(), Error> {
    let regex: &'static str = r##"\B"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb10() -> Result<(), Error> {
    let regex: &'static str = r##"\B"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb11() -> Result<(), Error> {
    let regex: &'static str = r##"\B(foo|bar|[A-Z])"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb12() -> Result<(), Error> {
    let regex: &'static str = r##"\B(foo|bar|[A-Z])\B"##;
    let text: &'static str = r##"xXy"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb13() -> Result<(), Error> {
    let regex: &'static str = r##"\B(foo|bar|[A-Z])\B"##;
    let text: &'static str = r##"XY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb14() -> Result<(), Error> {
    let regex: &'static str = r##"\B(foo|bar|[A-Z])\B"##;
    let text: &'static str = r##"XYZ"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb15() -> Result<(), Error> {
    let regex: &'static str = r##"\B(foo|bar|[A-Z])\B"##;
    let text: &'static str = r##"abara"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb16() -> Result<(), Error> {
    let regex: &'static str = r##"\B(foo|bar|[A-Z])\B"##;
    let text: &'static str = r##"xfoo_"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb17() -> Result<(), Error> {
    let regex: &'static str = r##"\B(foo|bar|[A-Z])\B"##;
    let text: &'static str = r##"xfoo
"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb18() -> Result<(), Error> {
    let regex: &'static str = r##"\B(foo|bar|[A-Z])\B"##;
    let text: &'static str = r##"foo bar vNX"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(9), Some(10))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb19() -> Result<(), Error> {
    let regex: &'static str = r##"\B(fo|foo)\B"##;
    let text: &'static str = r##"xfoo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb20() -> Result<(), Error> {
    let regex: &'static str = r##"\B(foo|fo)\B"##;
    let text: &'static str = r##"xfooo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(1), Some(3)),
        Capture::new(Some(1), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb21() -> Result<(), Error> {
    let regex: &'static str = r##"\B\B"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb22() -> Result<(), Error> {
    let regex: &'static str = r##"\B\B"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb23() -> Result<(), Error> {
    let regex: &'static str = r##"\B$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb24() -> Result<(), Error> {
    let regex: &'static str = r##"\B$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb25() -> Result<(), Error> {
    let regex: &'static str = r##"\B$"##;
    let text: &'static str = r##"y x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb26() -> Result<(), Error> {
    let regex: &'static str = r##"\B.$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb27() -> Result<(), Error> {
    let regex: &'static str = r##"^\B(fo|foo)\B"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb28() -> Result<(), Error> {
    let regex: &'static str = r##"^\B(fo|foo)\B"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb29() -> Result<(), Error> {
    let regex: &'static str = r##"^\B"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb30() -> Result<(), Error> {
    let regex: &'static str = r##"^\B"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb31() -> Result<(), Error> {
    let regex: &'static str = r##"^\B\B"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb32() -> Result<(), Error> {
    let regex: &'static str = r##"^\B\B"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb33() -> Result<(), Error> {
    let regex: &'static str = r##"^\B$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb34() -> Result<(), Error> {
    let regex: &'static str = r##"^\B$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb35() -> Result<(), Error> {
    let regex: &'static str = r##"^\B.$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb36() -> Result<(), Error> {
    let regex: &'static str = r##"^\B.\B$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb37() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^\B$$$$$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = TestHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb38() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^\B.$$$$$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}

#[test]
fn test_boundary_test_nb39() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^\B$$$$$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = ShouldNotMatchHandler::new(matches);
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();
    Ok(())
}
