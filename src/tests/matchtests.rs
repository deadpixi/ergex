use crate::*;

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
        assert!(self.expected.len() <= matches.len());
        let matched: Vec<Capture> = matches
            .iter()
            .map(|x| *x)
            .take(self.expected.len())
            .collect();
        // because ergex is designed to do streaming and multi-matching, it can report
        // multiple matches; we just need to make sure the right one is in there.
        if self.expected == matched {
            self.saw_match = true;
        }
        ContinueMatching::Yes
    }

    fn on_reset(&mut self) {
        assert!(self.saw_match);
    }
}

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

#[test]
fn test_basic_dat_0000000002() -> Result<(), Error> {
    let regex: &'static str = r##"abracadabra$"##;
    let text: &'static str = r##"abracadabracadabra"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(7), Some(18))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(2, regex)
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
fn test_basic_dat_0000000003() -> Result<(), Error> {
    let regex: &'static str = r##"a...b"##;
    let text: &'static str = r##"abababbb"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(3, regex)
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
fn test_basic_dat_0000000004() -> Result<(), Error> {
    let regex: &'static str = r##"XXXXXX"##;
    let text: &'static str = r##"..XXXXXX"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(8))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(4, regex)
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
fn test_basic_dat_0000000005() -> Result<(), Error> {
    let regex: &'static str = r##"\)"##;
    let text: &'static str = r##"()"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(5, regex)
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
fn test_basic_dat_0000000006() -> Result<(), Error> {
    let regex: &'static str = r##"a]"##;
    let text: &'static str = r##"a]a"##;
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
fn test_basic_dat_0000000007() -> Result<(), Error> {
    let regex: &'static str = r##"}"##;
    let text: &'static str = r##"}"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(7, regex)
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
fn test_basic_dat_0000000008() -> Result<(), Error> {
    let regex: &'static str = r##"\}"##;
    let text: &'static str = r##"}"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(8, regex)
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
fn test_basic_dat_0000000009() -> Result<(), Error> {
    let regex: &'static str = r##"\]"##;
    let text: &'static str = r##"]"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(9, regex)
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
fn test_basic_dat_0000000010() -> Result<(), Error> {
    let regex: &'static str = r##"]"##;
    let text: &'static str = r##"]"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(10, regex)
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
fn test_basic_dat_0000000011() -> Result<(), Error> {
    let regex: &'static str = r##"]"##;
    let text: &'static str = r##"]"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(11, regex)
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
fn test_basic_dat_0000000013() -> Result<(), Error> {
    let regex: &'static str = r##"}"##;
    let text: &'static str = r##"}"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(13, regex)
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
fn test_basic_dat_0000000014() -> Result<(), Error> {
    let regex: &'static str = r##"^a"##;
    let text: &'static str = r##"ax"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(14, regex)
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
fn test_basic_dat_0000000015() -> Result<(), Error> {
    let regex: &'static str = r##"\^a"##;
    let text: &'static str = r##"a^a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(15, regex)
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
fn test_basic_dat_0000000016() -> Result<(), Error> {
    let regex: &'static str = r##"a\^"##;
    let text: &'static str = r##"a^"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(16, regex)
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
fn test_basic_dat_0000000017() -> Result<(), Error> {
    let regex: &'static str = r##"a$"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(17, regex)
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
fn test_basic_dat_0000000018() -> Result<(), Error> {
    let regex: &'static str = r##"a\$"##;
    let text: &'static str = r##"a$"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(18, regex)
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
fn test_basic_dat_0000000019() -> Result<(), Error> {
    let regex: &'static str = r##"^$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(19, regex)
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
fn test_basic_dat_0000000020() -> Result<(), Error> {
    let regex: &'static str = r##"$^"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(20, regex)
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
fn test_basic_dat_0000000021() -> Result<(), Error> {
    let regex: &'static str = r##"a($)"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(1), Some(2)),
        Capture::new(Some(2), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(21, regex)
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
fn test_basic_dat_0000000022() -> Result<(), Error> {
    let regex: &'static str = r##"a*(^a)"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(22, regex)
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
fn test_basic_dat_0000000023() -> Result<(), Error> {
    let regex: &'static str = r##"(..)*(...)*"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(23, regex)
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
fn test_basic_dat_0000000024() -> Result<(), Error> {
    let regex: &'static str = r##"(..)*(...)*"##;
    let text: &'static str = r##"abcd"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(2), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(24, regex)
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
fn test_basic_dat_0000000025() -> Result<(), Error> {
    let regex: &'static str = r##"(ab|a)(bc|c)"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(2), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(25, regex)
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
fn test_basic_dat_0000000026() -> Result<(), Error> {
    let regex: &'static str = r##"(ab)c|abc"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(26, regex)
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
fn test_basic_dat_0000000027() -> Result<(), Error> {
    let regex: &'static str = r##"a{0}b"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(27, regex)
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
fn test_basic_dat_0000000028() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)(b?)(b+)b{3}"##;
    let text: &'static str = r##"aaabbbbbbb"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(10)),
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(3), Some(4)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(28, regex)
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
fn test_basic_dat_0000000029() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)(b{0,1})(b{1,})b{3}"##;
    let text: &'static str = r##"aaabbbbbbb"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(10)),
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(3), Some(4)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(29, regex)
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
fn test_basic_dat_0000000031() -> Result<(), Error> {
    let regex: &'static str = r##"((a|a)|a)"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(31, regex)
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
fn test_basic_dat_0000000032() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)(a|aa)"##;
    let text: &'static str = r##"aaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(3), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(32, regex)
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
fn test_basic_dat_0000000033() -> Result<(), Error> {
    let regex: &'static str = r##"a*(a.|aa)"##;
    let text: &'static str = r##"aaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(2), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(33, regex)
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
fn test_basic_dat_0000000034() -> Result<(), Error> {
    let regex: &'static str = r##"a(b)|c(d)|a(e)f"##;
    let text: &'static str = r##"aef"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(None, None),
        Capture::new(None, None),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(34, regex)
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
fn test_basic_dat_0000000035() -> Result<(), Error> {
    let regex: &'static str = r##"(a|b)?.*"##;
    let text: &'static str = r##"b"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(35, regex)
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
fn test_basic_dat_0000000036() -> Result<(), Error> {
    let regex: &'static str = r##"(a|b)c|a(b|c)"##;
    let text: &'static str = r##"ac"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(36, regex)
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
fn test_basic_dat_0000000037() -> Result<(), Error> {
    let regex: &'static str = r##"(a|b)c|a(b|c)"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(37, regex)
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
fn test_basic_dat_0000000038() -> Result<(), Error> {
    let regex: &'static str = r##"(a|b)*c|(a|ab)*c"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(38, regex)
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
fn test_basic_dat_0000000039() -> Result<(), Error> {
    let regex: &'static str = r##"(a|b)*c|(a|ab)*c"##;
    let text: &'static str = r##"xc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(39, regex)
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
fn test_basic_dat_0000000040() -> Result<(), Error> {
    let regex: &'static str = r##"(.a|.b).*|.*(.a|.b)"##;
    let text: &'static str = r##"xa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(40, regex)
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
fn test_basic_dat_0000000041() -> Result<(), Error> {
    let regex: &'static str = r##"a?(ab|ba)ab"##;
    let text: &'static str = r##"abab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(41, regex)
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
fn test_basic_dat_0000000042() -> Result<(), Error> {
    let regex: &'static str = r##"a?(ac{0}b|ba)ab"##;
    let text: &'static str = r##"abab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(42, regex)
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
fn test_basic_dat_0000000043() -> Result<(), Error> {
    let regex: &'static str = r##"ab|abab"##;
    let text: &'static str = r##"abbabab"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(43, regex)
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
fn test_basic_dat_0000000044() -> Result<(), Error> {
    let regex: &'static str = r##"aba|bab|bba"##;
    let text: &'static str = r##"baaabbbaba"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(5), Some(8))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(44, regex)
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
fn test_basic_dat_0000000045() -> Result<(), Error> {
    let regex: &'static str = r##"aba|bab"##;
    let text: &'static str = r##"baaabbbaba"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(6), Some(9))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(45, regex)
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
fn test_basic_dat_0000000046() -> Result<(), Error> {
    let regex: &'static str = r##"(aa|aaa)*|(a|aaaaa)"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(46, regex)
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
fn test_basic_dat_0000000047() -> Result<(), Error> {
    let regex: &'static str = r##"(a.|.a.)*|(a|.a...)"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(47, regex)
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
fn test_basic_dat_0000000048() -> Result<(), Error> {
    let regex: &'static str = r##"ab|a"##;
    let text: &'static str = r##"xabc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(48, regex)
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
fn test_basic_dat_0000000049() -> Result<(), Error> {
    let regex: &'static str = r##"ab|a"##;
    let text: &'static str = r##"xxabc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(49, regex)
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
fn test_basic_dat_0000000050() -> Result<(), Error> {
    let regex: &'static str = r##"(Ab|cD)*"##;
    let text: &'static str = r##"aBcD"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(2), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(50, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(false)
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
fn test_basic_dat_0000000051() -> Result<(), Error> {
    let regex: &'static str = r##"[^-]"##;
    let text: &'static str = r##"--a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(51, regex)
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
fn test_basic_dat_0000000052() -> Result<(), Error> {
    let regex: &'static str = r##"[a-]*"##;
    let text: &'static str = r##"--a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(52, regex)
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
fn test_basic_dat_0000000053() -> Result<(), Error> {
    let regex: &'static str = r##"[a-m-]*"##;
    let text: &'static str = r##"--amoma--"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(53, regex)
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
fn test_basic_dat_0000000054() -> Result<(), Error> {
    let regex: &'static str = r##":::1:::0:|:::1:1:0:"##;
    let text: &'static str = r##":::0:::1:::1:::0:"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(8), Some(17))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(54, regex)
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
fn test_basic_dat_0000000055() -> Result<(), Error> {
    let regex: &'static str = r##":::1:::0:|:::1:1:1:"##;
    let text: &'static str = r##":::0:::1:::1:::0:"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(8), Some(17))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(55, regex)
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
fn test_basic_dat_0000000057() -> Result<(), Error> {
    let regex: &'static str = r##"[[:lower:]]+"##;
    let text: &'static str = r##"`az{"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(57, regex)
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
fn test_basic_dat_0000000058() -> Result<(), Error> {
    let regex: &'static str = r##"[[:upper:]]+"##;
    let text: &'static str = r##"@AZ["##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(58, regex)
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
fn test_basic_dat_0000000068() -> Result<(), Error> {
    let regex: &'static str = r##"(a)(b)(c)"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(1), Some(2)),
        Capture::new(Some(2), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(68, regex)
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
fn test_basic_dat_0000000069() -> Result<(), Error> {
    let regex: &'static str = r##"xxx"##;
    let text: &'static str = r##"xxx"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(69, regex)
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
fn test_basic_dat_0000000070() -> Result<(), Error> {
    let regex: &'static str = r##"(^|[ (,;])((([Ff]eb[^ ]* *|0*2/|\* */?)0*[6-7]))([^0-9]|$)"##;
    let text: &'static str = r##"feb 6,"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(70, regex)
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
fn test_basic_dat_0000000071() -> Result<(), Error> {
    let regex: &'static str = r##"(^|[ (,;])((([Ff]eb[^ ]* *|0*2/|\* */?)0*[6-7]))([^0-9]|$)"##;
    let text: &'static str = r##"2/7"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(71, regex)
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
fn test_basic_dat_0000000072() -> Result<(), Error> {
    let regex: &'static str = r##"(^|[ (,;])((([Ff]eb[^ ]* *|0*2/|\* */?)0*[6-7]))([^0-9]|$)"##;
    let text: &'static str = r##"feb 1,Feb 6"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(5), Some(11))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(72, regex)
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
fn test_basic_dat_0000000073() -> Result<(), Error> {
    let regex: &'static str = r##"((((((((((((((((((((((((((((((x))))))))))))))))))))))))))))))"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(73, regex)
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
fn test_basic_dat_0000000074() -> Result<(), Error> {
    let regex: &'static str = r##"((((((((((((((((((((((((((((((x))))))))))))))))))))))))))))))*"##;
    let text: &'static str = r##"xx"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(2)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(74, regex)
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
fn test_basic_dat_0000000075() -> Result<(), Error> {
    let regex: &'static str = r##"a?(ab|ba)*"##;
    let text: &'static str =
        r##"ababababababababababababababababababababababababababababababababababababababababa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(81)),
        Capture::new(Some(79), Some(81)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(75, regex)
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
fn test_basic_dat_0000000076() -> Result<(), Error> {
    let regex: &'static str = r##"abaa|abbaa|abbbaa|abbbbaa"##;
    let text: &'static str = r##"ababbabbbabbbabbbbabbbbaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(18), Some(25))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(76, regex)
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
fn test_basic_dat_0000000077() -> Result<(), Error> {
    let regex: &'static str = r##"abaa|abbaa|abbbaa|abbbbaa"##;
    let text: &'static str = r##"ababbabbbabbbabbbbabaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(18), Some(22))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(77, regex)
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
fn test_basic_dat_0000000078() -> Result<(), Error> {
    let regex: &'static str = r##"aaac|aabc|abac|abbc|baac|babc|bbac|bbbc"##;
    let text: &'static str = r##"baaabbbabac"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(7), Some(11))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(78, regex)
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
fn test_basic_dat_0000000080() -> Result<(), Error> {
    let regex: &'static str =
        r##"aaaa|bbbb|cccc|ddddd|eeeeee|fffffff|gggg|hhhh|iiiii|jjjjj|kkkkk|llll"##;
    let text: &'static str = r##"XaaaXbbbXcccXdddXeeeXfffXgggXhhhXiiiXjjjXkkkXlllXcbaXaaaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(53), Some(57))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(80, regex)
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
fn test_basic_dat_0000000082() -> Result<(), Error> {
    let regex: &'static str = r##"a*a*a*a*a*b"##;
    let text: &'static str = r##"aaaaaaaaab"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(10))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(82, regex)
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
fn test_basic_dat_0000000083() -> Result<(), Error> {
    let regex: &'static str = r##"^"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(83, regex)
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
fn test_basic_dat_0000000084() -> Result<(), Error> {
    let regex: &'static str = r##"$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(84, regex)
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
fn test_basic_dat_0000000085() -> Result<(), Error> {
    let regex: &'static str = r##"^$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(85, regex)
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
fn test_basic_dat_0000000086() -> Result<(), Error> {
    let regex: &'static str = r##"^a$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(86, regex)
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
fn test_basic_dat_0000000087() -> Result<(), Error> {
    let regex: &'static str = r##"abc"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(87, regex)
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
fn test_basic_dat_0000000088() -> Result<(), Error> {
    let regex: &'static str = r##"abc"##;
    let text: &'static str = r##"xabcy"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(88, regex)
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
fn test_basic_dat_0000000089() -> Result<(), Error> {
    let regex: &'static str = r##"abc"##;
    let text: &'static str = r##"ababc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(5))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(89, regex)
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
fn test_basic_dat_0000000090() -> Result<(), Error> {
    let regex: &'static str = r##"ab*c"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(90, regex)
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
fn test_basic_dat_0000000091() -> Result<(), Error> {
    let regex: &'static str = r##"ab*bc"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(91, regex)
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
fn test_basic_dat_0000000092() -> Result<(), Error> {
    let regex: &'static str = r##"ab*bc"##;
    let text: &'static str = r##"abbc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(92, regex)
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
fn test_basic_dat_0000000093() -> Result<(), Error> {
    let regex: &'static str = r##"ab*bc"##;
    let text: &'static str = r##"abbbbc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(93, regex)
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
fn test_basic_dat_0000000094() -> Result<(), Error> {
    let regex: &'static str = r##"ab+bc"##;
    let text: &'static str = r##"abbc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(94, regex)
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
fn test_basic_dat_0000000095() -> Result<(), Error> {
    let regex: &'static str = r##"ab+bc"##;
    let text: &'static str = r##"abbbbc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(95, regex)
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
fn test_basic_dat_0000000096() -> Result<(), Error> {
    let regex: &'static str = r##"ab?bc"##;
    let text: &'static str = r##"abbc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(96, regex)
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
fn test_basic_dat_0000000097() -> Result<(), Error> {
    let regex: &'static str = r##"ab?bc"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(97, regex)
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
fn test_basic_dat_0000000098() -> Result<(), Error> {
    let regex: &'static str = r##"ab?c"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(98, regex)
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
fn test_basic_dat_0000000099() -> Result<(), Error> {
    let regex: &'static str = r##"^abc$"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(99, regex)
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
fn test_basic_dat_0000000100() -> Result<(), Error> {
    let regex: &'static str = r##"^abc"##;
    let text: &'static str = r##"abcc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(100, regex)
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
fn test_basic_dat_0000000101() -> Result<(), Error> {
    let regex: &'static str = r##"abc$"##;
    let text: &'static str = r##"aabc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(101, regex)
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
fn test_basic_dat_0000000102() -> Result<(), Error> {
    let regex: &'static str = r##"^"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(102, regex)
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
fn test_basic_dat_0000000103() -> Result<(), Error> {
    let regex: &'static str = r##"$"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(103, regex)
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
fn test_basic_dat_0000000104() -> Result<(), Error> {
    let regex: &'static str = r##"a.c"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(104, regex)
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
fn test_basic_dat_0000000105() -> Result<(), Error> {
    let regex: &'static str = r##"a.c"##;
    let text: &'static str = r##"axc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(105, regex)
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
fn test_basic_dat_0000000106() -> Result<(), Error> {
    let regex: &'static str = r##"a.*c"##;
    let text: &'static str = r##"axyzc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(5))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(106, regex)
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
fn test_basic_dat_0000000107() -> Result<(), Error> {
    let regex: &'static str = r##"a[bc]d"##;
    let text: &'static str = r##"abd"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(107, regex)
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
fn test_basic_dat_0000000108() -> Result<(), Error> {
    let regex: &'static str = r##"a[b-d]e"##;
    let text: &'static str = r##"ace"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(108, regex)
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
fn test_basic_dat_0000000109() -> Result<(), Error> {
    let regex: &'static str = r##"a[b-d]"##;
    let text: &'static str = r##"aac"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(109, regex)
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
fn test_basic_dat_0000000110() -> Result<(), Error> {
    let regex: &'static str = r##"a[-b]"##;
    let text: &'static str = r##"a-"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(110, regex)
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
fn test_basic_dat_0000000111() -> Result<(), Error> {
    let regex: &'static str = r##"a[b-]"##;
    let text: &'static str = r##"a-"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(111, regex)
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
fn test_basic_dat_0000000112() -> Result<(), Error> {
    let regex: &'static str = r##"a]"##;
    let text: &'static str = r##"a]"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(112, regex)
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
fn test_basic_dat_0000000113() -> Result<(), Error> {
    let regex: &'static str = r##"a[]]b"##;
    let text: &'static str = r##"a]b"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(113, regex)
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
fn test_basic_dat_0000000114() -> Result<(), Error> {
    let regex: &'static str = r##"a[^bc]d"##;
    let text: &'static str = r##"aed"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(114, regex)
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
fn test_basic_dat_0000000115() -> Result<(), Error> {
    let regex: &'static str = r##"a[^-b]c"##;
    let text: &'static str = r##"adc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(115, regex)
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
fn test_basic_dat_0000000116() -> Result<(), Error> {
    let regex: &'static str = r##"a[^]b]c"##;
    let text: &'static str = r##"adc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(116, regex)
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
fn test_basic_dat_0000000117() -> Result<(), Error> {
    let regex: &'static str = r##"ab|cd"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(117, regex)
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
fn test_basic_dat_0000000118() -> Result<(), Error> {
    let regex: &'static str = r##"ab|cd"##;
    let text: &'static str = r##"abcd"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(118, regex)
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
fn test_basic_dat_0000000119() -> Result<(), Error> {
    let regex: &'static str = r##"a\(b"##;
    let text: &'static str = r##"a(b"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(119, regex)
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
fn test_basic_dat_0000000120() -> Result<(), Error> {
    let regex: &'static str = r##"a\(*b"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(120, regex)
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
fn test_basic_dat_0000000121() -> Result<(), Error> {
    let regex: &'static str = r##"a\(*b"##;
    let text: &'static str = r##"a((b"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(121, regex)
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
fn test_basic_dat_0000000122() -> Result<(), Error> {
    let regex: &'static str = r##"((a))"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(122, regex)
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
fn test_basic_dat_0000000123() -> Result<(), Error> {
    let regex: &'static str = r##"(a)b(c)"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(2), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(123, regex)
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
fn test_basic_dat_0000000124() -> Result<(), Error> {
    let regex: &'static str = r##"a+b+c"##;
    let text: &'static str = r##"aabbabc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(4), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(124, regex)
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
fn test_basic_dat_0000000125() -> Result<(), Error> {
    let regex: &'static str = r##"a*"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(125, regex)
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
fn test_basic_dat_0000000127() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)+"##;
    let text: &'static str = r##"-"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(0)),
        Capture::new(Some(0), Some(0)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(127, regex)
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
fn test_basic_dat_0000000129() -> Result<(), Error> {
    let regex: &'static str = r##"(a+|b)*"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(129, regex)
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
fn test_basic_dat_0000000130() -> Result<(), Error> {
    let regex: &'static str = r##"(a+|b)+"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(130, regex)
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
fn test_basic_dat_0000000131() -> Result<(), Error> {
    let regex: &'static str = r##"(a+|b)?"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(131, regex)
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
fn test_basic_dat_0000000132() -> Result<(), Error> {
    let regex: &'static str = r##"[^ab]*"##;
    let text: &'static str = r##"cde"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(132, regex)
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
fn test_basic_dat_0000000134() -> Result<(), Error> {
    let regex: &'static str = r##"a*"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(134, regex)
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
fn test_basic_dat_0000000135() -> Result<(), Error> {
    let regex: &'static str = r##"([abc])*d"##;
    let text: &'static str = r##"abbbcd"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(6)),
        Capture::new(Some(4), Some(5)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(135, regex)
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
fn test_basic_dat_0000000136() -> Result<(), Error> {
    let regex: &'static str = r##"([abc])*bcd"##;
    let text: &'static str = r##"abcd"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(136, regex)
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
fn test_basic_dat_0000000137() -> Result<(), Error> {
    let regex: &'static str = r##"a|b|c|d|e"##;
    let text: &'static str = r##"e"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(137, regex)
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
fn test_basic_dat_0000000138() -> Result<(), Error> {
    let regex: &'static str = r##"(a|b|c|d|e)f"##;
    let text: &'static str = r##"ef"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(138, regex)
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
fn test_basic_dat_0000000140() -> Result<(), Error> {
    let regex: &'static str = r##"abcd*efg"##;
    let text: &'static str = r##"abcdefg"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(140, regex)
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
fn test_basic_dat_0000000141() -> Result<(), Error> {
    let regex: &'static str = r##"ab*"##;
    let text: &'static str = r##"xabyabbbz"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(141, regex)
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
fn test_basic_dat_0000000142() -> Result<(), Error> {
    let regex: &'static str = r##"ab*"##;
    let text: &'static str = r##"xayabbbz"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(142, regex)
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
fn test_basic_dat_0000000143() -> Result<(), Error> {
    let regex: &'static str = r##"(ab|cd)e"##;
    let text: &'static str = r##"abcde"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(2), Some(5)),
        Capture::new(Some(2), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(143, regex)
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
fn test_basic_dat_0000000144() -> Result<(), Error> {
    let regex: &'static str = r##"[abhgefdc]ij"##;
    let text: &'static str = r##"hij"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(144, regex)
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
fn test_basic_dat_0000000145() -> Result<(), Error> {
    let regex: &'static str = r##"(a|b)c*d"##;
    let text: &'static str = r##"abcd"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(1), Some(4)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(145, regex)
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
fn test_basic_dat_0000000146() -> Result<(), Error> {
    let regex: &'static str = r##"(ab|ab*)bc"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(146, regex)
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
fn test_basic_dat_0000000147() -> Result<(), Error> {
    let regex: &'static str = r##"a([bc]*)c*"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(1), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(147, regex)
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
fn test_basic_dat_0000000148() -> Result<(), Error> {
    let regex: &'static str = r##"a([bc]*)(c*d)"##;
    let text: &'static str = r##"abcd"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(1), Some(3)),
        Capture::new(Some(3), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(148, regex)
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
fn test_basic_dat_0000000149() -> Result<(), Error> {
    let regex: &'static str = r##"a([bc]+)(c*d)"##;
    let text: &'static str = r##"abcd"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(1), Some(3)),
        Capture::new(Some(3), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(149, regex)
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
fn test_basic_dat_0000000150() -> Result<(), Error> {
    let regex: &'static str = r##"a([bc]*)(c+d)"##;
    let text: &'static str = r##"abcd"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(1), Some(2)),
        Capture::new(Some(2), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(150, regex)
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
fn test_basic_dat_0000000151() -> Result<(), Error> {
    let regex: &'static str = r##"a[bcd]*dcdcde"##;
    let text: &'static str = r##"adcdcde"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(151, regex)
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
fn test_basic_dat_0000000152() -> Result<(), Error> {
    let regex: &'static str = r##"(ab|a)b*c"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(152, regex)
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
fn test_basic_dat_0000000153() -> Result<(), Error> {
    let regex: &'static str = r##"((a)(b)c)(d)"##;
    let text: &'static str = r##"abcd"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(1), Some(2)),
        Capture::new(Some(3), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(153, regex)
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
fn test_basic_dat_0000000154() -> Result<(), Error> {
    let regex: &'static str = r##"[A-Za-z_][A-Za-z0-9_]*"##;
    let text: &'static str = r##"alpha"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(5))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(154, regex)
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
fn test_basic_dat_0000000155() -> Result<(), Error> {
    let regex: &'static str = r##"^a(bc+|b[eh])g|.h$"##;
    let text: &'static str = r##"abh"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(155, regex)
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
fn test_basic_dat_0000000156() -> Result<(), Error> {
    let regex: &'static str = r##"(bc+d$|ef*g.|h?i(j|k))"##;
    let text: &'static str = r##"effgz"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(5)),
        Capture::new(Some(0), Some(5)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(156, regex)
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
fn test_basic_dat_0000000157() -> Result<(), Error> {
    let regex: &'static str = r##"(bc+d$|ef*g.|h?i(j|k))"##;
    let text: &'static str = r##"ij"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(157, regex)
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
fn test_basic_dat_0000000158() -> Result<(), Error> {
    let regex: &'static str = r##"(bc+d$|ef*g.|h?i(j|k))"##;
    let text: &'static str = r##"reffgz"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(1), Some(6)),
        Capture::new(Some(1), Some(6)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(158, regex)
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
fn test_basic_dat_0000000159() -> Result<(), Error> {
    let regex: &'static str = r##"(((((((((a)))))))))"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(159, regex)
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
fn test_basic_dat_0000000160() -> Result<(), Error> {
    let regex: &'static str = r##"multiple words"##;
    let text: &'static str = r##"multiple words yeah"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(14))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(160, regex)
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
fn test_basic_dat_0000000161() -> Result<(), Error> {
    let regex: &'static str = r##"(.*)c(.*)"##;
    let text: &'static str = r##"abcde"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(5)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(3), Some(5)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(161, regex)
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
fn test_basic_dat_0000000162() -> Result<(), Error> {
    let regex: &'static str = r##"abcd"##;
    let text: &'static str = r##"abcd"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(162, regex)
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
fn test_basic_dat_0000000163() -> Result<(), Error> {
    let regex: &'static str = r##"a(bc)d"##;
    let text: &'static str = r##"abcd"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(1), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(163, regex)
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
fn test_basic_dat_0000000164() -> Result<(), Error> {
    let regex: &'static str = r##"a[-]?c"##;
    let text: &'static str = r##"ac"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(164, regex)
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
fn test_basic_dat_0000000165() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muammar Qaddafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(15)),
        Capture::new(None, None),
        Capture::new(Some(10), Some(12)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(165, regex)
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
fn test_basic_dat_0000000166() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Mo'ammar Gadhafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(16)),
        Capture::new(None, None),
        Capture::new(Some(11), Some(13)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(166, regex)
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
fn test_basic_dat_0000000167() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muammar Kaddafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(15)),
        Capture::new(None, None),
        Capture::new(Some(10), Some(12)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(167, regex)
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
fn test_basic_dat_0000000168() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muammar Qadhafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(15)),
        Capture::new(None, None),
        Capture::new(Some(10), Some(12)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(168, regex)
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
fn test_basic_dat_0000000169() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muammar Gadafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(14)),
        Capture::new(None, None),
        Capture::new(Some(10), Some(11)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(169, regex)
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
fn test_basic_dat_0000000170() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Mu'ammar Qadafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(15)),
        Capture::new(None, None),
        Capture::new(Some(11), Some(12)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(170, regex)
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
fn test_basic_dat_0000000171() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Moamar Gaddafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(14)),
        Capture::new(None, None),
        Capture::new(Some(9), Some(11)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(171, regex)
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
fn test_basic_dat_0000000172() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Mu'ammar Qadhdhafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(18)),
        Capture::new(None, None),
        Capture::new(Some(13), Some(15)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(172, regex)
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
fn test_basic_dat_0000000173() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muammar Khaddafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(16)),
        Capture::new(None, None),
        Capture::new(Some(11), Some(13)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(173, regex)
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
fn test_basic_dat_0000000174() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muammar Ghaddafy"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(16)),
        Capture::new(None, None),
        Capture::new(Some(11), Some(13)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(174, regex)
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
fn test_basic_dat_0000000175() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muammar Ghadafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(15)),
        Capture::new(None, None),
        Capture::new(Some(11), Some(12)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(175, regex)
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
fn test_basic_dat_0000000176() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muammar Ghaddafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(16)),
        Capture::new(None, None),
        Capture::new(Some(11), Some(13)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(176, regex)
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
fn test_basic_dat_0000000177() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muamar Kaddafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(14)),
        Capture::new(None, None),
        Capture::new(Some(9), Some(11)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(177, regex)
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
fn test_basic_dat_0000000178() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muammar Quathafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(16)),
        Capture::new(None, None),
        Capture::new(Some(11), Some(13)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(178, regex)
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
fn test_basic_dat_0000000179() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Muammar Gheddafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(16)),
        Capture::new(None, None),
        Capture::new(Some(11), Some(13)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(179, regex)
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
fn test_basic_dat_0000000180() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Moammar Khadafy"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(15)),
        Capture::new(None, None),
        Capture::new(Some(11), Some(12)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(180, regex)
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
fn test_basic_dat_0000000181() -> Result<(), Error> {
    let regex: &'static str =
        r##"M[ou]'?am+[ae]r .*([AEae]l[- ])?[GKQ]h?[aeu]+([dtz][dhz]?)+af[iy]"##;
    let text: &'static str = r##"Moammar Qudhafi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(15)),
        Capture::new(None, None),
        Capture::new(Some(10), Some(12)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(181, regex)
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
fn test_basic_dat_0000000182() -> Result<(), Error> {
    let regex: &'static str = r##"a+(b|c)*d+"##;
    let text: &'static str = r##"aabcdd"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(6)),
        Capture::new(Some(3), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(182, regex)
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
fn test_basic_dat_0000000183() -> Result<(), Error> {
    let regex: &'static str = r##"^.+$"##;
    let text: &'static str = r##"vivi"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(183, regex)
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
fn test_basic_dat_0000000184() -> Result<(), Error> {
    let regex: &'static str = r##"^(.+)$"##;
    let text: &'static str = r##"vivi"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(184, regex)
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
fn test_basic_dat_0000000185() -> Result<(), Error> {
    let regex: &'static str = r##"^([^!.]+).att.com!(.+)$"##;
    let text: &'static str = r##"gryphon.att.com!eby"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(19)),
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(16), Some(19)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(185, regex)
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
fn test_basic_dat_0000000186() -> Result<(), Error> {
    let regex: &'static str = r##"^([^!]+!)?([^!]+)$"##;
    let text: &'static str = r##"bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(None, None),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(186, regex)
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
fn test_basic_dat_0000000187() -> Result<(), Error> {
    let regex: &'static str = r##"^([^!]+!)?([^!]+)$"##;
    let text: &'static str = r##"bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(187, regex)
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
fn test_basic_dat_0000000188() -> Result<(), Error> {
    let regex: &'static str = r##"^([^!]+!)?([^!]+)$"##;
    let text: &'static str = r##"foo!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(188, regex)
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
fn test_basic_dat_0000000189() -> Result<(), Error> {
    let regex: &'static str = r##"^.+!([^!]+!)([^!]+)$"##;
    let text: &'static str = r##"foo!bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(11)),
        Capture::new(Some(4), Some(8)),
        Capture::new(Some(8), Some(11)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(189, regex)
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
fn test_basic_dat_0000000190() -> Result<(), Error> {
    let regex: &'static str = r##"((foo)|(bar))!bas"##;
    let text: &'static str = r##"bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(3)),
        Capture::new(None, None),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(190, regex)
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
fn test_basic_dat_0000000191() -> Result<(), Error> {
    let regex: &'static str = r##"((foo)|(bar))!bas"##;
    let text: &'static str = r##"foo!bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(4), Some(11)),
        Capture::new(Some(4), Some(7)),
        Capture::new(None, None),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(191, regex)
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
fn test_basic_dat_0000000192() -> Result<(), Error> {
    let regex: &'static str = r##"((foo)|(bar))!bas"##;
    let text: &'static str = r##"foo!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(192, regex)
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
fn test_basic_dat_0000000193() -> Result<(), Error> {
    let regex: &'static str = r##"((foo)|bar)!bas"##;
    let text: &'static str = r##"bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(193, regex)
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
fn test_basic_dat_0000000194() -> Result<(), Error> {
    let regex: &'static str = r##"((foo)|bar)!bas"##;
    let text: &'static str = r##"foo!bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(4), Some(11)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(194, regex)
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
fn test_basic_dat_0000000195() -> Result<(), Error> {
    let regex: &'static str = r##"((foo)|bar)!bas"##;
    let text: &'static str = r##"foo!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(195, regex)
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
fn test_basic_dat_0000000196() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|(bar))!bas"##;
    let text: &'static str = r##"bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(196, regex)
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
fn test_basic_dat_0000000197() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|(bar))!bas"##;
    let text: &'static str = r##"foo!bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(4), Some(11)),
        Capture::new(Some(4), Some(7)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(197, regex)
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
fn test_basic_dat_0000000198() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|(bar))!bas"##;
    let text: &'static str = r##"foo!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(198, regex)
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
fn test_basic_dat_0000000199() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar)!bas"##;
    let text: &'static str = r##"bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(199, regex)
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
fn test_basic_dat_0000000200() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar)!bas"##;
    let text: &'static str = r##"foo!bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(4), Some(11)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(200, regex)
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
fn test_basic_dat_0000000201() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar)!bas"##;
    let text: &'static str = r##"foo!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(201, regex)
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
fn test_basic_dat_0000000202() -> Result<(), Error> {
    let regex: &'static str = r##"^(([^!]+!)?([^!]+)|.+!([^!]+!)([^!]+))$"##;
    let text: &'static str = r##"foo!bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(11)),
        Capture::new(Some(0), Some(11)),
        Capture::new(None, None),
        Capture::new(None, None),
        Capture::new(Some(4), Some(8)),
        Capture::new(Some(8), Some(11)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(202, regex)
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
fn test_basic_dat_0000000203() -> Result<(), Error> {
    let regex: &'static str = r##"^([^!]+!)?([^!]+)$|^.+!([^!]+!)([^!]+)$"##;
    let text: &'static str = r##"bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(None, None),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(203, regex)
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
fn test_basic_dat_0000000204() -> Result<(), Error> {
    let regex: &'static str = r##"^([^!]+!)?([^!]+)$|^.+!([^!]+!)([^!]+)$"##;
    let text: &'static str = r##"bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(204, regex)
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
fn test_basic_dat_0000000205() -> Result<(), Error> {
    let regex: &'static str = r##"^([^!]+!)?([^!]+)$|^.+!([^!]+!)([^!]+)$"##;
    let text: &'static str = r##"foo!bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(11)),
        Capture::new(None, None),
        Capture::new(None, None),
        Capture::new(Some(4), Some(8)),
        Capture::new(Some(8), Some(11)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(205, regex)
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
fn test_basic_dat_0000000206() -> Result<(), Error> {
    let regex: &'static str = r##"^([^!]+!)?([^!]+)$|^.+!([^!]+!)([^!]+)$"##;
    let text: &'static str = r##"foo!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(206, regex)
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
fn test_basic_dat_0000000207() -> Result<(), Error> {
    let regex: &'static str = r##"^(([^!]+!)?([^!]+)|.+!([^!]+!)([^!]+))$"##;
    let text: &'static str = r##"bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
        Capture::new(None, None),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(207, regex)
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
fn test_basic_dat_0000000208() -> Result<(), Error> {
    let regex: &'static str = r##"^(([^!]+!)?([^!]+)|.+!([^!]+!)([^!]+))$"##;
    let text: &'static str = r##"bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(208, regex)
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
fn test_basic_dat_0000000209() -> Result<(), Error> {
    let regex: &'static str = r##"^(([^!]+!)?([^!]+)|.+!([^!]+!)([^!]+))$"##;
    let text: &'static str = r##"foo!bar!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(11)),
        Capture::new(Some(0), Some(11)),
        Capture::new(None, None),
        Capture::new(None, None),
        Capture::new(Some(4), Some(8)),
        Capture::new(Some(8), Some(11)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(209, regex)
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
fn test_basic_dat_0000000210() -> Result<(), Error> {
    let regex: &'static str = r##"^(([^!]+!)?([^!]+)|.+!([^!]+!)([^!]+))$"##;
    let text: &'static str = r##"foo!bas"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(7)),
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(4), Some(7)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(210, regex)
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
fn test_basic_dat_0000000211() -> Result<(), Error> {
    let regex: &'static str = r##".*(/XXX).*"##;
    let text: &'static str = r##"/XXX"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(211, regex)
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
fn test_basic_dat_0000000212() -> Result<(), Error> {
    let regex: &'static str = r##".*(\\XXX).*"##;
    let text: &'static str = r##"\XXX"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(212, regex)
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
fn test_basic_dat_0000000213() -> Result<(), Error> {
    let regex: &'static str = r##"\\XXX"##;
    let text: &'static str = r##"\XXX"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(213, regex)
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
fn test_basic_dat_0000000214() -> Result<(), Error> {
    let regex: &'static str = r##".*(/000).*"##;
    let text: &'static str = r##"/000"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(214, regex)
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
fn test_basic_dat_0000000215() -> Result<(), Error> {
    let regex: &'static str = r##".*(\\000).*"##;
    let text: &'static str = r##"\000"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(215, regex)
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
fn test_basic_dat_0000000216() -> Result<(), Error> {
    let regex: &'static str = r##"\\000"##;
    let text: &'static str = r##"\000"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(216, regex)
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
fn test_nullsubexpr_dat_0000000006() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)+"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
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
fn test_nullsubexpr_dat_0000000007() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)+"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(0)),
        Capture::new(Some(0), Some(0)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(7, regex)
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
fn test_nullsubexpr_dat_0000000008() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)+"##;
    let text: &'static str = r##"aaaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(6)),
        Capture::new(Some(0), Some(6)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(8, regex)
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
fn test_nullsubexpr_dat_0000000009() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)+"##;
    let text: &'static str = r##"aaaaaax"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(6)),
        Capture::new(Some(0), Some(6)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(9, regex)
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
fn test_nullsubexpr_dat_0000000010() -> Result<(), Error> {
    let regex: &'static str = r##"(a+)*"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(10, regex)
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
fn test_nullsubexpr_dat_0000000011() -> Result<(), Error> {
    let regex: &'static str = r##"(a+)*"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(11, regex)
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
fn test_nullsubexpr_dat_0000000012() -> Result<(), Error> {
    let regex: &'static str = r##"(a+)*"##;
    let text: &'static str = r##"aaaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(6)),
        Capture::new(Some(0), Some(6)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(12, regex)
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
fn test_nullsubexpr_dat_0000000013() -> Result<(), Error> {
    let regex: &'static str = r##"(a+)*"##;
    let text: &'static str = r##"aaaaaax"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(6)),
        Capture::new(Some(0), Some(6)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(13, regex)
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
fn test_nullsubexpr_dat_0000000014() -> Result<(), Error> {
    let regex: &'static str = r##"(a+)+"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(14, regex)
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
fn test_nullsubexpr_dat_0000000015() -> Result<(), Error> {
    let regex: &'static str = r##"(a+)+"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(15, regex)
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
fn test_nullsubexpr_dat_0000000016() -> Result<(), Error> {
    let regex: &'static str = r##"(a+)+"##;
    let text: &'static str = r##"aaaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(6)),
        Capture::new(Some(0), Some(6)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(16, regex)
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
fn test_nullsubexpr_dat_0000000017() -> Result<(), Error> {
    let regex: &'static str = r##"(a+)+"##;
    let text: &'static str = r##"aaaaaax"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(6)),
        Capture::new(Some(0), Some(6)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(17, regex)
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
fn test_nullsubexpr_dat_0000000044() -> Result<(), Error> {
    let regex: &'static str = r##"((z)+|a)*"##;
    let text: &'static str = r##"zabcde"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(44, regex)
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
fn test_nullsubexpr_dat_0000000063() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)*(x)"##;
    let text: &'static str = r##"ax"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(63, regex)
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
fn test_nullsubexpr_dat_0000000064() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)*(x)"##;
    let text: &'static str = r##"axa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(64, regex)
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
fn test_nullsubexpr_dat_0000000066() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)+(x)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(0)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(66, regex)
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
fn test_nullsubexpr_dat_0000000067() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)+(x)"##;
    let text: &'static str = r##"ax"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(67, regex)
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
fn test_nullsubexpr_dat_0000000068() -> Result<(), Error> {
    let regex: &'static str = r##"(a*)+(x)"##;
    let text: &'static str = r##"axa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(68, regex)
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
fn test_nullsubexpr_dat_0000000070() -> Result<(), Error> {
    let regex: &'static str = r##"(a*){2}(x)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(0)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(70, regex)
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
fn test_nullsubexpr_dat_0000000071() -> Result<(), Error> {
    let regex: &'static str = r##"(a*){2}(x)"##;
    let text: &'static str = r##"ax"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(1)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(71, regex)
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
fn test_nullsubexpr_dat_0000000072() -> Result<(), Error> {
    let regex: &'static str = r##"(a*){2}(x)"##;
    let text: &'static str = r##"axa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(1)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(72, regex)
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
fn test_re2_dat_0000000000() -> Result<(), Error> {
    let regex: &'static str = r##"a"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(0, regex)
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
fn test_re2_dat_0000000001() -> Result<(), Error> {
    let regex: &'static str = r##"a"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(1, regex)
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
fn test_re2_dat_0000000002() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(2, regex)
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
fn test_re2_dat_0000000003() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(3, regex)
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
fn test_re2_dat_0000000004() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(4, regex)
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
fn test_re2_dat_0000000005() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(5, regex)
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
fn test_re2_dat_0000000006() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a)$"##;
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
fn test_re2_dat_0000000007() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a)$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(7, regex)
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
fn test_re2_dat_0000000008() -> Result<(), Error> {
    let regex: &'static str = r##"a"##;
    let text: &'static str = r##"zyzzyva"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(6), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(8, regex)
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
fn test_re2_dat_0000000009() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)$"##;
    let text: &'static str = r##"zyzzyva"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(9, regex)
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
fn test_re2_dat_0000000010() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)"##;
    let text: &'static str = r##"zyzzyva"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(10, regex)
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
fn test_re2_dat_0000000011() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a)$"##;
    let text: &'static str = r##"zyzzyva"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(6), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(11, regex)
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
fn test_re2_dat_0000000012() -> Result<(), Error> {
    let regex: &'static str = r##"a+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(12, regex)
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
fn test_re2_dat_0000000013() -> Result<(), Error> {
    let regex: &'static str = r##"a+"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(13, regex)
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
fn test_re2_dat_0000000014() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(14, regex)
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
fn test_re2_dat_0000000015() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a+)$"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(15, regex)
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
fn test_re2_dat_0000000016() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(16, regex)
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
fn test_re2_dat_0000000017() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a+)"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(17, regex)
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
fn test_re2_dat_0000000018() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(18, regex)
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
fn test_re2_dat_0000000019() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a+)$"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(19, regex)
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
fn test_re2_dat_0000000020() -> Result<(), Error> {
    let regex: &'static str = r##"(a+|b)+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(20, regex)
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
fn test_re2_dat_0000000021() -> Result<(), Error> {
    let regex: &'static str = r##"(a+|b)+"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(21, regex)
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
fn test_re2_dat_0000000022() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(a+|b)+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(22, regex)
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
fn test_re2_dat_0000000023() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(a+|b)+)$"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(23, regex)
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
fn test_re2_dat_0000000024() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(a+|b)+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(24, regex)
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
fn test_re2_dat_0000000025() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(a+|b)+)"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(25, regex)
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
fn test_re2_dat_0000000026() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(a+|b)+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(26, regex)
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
fn test_re2_dat_0000000027() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(a+|b)+)$"##;
    let text: &'static str = r##"ab"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(27, regex)
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
fn test_re2_dat_0000000028() -> Result<(), Error> {
    let regex: &'static str = r##"ab|cd"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(28, regex)
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
fn test_re2_dat_0000000029() -> Result<(), Error> {
    let regex: &'static str = r##"ab|cd"##;
    let text: &'static str = r##"xabcdx"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(29, regex)
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
fn test_re2_dat_0000000030() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ab|cd)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(30, regex)
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
fn test_re2_dat_0000000031() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ab|cd)$"##;
    let text: &'static str = r##"xabcdx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(31, regex)
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
fn test_re2_dat_0000000032() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ab|cd)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(32, regex)
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
fn test_re2_dat_0000000033() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ab|cd)"##;
    let text: &'static str = r##"xabcdx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(33, regex)
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
fn test_re2_dat_0000000034() -> Result<(), Error> {
    let regex: &'static str = r##"(?:ab|cd)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(34, regex)
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
fn test_re2_dat_0000000035() -> Result<(), Error> {
    let regex: &'static str = r##"(?:ab|cd)$"##;
    let text: &'static str = r##"xabcdx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(35, regex)
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
fn test_re2_dat_0000000036() -> Result<(), Error> {
    let regex: &'static str = r##"h.*od?"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(36, regex)
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
fn test_re2_dat_0000000037() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:h.*od?)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(37, regex)
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
fn test_re2_dat_0000000038() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:h.*od?)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(38, regex)
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
fn test_re2_dat_0000000039() -> Result<(), Error> {
    let regex: &'static str = r##"(?:h.*od?)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(39, regex)
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
fn test_re2_dat_0000000040() -> Result<(), Error> {
    let regex: &'static str = r##"h.*o"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(40, regex)
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
fn test_re2_dat_0000000041() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:h.*o)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(41, regex)
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
fn test_re2_dat_0000000042() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:h.*o)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(42, regex)
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
fn test_re2_dat_0000000043() -> Result<(), Error> {
    let regex: &'static str = r##"(?:h.*o)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(43, regex)
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
fn test_re2_dat_0000000044() -> Result<(), Error> {
    let regex: &'static str = r##"h.*o"##;
    let text: &'static str = r##"hello world"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(8))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(44, regex)
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
fn test_re2_dat_0000000045() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:h.*o)$"##;
    let text: &'static str = r##"hello world"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(45, regex)
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
fn test_re2_dat_0000000046() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:h.*o)"##;
    let text: &'static str = r##"hello world"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(8))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(46, regex)
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
fn test_re2_dat_0000000047() -> Result<(), Error> {
    let regex: &'static str = r##"(?:h.*o)$"##;
    let text: &'static str = r##"hello world"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(47, regex)
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
fn test_re2_dat_0000000048() -> Result<(), Error> {
    let regex: &'static str = r##"h.*o"##;
    let text: &'static str = r##"othello, world"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(11))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(48, regex)
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
fn test_re2_dat_0000000049() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:h.*o)$"##;
    let text: &'static str = r##"othello, world"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(49, regex)
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
fn test_re2_dat_0000000050() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:h.*o)"##;
    let text: &'static str = r##"othello, world"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(50, regex)
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
fn test_re2_dat_0000000051() -> Result<(), Error> {
    let regex: &'static str = r##"(?:h.*o)$"##;
    let text: &'static str = r##"othello, world"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(51, regex)
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
fn test_re2_dat_0000000052() -> Result<(), Error> {
    let regex: &'static str = r##"a"##;
    let text: &'static str = r##"aaaaaaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(52, regex)
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
fn test_re2_dat_0000000053() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)$"##;
    let text: &'static str = r##"aaaaaaa"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(53, regex)
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
fn test_re2_dat_0000000054() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)"##;
    let text: &'static str = r##"aaaaaaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(54, regex)
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
fn test_re2_dat_0000000055() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a)$"##;
    let text: &'static str = r##"aaaaaaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(6), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(55, regex)
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
fn test_re2_dat_0000000056() -> Result<(), Error> {
    let regex: &'static str = r##"a*"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(56, regex)
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
fn test_re2_dat_0000000057() -> Result<(), Error> {
    let regex: &'static str = r##"a*"##;
    let text: &'static str = r##"aaaaaaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(57, regex)
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
fn test_re2_dat_0000000058() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a*)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(58, regex)
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
fn test_re2_dat_0000000059() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a*)$"##;
    let text: &'static str = r##"aaaaaaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(59, regex)
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
fn test_re2_dat_0000000060() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a*)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(60, regex)
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
fn test_re2_dat_0000000061() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a*)"##;
    let text: &'static str = r##"aaaaaaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(61, regex)
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
fn test_re2_dat_0000000062() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a*)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(62, regex)
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
fn test_re2_dat_0000000063() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a*)$"##;
    let text: &'static str = r##"aaaaaaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(63, regex)
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
fn test_re2_dat_0000000064() -> Result<(), Error> {
    let regex: &'static str = r##"a"##;
    let text: &'static str = r##"cab"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(64, regex)
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
fn test_re2_dat_0000000065() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)$"##;
    let text: &'static str = r##"cab"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(65, regex)
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
fn test_re2_dat_0000000066() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)"##;
    let text: &'static str = r##"cab"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(66, regex)
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
fn test_re2_dat_0000000067() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a)$"##;
    let text: &'static str = r##"cab"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(67, regex)
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
fn test_re2_dat_0000000068() -> Result<(), Error> {
    let regex: &'static str = r##"a*b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(68, regex)
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
fn test_re2_dat_0000000069() -> Result<(), Error> {
    let regex: &'static str = r##"a*b"##;
    let text: &'static str = r##"cab"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(69, regex)
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
fn test_re2_dat_0000000070() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a*b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(70, regex)
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
fn test_re2_dat_0000000071() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a*b)$"##;
    let text: &'static str = r##"cab"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(71, regex)
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
fn test_re2_dat_0000000072() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a*b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(72, regex)
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
fn test_re2_dat_0000000073() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a*b)"##;
    let text: &'static str = r##"cab"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(73, regex)
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
fn test_re2_dat_0000000074() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a*b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(74, regex)
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
fn test_re2_dat_0000000075() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a*b)$"##;
    let text: &'static str = r##"cab"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(75, regex)
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
fn test_re2_dat_0000000076() -> Result<(), Error> {
    let regex: &'static str = r##"((((((((((((((((((((x))))))))))))))))))))"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(76, regex)
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
fn test_re2_dat_0000000077() -> Result<(), Error> {
    let regex: &'static str = r##"((((((((((((((((((((x))))))))))))))))))))"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(77, regex)
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
fn test_re2_dat_0000000078() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:((((((((((((((((((((x)))))))))))))))))))))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(78, regex)
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
fn test_re2_dat_0000000079() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:((((((((((((((((((((x)))))))))))))))))))))$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(79, regex)
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
fn test_re2_dat_0000000080() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:((((((((((((((((((((x)))))))))))))))))))))"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(80, regex)
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
fn test_re2_dat_0000000081() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:((((((((((((((((((((x)))))))))))))))))))))"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(81, regex)
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
fn test_re2_dat_0000000082() -> Result<(), Error> {
    let regex: &'static str = r##"(?:((((((((((((((((((((x)))))))))))))))))))))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(82, regex)
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
fn test_re2_dat_0000000083() -> Result<(), Error> {
    let regex: &'static str = r##"(?:((((((((((((((((((((x)))))))))))))))))))))$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(83, regex)
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
fn test_re2_dat_0000000084() -> Result<(), Error> {
    let regex: &'static str = r##"[abcd]"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(84, regex)
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
fn test_re2_dat_0000000085() -> Result<(), Error> {
    let regex: &'static str = r##"[abcd]"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(85, regex)
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
fn test_re2_dat_0000000086() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[abcd])$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(86, regex)
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
fn test_re2_dat_0000000087() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[abcd])$"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(87, regex)
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
fn test_re2_dat_0000000088() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[abcd])"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(88, regex)
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
fn test_re2_dat_0000000089() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[abcd])"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(89, regex)
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
fn test_re2_dat_0000000090() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[abcd])$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(90, regex)
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
fn test_re2_dat_0000000091() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[abcd])$"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(91, regex)
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
fn test_re2_dat_0000000092() -> Result<(), Error> {
    let regex: &'static str = r##"[^x]"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(92, regex)
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
fn test_re2_dat_0000000093() -> Result<(), Error> {
    let regex: &'static str = r##"[^x]"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(93, regex)
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
fn test_re2_dat_0000000094() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^x])$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(94, regex)
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
fn test_re2_dat_0000000095() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^x])$"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(95, regex)
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
fn test_re2_dat_0000000096() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^x])"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(96, regex)
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
fn test_re2_dat_0000000097() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^x])"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(97, regex)
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
fn test_re2_dat_0000000098() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[^x])$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(98, regex)
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
fn test_re2_dat_0000000099() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[^x])$"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(99, regex)
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
fn test_re2_dat_0000000100() -> Result<(), Error> {
    let regex: &'static str = r##"[abcd]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(100, regex)
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
fn test_re2_dat_0000000101() -> Result<(), Error> {
    let regex: &'static str = r##"[abcd]+"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(101, regex)
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
fn test_re2_dat_0000000102() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[abcd]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(102, regex)
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
fn test_re2_dat_0000000103() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[abcd]+)$"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(103, regex)
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
fn test_re2_dat_0000000104() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[abcd]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(104, regex)
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
fn test_re2_dat_0000000105() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[abcd]+)"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(105, regex)
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
fn test_re2_dat_0000000106() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[abcd]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(106, regex)
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
fn test_re2_dat_0000000107() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[abcd]+)$"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(107, regex)
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
fn test_re2_dat_0000000108() -> Result<(), Error> {
    let regex: &'static str = r##"[^x]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(108, regex)
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
fn test_re2_dat_0000000109() -> Result<(), Error> {
    let regex: &'static str = r##"[^x]+"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(7))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(109, regex)
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
fn test_re2_dat_0000000110() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^x]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(110, regex)
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
fn test_re2_dat_0000000111() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^x]+)$"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(111, regex)
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
fn test_re2_dat_0000000112() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^x]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(112, regex)
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
fn test_re2_dat_0000000113() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^x]+)"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(113, regex)
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
fn test_re2_dat_0000000114() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[^x]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(114, regex)
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
fn test_re2_dat_0000000115() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[^x]+)$"##;
    let text: &'static str = r##"xxxabcdxxx"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(115, regex)
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
fn test_re2_dat_0000000116() -> Result<(), Error> {
    let regex: &'static str = r##"(fo|foo)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(116, regex)
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
fn test_re2_dat_0000000117() -> Result<(), Error> {
    let regex: &'static str = r##"(fo|foo)"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(117, regex)
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
fn test_re2_dat_0000000118() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(fo|foo))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(118, regex)
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
fn test_re2_dat_0000000119() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(fo|foo))$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(119, regex)
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
fn test_re2_dat_0000000120() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(fo|foo))"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(120, regex)
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
fn test_re2_dat_0000000121() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(fo|foo))"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(121, regex)
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
fn test_re2_dat_0000000122() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(fo|foo))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(122, regex)
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
fn test_re2_dat_0000000123() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(fo|foo))$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(123, regex)
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
fn test_re2_dat_0000000124() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|fo)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(124, regex)
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
fn test_re2_dat_0000000125() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|fo)"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(125, regex)
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
fn test_re2_dat_0000000126() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|fo))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(126, regex)
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
fn test_re2_dat_0000000127() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|fo))$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(127, regex)
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
fn test_re2_dat_0000000128() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|fo))"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(128, regex)
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
fn test_re2_dat_0000000129() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|fo))"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(129, regex)
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
fn test_re2_dat_0000000130() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(foo|fo))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(130, regex)
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
fn test_re2_dat_0000000131() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(foo|fo))$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(131, regex)
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
fn test_re2_dat_0000000132() -> Result<(), Error> {
    let regex: &'static str = r##"aa"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(132, regex)
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
fn test_re2_dat_0000000133() -> Result<(), Error> {
    let regex: &'static str = r##"aa"##;
    let text: &'static str = r##"aA"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(133, regex)
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
fn test_re2_dat_0000000134() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:aa)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(134, regex)
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
fn test_re2_dat_0000000135() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:aa)$"##;
    let text: &'static str = r##"aA"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(135, regex)
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
fn test_re2_dat_0000000136() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:aa)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(136, regex)
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
fn test_re2_dat_0000000137() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:aa)"##;
    let text: &'static str = r##"aA"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(137, regex)
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
fn test_re2_dat_0000000138() -> Result<(), Error> {
    let regex: &'static str = r##"(?:aa)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(138, regex)
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
fn test_re2_dat_0000000139() -> Result<(), Error> {
    let regex: &'static str = r##"(?:aa)$"##;
    let text: &'static str = r##"aA"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(139, regex)
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
fn test_re2_dat_0000000140() -> Result<(), Error> {
    let regex: &'static str = r##"a"##;
    let text: &'static str = r##"Aa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(140, regex)
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
fn test_re2_dat_0000000141() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)$"##;
    let text: &'static str = r##"Aa"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(141, regex)
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
fn test_re2_dat_0000000142() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)"##;
    let text: &'static str = r##"Aa"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(142, regex)
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
fn test_re2_dat_0000000143() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a)$"##;
    let text: &'static str = r##"Aa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(143, regex)
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
fn test_re2_dat_0000000144() -> Result<(), Error> {
    let regex: &'static str = r##"a"##;
    let text: &'static str = r##"A"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(144, regex)
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
fn test_re2_dat_0000000145() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)$"##;
    let text: &'static str = r##"A"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(145, regex)
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
fn test_re2_dat_0000000146() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a)"##;
    let text: &'static str = r##"A"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(146, regex)
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
fn test_re2_dat_0000000147() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a)$"##;
    let text: &'static str = r##"A"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(147, regex)
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
fn test_re2_dat_0000000148() -> Result<(), Error> {
    let regex: &'static str = r##"ABC"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(148, regex)
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
fn test_re2_dat_0000000149() -> Result<(), Error> {
    let regex: &'static str = r##"ABC"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(149, regex)
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
fn test_re2_dat_0000000150() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ABC)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(150, regex)
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
fn test_re2_dat_0000000151() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ABC)$"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(151, regex)
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
fn test_re2_dat_0000000152() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ABC)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(152, regex)
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
fn test_re2_dat_0000000153() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ABC)"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(153, regex)
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
fn test_re2_dat_0000000154() -> Result<(), Error> {
    let regex: &'static str = r##"(?:ABC)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(154, regex)
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
fn test_re2_dat_0000000155() -> Result<(), Error> {
    let regex: &'static str = r##"(?:ABC)$"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(155, regex)
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
fn test_re2_dat_0000000156() -> Result<(), Error> {
    let regex: &'static str = r##"abc"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(156, regex)
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
fn test_re2_dat_0000000157() -> Result<(), Error> {
    let regex: &'static str = r##"abc"##;
    let text: &'static str = r##"XABCY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(157, regex)
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
fn test_re2_dat_0000000158() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:abc)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(158, regex)
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
fn test_re2_dat_0000000159() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:abc)$"##;
    let text: &'static str = r##"XABCY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(159, regex)
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
fn test_re2_dat_0000000160() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:abc)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(160, regex)
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
fn test_re2_dat_0000000161() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:abc)"##;
    let text: &'static str = r##"XABCY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(161, regex)
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
fn test_re2_dat_0000000162() -> Result<(), Error> {
    let regex: &'static str = r##"(?:abc)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(162, regex)
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
fn test_re2_dat_0000000163() -> Result<(), Error> {
    let regex: &'static str = r##"(?:abc)$"##;
    let text: &'static str = r##"XABCY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(163, regex)
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
fn test_re2_dat_0000000164() -> Result<(), Error> {
    let regex: &'static str = r##"ABC"##;
    let text: &'static str = r##"xabcy"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(164, regex)
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
fn test_re2_dat_0000000165() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ABC)$"##;
    let text: &'static str = r##"xabcy"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(165, regex)
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
fn test_re2_dat_0000000166() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ABC)"##;
    let text: &'static str = r##"xabcy"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(166, regex)
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
fn test_re2_dat_0000000167() -> Result<(), Error> {
    let regex: &'static str = r##"(?:ABC)$"##;
    let text: &'static str = r##"xabcy"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(167, regex)
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
fn test_re2_dat_0000000168() -> Result<(), Error> {
    let regex: &'static str = r##"foo|bar|[A-Z]"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(168, regex)
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
fn test_re2_dat_0000000169() -> Result<(), Error> {
    let regex: &'static str = r##"foo|bar|[A-Z]"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(169, regex)
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
fn test_re2_dat_0000000170() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:foo|bar|[A-Z])$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(170, regex)
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
fn test_re2_dat_0000000171() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:foo|bar|[A-Z])$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(171, regex)
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
fn test_re2_dat_0000000172() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:foo|bar|[A-Z])"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(172, regex)
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
fn test_re2_dat_0000000173() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:foo|bar|[A-Z])"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(173, regex)
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
fn test_re2_dat_0000000174() -> Result<(), Error> {
    let regex: &'static str = r##"(?:foo|bar|[A-Z])$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(174, regex)
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
fn test_re2_dat_0000000175() -> Result<(), Error> {
    let regex: &'static str = r##"(?:foo|bar|[A-Z])$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(175, regex)
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
fn test_re2_dat_0000000176() -> Result<(), Error> {
    let regex: &'static str = r##"^(foo|bar|[A-Z])"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(176, regex)
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
fn test_re2_dat_0000000177() -> Result<(), Error> {
    let regex: &'static str = r##"^(foo|bar|[A-Z])"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(177, regex)
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
fn test_re2_dat_0000000178() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z]))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(178, regex)
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
fn test_re2_dat_0000000179() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z]))$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(179, regex)
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
fn test_re2_dat_0000000180() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z]))"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(180, regex)
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
fn test_re2_dat_0000000181() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z]))"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(181, regex)
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
fn test_re2_dat_0000000182() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(foo|bar|[A-Z]))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(182, regex)
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
fn test_re2_dat_0000000183() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(foo|bar|[A-Z]))$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(183, regex)
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
fn test_re2_dat_0000000184() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar|[A-Z])$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(184, regex)
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
fn test_re2_dat_0000000185() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(185, regex)
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
fn test_re2_dat_0000000186() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|bar|[A-Z])$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(186, regex)
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
fn test_re2_dat_0000000187() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(187, regex)
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
fn test_re2_dat_0000000188() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar|[A-Z])$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(188, regex)
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
fn test_re2_dat_0000000189() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(189, regex)
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
fn test_re2_dat_0000000190() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|bar|[A-Z])$)"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(190, regex)
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
fn test_re2_dat_0000000191() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(191, regex)
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
fn test_re2_dat_0000000192() -> Result<(), Error> {
    let regex: &'static str = r##"^(foo|bar|[A-Z])$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(192, regex)
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
fn test_re2_dat_0000000193() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(193, regex)
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
fn test_re2_dat_0000000194() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z])$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(194, regex)
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
fn test_re2_dat_0000000195() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(195, regex)
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
fn test_re2_dat_0000000196() -> Result<(), Error> {
    let regex: &'static str = r##"^(foo|bar|[A-Z])$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(196, regex)
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
fn test_re2_dat_0000000197() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(197, regex)
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
fn test_re2_dat_0000000198() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z])$)"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(198, regex)
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
fn test_re2_dat_0000000199() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(199, regex)
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
fn test_re2_dat_0000000200() -> Result<(), Error> {
    let regex: &'static str = r##"^(foo|bar|[A-Z])$"##;
    let text: &'static str = r##"bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(200, regex)
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
fn test_re2_dat_0000000201() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##"bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(201, regex)
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
fn test_re2_dat_0000000202() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z])$)"##;
    let text: &'static str = r##"bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(202, regex)
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
fn test_re2_dat_0000000203() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##"bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(203, regex)
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
fn test_re2_dat_0000000204() -> Result<(), Error> {
    let regex: &'static str = r##"^(foo|bar|[A-Z])$"##;
    let text: &'static str = r##"X"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(204, regex)
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
fn test_re2_dat_0000000205() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##"X"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(205, regex)
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
fn test_re2_dat_0000000206() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z])$)"##;
    let text: &'static str = r##"X"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(206, regex)
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
fn test_re2_dat_0000000207() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##"X"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(207, regex)
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
fn test_re2_dat_0000000208() -> Result<(), Error> {
    let regex: &'static str = r##"^(foo|bar|[A-Z])$"##;
    let text: &'static str = r##"XY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(208, regex)
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
fn test_re2_dat_0000000209() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##"XY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(209, regex)
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
fn test_re2_dat_0000000210() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo|bar|[A-Z])$)"##;
    let text: &'static str = r##"XY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(210, regex)
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
fn test_re2_dat_0000000211() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(foo|bar|[A-Z])$)$"##;
    let text: &'static str = r##"XY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(211, regex)
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
fn test_re2_dat_0000000212() -> Result<(), Error> {
    let regex: &'static str = r##"^(fo|foo)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(212, regex)
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
fn test_re2_dat_0000000213() -> Result<(), Error> {
    let regex: &'static str = r##"^(fo|foo)$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(213, regex)
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
fn test_re2_dat_0000000214() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(fo|foo)$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(214, regex)
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
fn test_re2_dat_0000000215() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(fo|foo)$)$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(215, regex)
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
fn test_re2_dat_0000000216() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(fo|foo)$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(216, regex)
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
fn test_re2_dat_0000000217() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(fo|foo)$)"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(217, regex)
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
fn test_re2_dat_0000000218() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(fo|foo)$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(218, regex)
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
fn test_re2_dat_0000000219() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(fo|foo)$)$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(219, regex)
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
fn test_re2_dat_0000000220() -> Result<(), Error> {
    let regex: &'static str = r##"^(fo|foo)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(220, regex)
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
fn test_re2_dat_0000000221() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(fo|foo)$)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(221, regex)
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
fn test_re2_dat_0000000222() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(fo|foo)$)"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(222, regex)
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
fn test_re2_dat_0000000223() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(fo|foo)$)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(223, regex)
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
fn test_re2_dat_0000000224() -> Result<(), Error> {
    let regex: &'static str = r##"^^(fo|foo)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(224, regex)
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
fn test_re2_dat_0000000225() -> Result<(), Error> {
    let regex: &'static str = r##"^^(fo|foo)$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(225, regex)
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
fn test_re2_dat_0000000226() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^(fo|foo)$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(226, regex)
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
fn test_re2_dat_0000000227() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^(fo|foo)$)$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(227, regex)
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
fn test_re2_dat_0000000228() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^(fo|foo)$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(228, regex)
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
fn test_re2_dat_0000000229() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^(fo|foo)$)"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(229, regex)
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
fn test_re2_dat_0000000230() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^(fo|foo)$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(230, regex)
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
fn test_re2_dat_0000000231() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^(fo|foo)$)$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(231, regex)
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
fn test_re2_dat_0000000232() -> Result<(), Error> {
    let regex: &'static str = r##"^^(fo|foo)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(232, regex)
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
fn test_re2_dat_0000000233() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^(fo|foo)$)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(233, regex)
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
fn test_re2_dat_0000000234() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^(fo|foo)$)"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(234, regex)
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
fn test_re2_dat_0000000235() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^(fo|foo)$)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(235, regex)
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
fn test_re2_dat_0000000236() -> Result<(), Error> {
    let regex: &'static str = r##"^$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(236, regex)
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
fn test_re2_dat_0000000237() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(237, regex)
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
fn test_re2_dat_0000000238() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(238, regex)
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
fn test_re2_dat_0000000239() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(239, regex)
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
fn test_re2_dat_0000000240() -> Result<(), Error> {
    let regex: &'static str = r##"^$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(240, regex)
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
fn test_re2_dat_0000000241() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(241, regex)
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
fn test_re2_dat_0000000242() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(242, regex)
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
fn test_re2_dat_0000000243() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(243, regex)
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
fn test_re2_dat_0000000244() -> Result<(), Error> {
    let regex: &'static str = r##"^^$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(244, regex)
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
fn test_re2_dat_0000000245() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(245, regex)
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
fn test_re2_dat_0000000246() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(246, regex)
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
fn test_re2_dat_0000000247() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(247, regex)
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
fn test_re2_dat_0000000248() -> Result<(), Error> {
    let regex: &'static str = r##"^$$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(248, regex)
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
fn test_re2_dat_0000000249() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(249, regex)
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
fn test_re2_dat_0000000250() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(250, regex)
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
fn test_re2_dat_0000000251() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^$$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(251, regex)
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
fn test_re2_dat_0000000252() -> Result<(), Error> {
    let regex: &'static str = r##"^^$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(252, regex)
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
fn test_re2_dat_0000000253() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(253, regex)
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
fn test_re2_dat_0000000254() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(254, regex)
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
fn test_re2_dat_0000000255() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(255, regex)
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
fn test_re2_dat_0000000256() -> Result<(), Error> {
    let regex: &'static str = r##"^$$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(256, regex)
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
fn test_re2_dat_0000000257() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(257, regex)
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
fn test_re2_dat_0000000258() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(258, regex)
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
fn test_re2_dat_0000000259() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^$$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(259, regex)
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
fn test_re2_dat_0000000260() -> Result<(), Error> {
    let regex: &'static str = r##"^^$$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(260, regex)
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
fn test_re2_dat_0000000261() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^$$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(261, regex)
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
fn test_re2_dat_0000000262() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^$$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(262, regex)
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
fn test_re2_dat_0000000263() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^$$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(263, regex)
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
fn test_re2_dat_0000000264() -> Result<(), Error> {
    let regex: &'static str = r##"^^$$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(264, regex)
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
fn test_re2_dat_0000000265() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^$$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(265, regex)
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
fn test_re2_dat_0000000266() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^$$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(266, regex)
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
fn test_re2_dat_0000000267() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^$$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(267, regex)
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
fn test_re2_dat_0000000268() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^^^^$$$$$$$$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(268, regex)
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
fn test_re2_dat_0000000269() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^^^^^^^$$$$$$$$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(269, regex)
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
fn test_re2_dat_0000000270() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^^^^^^^$$$$$$$$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(270, regex)
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
fn test_re2_dat_0000000271() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^^^^^^^$$$$$$$$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(271, regex)
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
fn test_re2_dat_0000000272() -> Result<(), Error> {
    let regex: &'static str = r##"^"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(272, regex)
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
fn test_re2_dat_0000000273() -> Result<(), Error> {
    let regex: &'static str = r##"^"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(273, regex)
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
fn test_re2_dat_0000000274() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(274, regex)
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
fn test_re2_dat_0000000275() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(275, regex)
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
fn test_re2_dat_0000000276() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(276, regex)
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
fn test_re2_dat_0000000277() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(277, regex)
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
fn test_re2_dat_0000000278() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(278, regex)
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
fn test_re2_dat_0000000279() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(279, regex)
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
fn test_re2_dat_0000000280() -> Result<(), Error> {
    let regex: &'static str = r##"$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(280, regex)
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
fn test_re2_dat_0000000281() -> Result<(), Error> {
    let regex: &'static str = r##"$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(281, regex)
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
fn test_re2_dat_0000000282() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(282, regex)
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
fn test_re2_dat_0000000283() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(283, regex)
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
fn test_re2_dat_0000000284() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(284, regex)
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
fn test_re2_dat_0000000285() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(285, regex)
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
fn test_re2_dat_0000000286() -> Result<(), Error> {
    let regex: &'static str = r##"(?:$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(286, regex)
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
fn test_re2_dat_0000000287() -> Result<(), Error> {
    let regex: &'static str = r##"(?:$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(287, regex)
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
fn test_re2_dat_0000000288() -> Result<(), Error> {
    let regex: &'static str = r##"\bfoo\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(288, regex)
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
fn test_re2_dat_0000000289() -> Result<(), Error> {
    let regex: &'static str = r##"\bfoo\b"##;
    let text: &'static str = r##"nofoo foo that"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(6), Some(9))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(289, regex)
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
fn test_re2_dat_0000000290() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bfoo\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(290, regex)
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
fn test_re2_dat_0000000291() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bfoo\b)$"##;
    let text: &'static str = r##"nofoo foo that"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(291, regex)
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
fn test_re2_dat_0000000292() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bfoo\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(292, regex)
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
fn test_re2_dat_0000000293() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bfoo\b)"##;
    let text: &'static str = r##"nofoo foo that"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(293, regex)
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
fn test_re2_dat_0000000294() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bfoo\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(294, regex)
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
fn test_re2_dat_0000000295() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bfoo\b)$"##;
    let text: &'static str = r##"nofoo foo that"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(295, regex)
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
fn test_re2_dat_0000000296() -> Result<(), Error> {
    let regex: &'static str = r##"a\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(296, regex)
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
fn test_re2_dat_0000000297() -> Result<(), Error> {
    let regex: &'static str = r##"a\b"##;
    let text: &'static str = r##"faoa x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(297, regex)
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
fn test_re2_dat_0000000298() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(298, regex)
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
fn test_re2_dat_0000000299() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a\b)$"##;
    let text: &'static str = r##"faoa x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(299, regex)
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
fn test_re2_dat_0000000300() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(300, regex)
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
fn test_re2_dat_0000000301() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:a\b)"##;
    let text: &'static str = r##"faoa x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(301, regex)
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
fn test_re2_dat_0000000302() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(302, regex)
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
fn test_re2_dat_0000000303() -> Result<(), Error> {
    let regex: &'static str = r##"(?:a\b)$"##;
    let text: &'static str = r##"faoa x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(303, regex)
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
fn test_re2_dat_0000000304() -> Result<(), Error> {
    let regex: &'static str = r##"\bbar"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(304, regex)
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
fn test_re2_dat_0000000305() -> Result<(), Error> {
    let regex: &'static str = r##"\bbar"##;
    let text: &'static str = r##"bar x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(305, regex)
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
fn test_re2_dat_0000000306() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bbar)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(306, regex)
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
fn test_re2_dat_0000000307() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bbar)$"##;
    let text: &'static str = r##"bar x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(307, regex)
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
fn test_re2_dat_0000000308() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bbar)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(308, regex)
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
fn test_re2_dat_0000000309() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bbar)"##;
    let text: &'static str = r##"bar x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(309, regex)
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
fn test_re2_dat_0000000310() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bbar)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(310, regex)
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
fn test_re2_dat_0000000311() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bbar)$"##;
    let text: &'static str = r##"bar x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(311, regex)
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
fn test_re2_dat_0000000312() -> Result<(), Error> {
    let regex: &'static str = r##"bar\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(312, regex)
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
fn test_re2_dat_0000000313() -> Result<(), Error> {
    let regex: &'static str = r##"bar\b"##;
    let text: &'static str = r##"foobar"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(313, regex)
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
fn test_re2_dat_0000000314() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:bar\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(314, regex)
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
fn test_re2_dat_0000000315() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:bar\b)$"##;
    let text: &'static str = r##"foobar"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(315, regex)
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
fn test_re2_dat_0000000316() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:bar\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(316, regex)
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
fn test_re2_dat_0000000317() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:bar\b)"##;
    let text: &'static str = r##"foobar"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(317, regex)
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
fn test_re2_dat_0000000318() -> Result<(), Error> {
    let regex: &'static str = r##"(?:bar\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(318, regex)
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
fn test_re2_dat_0000000319() -> Result<(), Error> {
    let regex: &'static str = r##"(?:bar\b)$"##;
    let text: &'static str = r##"foobar"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(319, regex)
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
fn test_re2_dat_0000000320() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(320, regex)
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
fn test_re2_dat_0000000321() -> Result<(), Error> {
    let regex: &'static str = r##"(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(321, regex)
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
fn test_re2_dat_0000000322() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(322, regex)
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
fn test_re2_dat_0000000323() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(323, regex)
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
fn test_re2_dat_0000000324() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|bar|[A-Z])\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(324, regex)
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
fn test_re2_dat_0000000325() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo|bar|[A-Z])\b)"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(325, regex)
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
fn test_re2_dat_0000000326() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(326, regex)
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
fn test_re2_dat_0000000327() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(327, regex)
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
fn test_re2_dat_0000000328() -> Result<(), Error> {
    let regex: &'static str = r##"\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(328, regex)
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
fn test_re2_dat_0000000329() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(329, regex)
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
fn test_re2_dat_0000000330() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(330, regex)
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
fn test_re2_dat_0000000331() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(331, regex)
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
fn test_re2_dat_0000000332() -> Result<(), Error> {
    let regex: &'static str = r##"\b"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(332, regex)
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
fn test_re2_dat_0000000333() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(333, regex)
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
fn test_re2_dat_0000000334() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(334, regex)
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
fn test_re2_dat_0000000335() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(335, regex)
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
fn test_re2_dat_0000000336() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(336, regex)
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
fn test_re2_dat_0000000337() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(337, regex)
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
fn test_re2_dat_0000000338() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z]))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(338, regex)
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
fn test_re2_dat_0000000339() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z]))$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(339, regex)
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
fn test_re2_dat_0000000340() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z]))"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(340, regex)
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
fn test_re2_dat_0000000341() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z]))"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(341, regex)
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
fn test_re2_dat_0000000342() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(foo|bar|[A-Z]))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(342, regex)
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
fn test_re2_dat_0000000343() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(foo|bar|[A-Z]))$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(343, regex)
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
fn test_re2_dat_0000000344() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(344, regex)
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
fn test_re2_dat_0000000345() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"X"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(345, regex)
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
fn test_re2_dat_0000000346() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(346, regex)
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
fn test_re2_dat_0000000347() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"X"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(347, regex)
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
fn test_re2_dat_0000000348() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(348, regex)
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
fn test_re2_dat_0000000349() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)"##;
    let text: &'static str = r##"X"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(349, regex)
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
fn test_re2_dat_0000000350() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(350, regex)
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
fn test_re2_dat_0000000351() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"X"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(351, regex)
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
fn test_re2_dat_0000000352() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"XY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(352, regex)
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
fn test_re2_dat_0000000353() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"XY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(353, regex)
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
fn test_re2_dat_0000000354() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)"##;
    let text: &'static str = r##"XY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(354, regex)
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
fn test_re2_dat_0000000355() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"XY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(355, regex)
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
fn test_re2_dat_0000000356() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(356, regex)
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
fn test_re2_dat_0000000357() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(357, regex)
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
fn test_re2_dat_0000000358() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)"##;
    let text: &'static str = r##"bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(358, regex)
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
fn test_re2_dat_0000000359() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(359, regex)
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
fn test_re2_dat_0000000360() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(360, regex)
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
fn test_re2_dat_0000000361() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(361, regex)
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
fn test_re2_dat_0000000362() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(362, regex)
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
fn test_re2_dat_0000000363() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(363, regex)
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
fn test_re2_dat_0000000364() -> Result<(), Error> {
    let regex: &'static str = r##"\b(foo|bar|[A-Z])\b"##;
    let text: &'static str = r##"ffoo bbar N x"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(10), Some(11)),
        Capture::new(Some(10), Some(11)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(364, regex)
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
fn test_re2_dat_0000000365() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"ffoo bbar N x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(365, regex)
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
fn test_re2_dat_0000000366() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(foo|bar|[A-Z])\b)"##;
    let text: &'static str = r##"ffoo bbar N x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(366, regex)
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
fn test_re2_dat_0000000367() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(foo|bar|[A-Z])\b)$"##;
    let text: &'static str = r##"ffoo bbar N x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(367, regex)
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
fn test_re2_dat_0000000368() -> Result<(), Error> {
    let regex: &'static str = r##"\b(fo|foo)\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(368, regex)
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
fn test_re2_dat_0000000369() -> Result<(), Error> {
    let regex: &'static str = r##"\b(fo|foo)\b"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(369, regex)
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
fn test_re2_dat_0000000370() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(fo|foo)\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(370, regex)
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
fn test_re2_dat_0000000371() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(fo|foo)\b)$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(371, regex)
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
fn test_re2_dat_0000000372() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(fo|foo)\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(372, regex)
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
fn test_re2_dat_0000000373() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(fo|foo)\b)"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(373, regex)
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
fn test_re2_dat_0000000374() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(fo|foo)\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(374, regex)
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
fn test_re2_dat_0000000375() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(fo|foo)\b)$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(375, regex)
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
fn test_re2_dat_0000000376() -> Result<(), Error> {
    let regex: &'static str = r##"\b(fo|foo)\b"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(376, regex)
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
fn test_re2_dat_0000000377() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(fo|foo)\b)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(377, regex)
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
fn test_re2_dat_0000000378() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b(fo|foo)\b)"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(378, regex)
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
fn test_re2_dat_0000000379() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b(fo|foo)\b)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(379, regex)
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
fn test_re2_dat_0000000380() -> Result<(), Error> {
    let regex: &'static str = r##"\b\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(380, regex)
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
fn test_re2_dat_0000000381() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(381, regex)
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
fn test_re2_dat_0000000382() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(382, regex)
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
fn test_re2_dat_0000000383() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(383, regex)
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
fn test_re2_dat_0000000384() -> Result<(), Error> {
    let regex: &'static str = r##"\b\b"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(384, regex)
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
fn test_re2_dat_0000000385() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b\b)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(385, regex)
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
fn test_re2_dat_0000000386() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b\b)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(386, regex)
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
fn test_re2_dat_0000000387() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b\b)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(387, regex)
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
fn test_re2_dat_0000000388() -> Result<(), Error> {
    let regex: &'static str = r##"\b$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(388, regex)
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
fn test_re2_dat_0000000389() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(389, regex)
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
fn test_re2_dat_0000000390() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(390, regex)
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
fn test_re2_dat_0000000391() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(391, regex)
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
fn test_re2_dat_0000000392() -> Result<(), Error> {
    let regex: &'static str = r##"\b$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(392, regex)
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
fn test_re2_dat_0000000393() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(393, regex)
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
fn test_re2_dat_0000000394() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(394, regex)
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
fn test_re2_dat_0000000395() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(395, regex)
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
fn test_re2_dat_0000000396() -> Result<(), Error> {
    let regex: &'static str = r##"\b$"##;
    let text: &'static str = r##"y x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(396, regex)
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
fn test_re2_dat_0000000397() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b$)$"##;
    let text: &'static str = r##"y x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(397, regex)
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
fn test_re2_dat_0000000398() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b$)"##;
    let text: &'static str = r##"y x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(398, regex)
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
fn test_re2_dat_0000000399() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b$)$"##;
    let text: &'static str = r##"y x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(399, regex)
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
fn test_re2_dat_0000000400() -> Result<(), Error> {
    let regex: &'static str = r##"\b.$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(400, regex)
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
fn test_re2_dat_0000000401() -> Result<(), Error> {
    let regex: &'static str = r##"\b.$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(401, regex)
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
fn test_re2_dat_0000000402() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b.$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(402, regex)
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
fn test_re2_dat_0000000403() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b.$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(403, regex)
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
fn test_re2_dat_0000000404() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b.$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(404, regex)
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
fn test_re2_dat_0000000405() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\b.$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(405, regex)
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
fn test_re2_dat_0000000406() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b.$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(406, regex)
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
fn test_re2_dat_0000000407() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\b.$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(407, regex)
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
fn test_re2_dat_0000000408() -> Result<(), Error> {
    let regex: &'static str = r##"^\b(fo|foo)\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(408, regex)
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
fn test_re2_dat_0000000409() -> Result<(), Error> {
    let regex: &'static str = r##"^\b(fo|foo)\b"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(409, regex)
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
fn test_re2_dat_0000000410() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b(fo|foo)\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(410, regex)
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
fn test_re2_dat_0000000411() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b(fo|foo)\b)$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(411, regex)
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
fn test_re2_dat_0000000412() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b(fo|foo)\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(412, regex)
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
fn test_re2_dat_0000000413() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b(fo|foo)\b)"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(413, regex)
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
fn test_re2_dat_0000000414() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b(fo|foo)\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(414, regex)
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
fn test_re2_dat_0000000415() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b(fo|foo)\b)$"##;
    let text: &'static str = r##"fo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(415, regex)
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
fn test_re2_dat_0000000416() -> Result<(), Error> {
    let regex: &'static str = r##"^\b(fo|foo)\b"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(416, regex)
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
fn test_re2_dat_0000000417() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b(fo|foo)\b)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(417, regex)
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
fn test_re2_dat_0000000418() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b(fo|foo)\b)"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(418, regex)
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
fn test_re2_dat_0000000419() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b(fo|foo)\b)$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(419, regex)
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
fn test_re2_dat_0000000420() -> Result<(), Error> {
    let regex: &'static str = r##"^\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(420, regex)
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
fn test_re2_dat_0000000421() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(421, regex)
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
fn test_re2_dat_0000000422() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(422, regex)
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
fn test_re2_dat_0000000423() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(423, regex)
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
fn test_re2_dat_0000000424() -> Result<(), Error> {
    let regex: &'static str = r##"^\b"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(424, regex)
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
fn test_re2_dat_0000000425() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(425, regex)
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
fn test_re2_dat_0000000426() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(426, regex)
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
fn test_re2_dat_0000000427() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(427, regex)
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
fn test_re2_dat_0000000428() -> Result<(), Error> {
    let regex: &'static str = r##"^\b\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(428, regex)
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
fn test_re2_dat_0000000429() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(429, regex)
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
fn test_re2_dat_0000000430() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(430, regex)
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
fn test_re2_dat_0000000431() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(431, regex)
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
fn test_re2_dat_0000000432() -> Result<(), Error> {
    let regex: &'static str = r##"^\b\b"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(432, regex)
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
fn test_re2_dat_0000000433() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b\b)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(433, regex)
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
fn test_re2_dat_0000000434() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b\b)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(434, regex)
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
fn test_re2_dat_0000000435() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b\b)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(435, regex)
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
fn test_re2_dat_0000000436() -> Result<(), Error> {
    let regex: &'static str = r##"^\b$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(436, regex)
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
fn test_re2_dat_0000000437() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(437, regex)
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
fn test_re2_dat_0000000438() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(438, regex)
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
fn test_re2_dat_0000000439() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(439, regex)
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
fn test_re2_dat_0000000440() -> Result<(), Error> {
    let regex: &'static str = r##"^\b$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(440, regex)
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
fn test_re2_dat_0000000441() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(441, regex)
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
fn test_re2_dat_0000000442() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(442, regex)
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
fn test_re2_dat_0000000443() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(443, regex)
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
fn test_re2_dat_0000000444() -> Result<(), Error> {
    let regex: &'static str = r##"^\b.$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(444, regex)
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
fn test_re2_dat_0000000445() -> Result<(), Error> {
    let regex: &'static str = r##"^\b.$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(445, regex)
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
fn test_re2_dat_0000000446() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b.$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(446, regex)
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
fn test_re2_dat_0000000447() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b.$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(447, regex)
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
fn test_re2_dat_0000000448() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b.$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(448, regex)
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
fn test_re2_dat_0000000449() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b.$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(449, regex)
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
fn test_re2_dat_0000000450() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b.$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(450, regex)
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
fn test_re2_dat_0000000451() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b.$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(451, regex)
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
fn test_re2_dat_0000000452() -> Result<(), Error> {
    let regex: &'static str = r##"^\b.\b$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(452, regex)
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
fn test_re2_dat_0000000453() -> Result<(), Error> {
    let regex: &'static str = r##"^\b.\b$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(453, regex)
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
fn test_re2_dat_0000000454() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b.\b$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(454, regex)
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
fn test_re2_dat_0000000455() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b.\b$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(455, regex)
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
fn test_re2_dat_0000000456() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b.\b$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(456, regex)
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
fn test_re2_dat_0000000457() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^\b.\b$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(457, regex)
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
fn test_re2_dat_0000000458() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b.\b$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(458, regex)
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
fn test_re2_dat_0000000459() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^\b.\b$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(459, regex)
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
fn test_re2_dat_0000000460() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^^^^\b$$$$$$$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(460, regex)
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
fn test_re2_dat_0000000461() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^^^^^^^\b$$$$$$$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(461, regex)
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
fn test_re2_dat_0000000462() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^^^^^^^\b$$$$$$$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(462, regex)
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
fn test_re2_dat_0000000463() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^^^^^^^\b$$$$$$$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(463, regex)
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
fn test_re2_dat_0000000464() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^^^^\b.$$$$$$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(464, regex)
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
fn test_re2_dat_0000000465() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^^^^\b.$$$$$$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(465, regex)
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
fn test_re2_dat_0000000466() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^^^^^^^\b.$$$$$$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(466, regex)
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
fn test_re2_dat_0000000467() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^^^^^^^\b.$$$$$$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(467, regex)
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
fn test_re2_dat_0000000468() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^^^^^^^\b.$$$$$$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(468, regex)
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
fn test_re2_dat_0000000469() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^^^^^^^\b.$$$$$$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(469, regex)
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
fn test_re2_dat_0000000470() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^^^^^^^\b.$$$$$$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(470, regex)
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
fn test_re2_dat_0000000471() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^^^^^^^\b.$$$$$$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(471, regex)
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
fn test_re2_dat_0000000472() -> Result<(), Error> {
    let regex: &'static str = r##"^^^^^^^^\b$$$$$$$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(472, regex)
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
fn test_re2_dat_0000000473() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^^^^^^^\b$$$$$$$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(473, regex)
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
fn test_re2_dat_0000000474() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^^^^^^^\b$$$$$$$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(474, regex)
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
fn test_re2_dat_0000000475() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^^^^^^^\b$$$$$$$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(475, regex)
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
fn test_re2_dat_0000000476() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(476, regex)
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
fn test_re2_dat_0000000477() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(477, regex)
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
fn test_re2_dat_0000000478() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(478, regex)
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
fn test_re2_dat_0000000479() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(479, regex)
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
fn test_re2_dat_0000000480() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(480, regex)
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
fn test_re2_dat_0000000481() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(481, regex)
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
fn test_re2_dat_0000000482() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(482, regex)
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
fn test_re2_dat_0000000483() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(483, regex)
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
fn test_re2_dat_0000000484() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"x>"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(484, regex)
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
fn test_re2_dat_0000000485() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"x>"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(485, regex)
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
fn test_re2_dat_0000000486() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"x>"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(486, regex)
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
fn test_re2_dat_0000000487() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"x>"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(487, regex)
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
fn test_re2_dat_0000000488() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"<x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(488, regex)
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
fn test_re2_dat_0000000489() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"<x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(489, regex)
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
fn test_re2_dat_0000000490() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"<x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(490, regex)
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
fn test_re2_dat_0000000491() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"<x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(491, regex)
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
fn test_re2_dat_0000000492() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"<x>"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(492, regex)
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
fn test_re2_dat_0000000493() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"<x>"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(493, regex)
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
fn test_re2_dat_0000000494() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"<x>"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(494, regex)
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
fn test_re2_dat_0000000495() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"<x>"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(495, regex)
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
fn test_re2_dat_0000000496() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"ax"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(496, regex)
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
fn test_re2_dat_0000000497() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"ax"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(497, regex)
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
fn test_re2_dat_0000000498() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"ax"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(498, regex)
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
fn test_re2_dat_0000000499() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"ax"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(499, regex)
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
fn test_re2_dat_0000000500() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"xb"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(500, regex)
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
fn test_re2_dat_0000000501() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"xb"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(501, regex)
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
fn test_re2_dat_0000000502() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"xb"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(502, regex)
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
fn test_re2_dat_0000000503() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"xb"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(503, regex)
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
fn test_re2_dat_0000000504() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"axb"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(504, regex)
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
fn test_re2_dat_0000000505() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"axb"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(505, regex)
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
fn test_re2_dat_0000000506() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"axb"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(506, regex)
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
fn test_re2_dat_0000000507() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"axb"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(507, regex)
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
fn test_re2_dat_0000000508() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"«x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(508, regex)
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
fn test_re2_dat_0000000509() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"«x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(509, regex)
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
fn test_re2_dat_0000000510() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"«x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(510, regex)
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
fn test_re2_dat_0000000511() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"«x"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(511, regex)
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
fn test_re2_dat_0000000512() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"x»"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(512, regex)
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
fn test_re2_dat_0000000513() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"x»"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(513, regex)
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
fn test_re2_dat_0000000514() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"x»"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(514, regex)
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
fn test_re2_dat_0000000515() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"x»"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(515, regex)
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
fn test_re2_dat_0000000516() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"«x»"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(516, regex)
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
fn test_re2_dat_0000000517() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"«x»"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(517, regex)
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
fn test_re2_dat_0000000518() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"«x»"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(518, regex)
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
fn test_re2_dat_0000000519() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"«x»"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(519, regex)
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
fn test_re2_dat_0000000520() -> Result<(), Error> {
    let regex: &'static str = r##"\bx\b"##;
    let text: &'static str = r##"áxβ"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(2), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(520, regex)
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
fn test_re2_dat_0000000521() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)$"##;
    let text: &'static str = r##"áxβ"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(521, regex)
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
fn test_re2_dat_0000000522() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\bx\b)"##;
    let text: &'static str = r##"áxβ"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(522, regex)
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
fn test_re2_dat_0000000523() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\bx\b)$"##;
    let text: &'static str = r##"áxβ"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(523, regex)
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
fn test_re2_dat_0000000524() -> Result<(), Error> {
    let regex: &'static str = r##"^$^$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(524, regex)
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
fn test_re2_dat_0000000525() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$^$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(525, regex)
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
fn test_re2_dat_0000000526() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$^$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(526, regex)
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
fn test_re2_dat_0000000527() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^$^$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(527, regex)
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
fn test_re2_dat_0000000528() -> Result<(), Error> {
    let regex: &'static str = r##"^$^"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(528, regex)
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
fn test_re2_dat_0000000529() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$^)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(529, regex)
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
fn test_re2_dat_0000000530() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$^)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(530, regex)
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
fn test_re2_dat_0000000531() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^$^)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(531, regex)
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
fn test_re2_dat_0000000532() -> Result<(), Error> {
    let regex: &'static str = r##"$^$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(532, regex)
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
fn test_re2_dat_0000000533() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:$^$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(533, regex)
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
fn test_re2_dat_0000000534() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:$^$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(534, regex)
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
fn test_re2_dat_0000000535() -> Result<(), Error> {
    let regex: &'static str = r##"(?:$^$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(535, regex)
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
fn test_re2_dat_0000000536() -> Result<(), Error> {
    let regex: &'static str = r##"^$^$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(536, regex)
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
fn test_re2_dat_0000000537() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$^$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(537, regex)
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
fn test_re2_dat_0000000538() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$^$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(538, regex)
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
fn test_re2_dat_0000000539() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^$^$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(539, regex)
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
fn test_re2_dat_0000000540() -> Result<(), Error> {
    let regex: &'static str = r##"^$^"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(540, regex)
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
fn test_re2_dat_0000000541() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$^)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(541, regex)
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
fn test_re2_dat_0000000542() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^$^)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(542, regex)
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
fn test_re2_dat_0000000543() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^$^)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(543, regex)
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
fn test_re2_dat_0000000544() -> Result<(), Error> {
    let regex: &'static str = r##"$^$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(544, regex)
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
fn test_re2_dat_0000000545() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:$^$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(545, regex)
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
fn test_re2_dat_0000000546() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:$^$)"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(546, regex)
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
fn test_re2_dat_0000000547() -> Result<(), Error> {
    let regex: &'static str = r##"(?:$^$)$"##;
    let text: &'static str = r##"x"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(547, regex)
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
fn test_re2_dat_0000000548() -> Result<(), Error> {
    let regex: &'static str = r##"^(foo\$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(548, regex)
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
fn test_re2_dat_0000000549() -> Result<(), Error> {
    let regex: &'static str = r##"^(foo\$)$"##;
    let text: &'static str = r##"foo$bar"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(549, regex)
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
fn test_re2_dat_0000000550() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo\$)$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(550, regex)
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
fn test_re2_dat_0000000551() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo\$)$)$"##;
    let text: &'static str = r##"foo$bar"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(551, regex)
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
fn test_re2_dat_0000000552() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo\$)$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(552, regex)
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
fn test_re2_dat_0000000553() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^(foo\$)$)"##;
    let text: &'static str = r##"foo$bar"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(553, regex)
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
fn test_re2_dat_0000000554() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(foo\$)$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(554, regex)
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
fn test_re2_dat_0000000555() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^(foo\$)$)$"##;
    let text: &'static str = r##"foo$bar"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(555, regex)
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
fn test_re2_dat_0000000556() -> Result<(), Error> {
    let regex: &'static str = r##"(foo\$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(556, regex)
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
fn test_re2_dat_0000000557() -> Result<(), Error> {
    let regex: &'static str = r##"(foo\$)"##;
    let text: &'static str = r##"foo$bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(557, regex)
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
fn test_re2_dat_0000000558() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo\$))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(558, regex)
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
fn test_re2_dat_0000000559() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo\$))$"##;
    let text: &'static str = r##"foo$bar"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(559, regex)
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
fn test_re2_dat_0000000560() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo\$))"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(560, regex)
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
fn test_re2_dat_0000000561() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(foo\$))"##;
    let text: &'static str = r##"foo$bar"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(561, regex)
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
fn test_re2_dat_0000000562() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(foo\$))$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(562, regex)
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
fn test_re2_dat_0000000563() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(foo\$))$"##;
    let text: &'static str = r##"foo$bar"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(563, regex)
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
fn test_re2_dat_0000000564() -> Result<(), Error> {
    let regex: &'static str = r##"^...$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(564, regex)
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
fn test_re2_dat_0000000565() -> Result<(), Error> {
    let regex: &'static str = r##"^...$"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(565, regex)
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
fn test_re2_dat_0000000566() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^...$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(566, regex)
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
fn test_re2_dat_0000000567() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^...$)$"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(567, regex)
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
fn test_re2_dat_0000000568() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^...$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(568, regex)
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
fn test_re2_dat_0000000569() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^...$)"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(569, regex)
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
fn test_re2_dat_0000000570() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^...$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(570, regex)
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
fn test_re2_dat_0000000571() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^...$)$"##;
    let text: &'static str = r##"abc"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(571, regex)
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
fn test_re2_dat_0000000572() -> Result<(), Error> {
    let regex: &'static str = r##"^.........$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(572, regex)
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
fn test_re2_dat_0000000573() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^.........$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(573, regex)
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
fn test_re2_dat_0000000574() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^.........$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(574, regex)
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
fn test_re2_dat_0000000575() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^.........$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(575, regex)
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
fn test_re2_dat_0000000576() -> Result<(), Error> {
    let regex: &'static str = r##"^.....$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(576, regex)
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
fn test_re2_dat_0000000577() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^.....$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(577, regex)
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
fn test_re2_dat_0000000578() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^.....$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(578, regex)
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
fn test_re2_dat_0000000579() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^.....$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(579, regex)
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
fn test_re2_dat_0000000580() -> Result<(), Error> {
    let regex: &'static str = r##"(fo|foo)"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(580, regex)
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
fn test_re2_dat_0000000581() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(fo|foo))$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(581, regex)
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
fn test_re2_dat_0000000582() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(fo|foo))"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(582, regex)
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
fn test_re2_dat_0000000583() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(fo|foo))$"##;
    let text: &'static str = r##"foo"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(583, regex)
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
fn test_re2_dat_0000000584() -> Result<(), Error> {
    let regex: &'static str = r##"\x{61}"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(584, regex)
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
fn test_re2_dat_0000000585() -> Result<(), Error> {
    let regex: &'static str = r##"\x{61}"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(585, regex)
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
fn test_re2_dat_0000000586() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x{61})$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(586, regex)
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
fn test_re2_dat_0000000587() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x{61})$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(587, regex)
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
fn test_re2_dat_0000000588() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x{61})"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(588, regex)
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
fn test_re2_dat_0000000589() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x{61})"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(589, regex)
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
fn test_re2_dat_0000000590() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\x{61})$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(590, regex)
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
fn test_re2_dat_0000000591() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\x{61})$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(591, regex)
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
fn test_re2_dat_0000000592() -> Result<(), Error> {
    let regex: &'static str = r##"\x61"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(592, regex)
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
fn test_re2_dat_0000000593() -> Result<(), Error> {
    let regex: &'static str = r##"\x61"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(593, regex)
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
fn test_re2_dat_0000000594() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x61)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(594, regex)
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
fn test_re2_dat_0000000595() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x61)$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(595, regex)
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
fn test_re2_dat_0000000596() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x61)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(596, regex)
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
fn test_re2_dat_0000000597() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x61)"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(597, regex)
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
fn test_re2_dat_0000000598() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\x61)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(598, regex)
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
fn test_re2_dat_0000000599() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\x61)$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(599, regex)
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
fn test_re2_dat_0000000600() -> Result<(), Error> {
    let regex: &'static str = r##"\x{00000061}"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(600, regex)
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
fn test_re2_dat_0000000601() -> Result<(), Error> {
    let regex: &'static str = r##"\x{00000061}"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(601, regex)
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
fn test_re2_dat_0000000602() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x{00000061})$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(602, regex)
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
fn test_re2_dat_0000000603() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x{00000061})$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(603, regex)
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
fn test_re2_dat_0000000604() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x{00000061})"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(604, regex)
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
fn test_re2_dat_0000000605() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\x{00000061})"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(605, regex)
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
fn test_re2_dat_0000000606() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\x{00000061})$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(606, regex)
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
fn test_re2_dat_0000000607() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\x{00000061})$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(607, regex)
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
fn test_re2_dat_0000000608() -> Result<(), Error> {
    let regex: &'static str = r##"[^0-9]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(608, regex)
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
fn test_re2_dat_0000000609() -> Result<(), Error> {
    let regex: &'static str = r##"[^0-9]+"##;
    let text: &'static str = r##"abc123"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(609, regex)
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
fn test_re2_dat_0000000610() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^0-9]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(610, regex)
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
fn test_re2_dat_0000000611() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^0-9]+)$"##;
    let text: &'static str = r##"abc123"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(611, regex)
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
fn test_re2_dat_0000000612() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^0-9]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(612, regex)
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
fn test_re2_dat_0000000613() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^0-9]+)"##;
    let text: &'static str = r##"abc123"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(613, regex)
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
fn test_re2_dat_0000000614() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[^0-9]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(614, regex)
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
fn test_re2_dat_0000000615() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[^0-9]+)$"##;
    let text: &'static str = r##"abc123"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(615, regex)
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
fn test_re2_dat_0000000616() -> Result<(), Error> {
    let regex: &'static str = r##"(?i)[@-A]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(616, regex)
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
fn test_re2_dat_0000000617() -> Result<(), Error> {
    let regex: &'static str = r##"(?i)[@-A]+"##;
    let text: &'static str = r##"@AaB"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(617, regex)
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
fn test_re2_dat_0000000618() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[@-A]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(618, regex)
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
fn test_re2_dat_0000000619() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[@-A]+)$"##;
    let text: &'static str = r##"@AaB"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(619, regex)
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
fn test_re2_dat_0000000620() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[@-A]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(620, regex)
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
fn test_re2_dat_0000000621() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[@-A]+)"##;
    let text: &'static str = r##"@AaB"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(621, regex)
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
fn test_re2_dat_0000000622() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?i)[@-A]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(622, regex)
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
fn test_re2_dat_0000000623() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?i)[@-A]+)$"##;
    let text: &'static str = r##"@AaB"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(623, regex)
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
fn test_re2_dat_0000000624() -> Result<(), Error> {
    let regex: &'static str = r##"(?i)[A-Z]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(624, regex)
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
fn test_re2_dat_0000000625() -> Result<(), Error> {
    let regex: &'static str = r##"(?i)[A-Z]+"##;
    let text: &'static str = r##"aAzZ"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(625, regex)
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
fn test_re2_dat_0000000626() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[A-Z]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(626, regex)
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
fn test_re2_dat_0000000627() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[A-Z]+)$"##;
    let text: &'static str = r##"aAzZ"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(627, regex)
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
fn test_re2_dat_0000000628() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[A-Z]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(628, regex)
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
fn test_re2_dat_0000000629() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[A-Z]+)"##;
    let text: &'static str = r##"aAzZ"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(629, regex)
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
fn test_re2_dat_0000000630() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?i)[A-Z]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(630, regex)
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
fn test_re2_dat_0000000631() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?i)[A-Z]+)$"##;
    let text: &'static str = r##"aAzZ"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(631, regex)
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
fn test_re2_dat_0000000632() -> Result<(), Error> {
    let regex: &'static str = r##"(?i)[^\\]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(632, regex)
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
fn test_re2_dat_0000000633() -> Result<(), Error> {
    let regex: &'static str = r##"(?i)[^\\]+"##;
    let text: &'static str = r##"Aa\"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(633, regex)
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
fn test_re2_dat_0000000634() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[^\\]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(634, regex)
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
fn test_re2_dat_0000000635() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[^\\]+)$"##;
    let text: &'static str = r##"Aa\"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(635, regex)
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
fn test_re2_dat_0000000636() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[^\\]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(636, regex)
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
fn test_re2_dat_0000000637() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[^\\]+)"##;
    let text: &'static str = r##"Aa\"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(637, regex)
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
fn test_re2_dat_0000000638() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?i)[^\\]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(638, regex)
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
fn test_re2_dat_0000000639() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?i)[^\\]+)$"##;
    let text: &'static str = r##"Aa\"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(639, regex)
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
fn test_re2_dat_0000000640() -> Result<(), Error> {
    let regex: &'static str = r##"(?i)[acegikmoqsuwy]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(640, regex)
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
fn test_re2_dat_0000000641() -> Result<(), Error> {
    let regex: &'static str = r##"(?i)[acegikmoqsuwy]+"##;
    let text: &'static str = r##"acegikmoqsuwyACEGIKMOQSUWY"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(26))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(641, regex)
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
fn test_re2_dat_0000000642() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[acegikmoqsuwy]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(642, regex)
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
fn test_re2_dat_0000000643() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[acegikmoqsuwy]+)$"##;
    let text: &'static str = r##"acegikmoqsuwyACEGIKMOQSUWY"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(26))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(643, regex)
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
fn test_re2_dat_0000000644() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[acegikmoqsuwy]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(644, regex)
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
fn test_re2_dat_0000000645() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?i)[acegikmoqsuwy]+)"##;
    let text: &'static str = r##"acegikmoqsuwyACEGIKMOQSUWY"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(26))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(645, regex)
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
fn test_re2_dat_0000000646() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?i)[acegikmoqsuwy]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(646, regex)
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
fn test_re2_dat_0000000647() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?i)[acegikmoqsuwy]+)$"##;
    let text: &'static str = r##"acegikmoqsuwyACEGIKMOQSUWY"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(26))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(647, regex)
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
fn test_re2_dat_0000000648() -> Result<(), Error> {
    let regex: &'static str = r##"[@-A]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(648, regex)
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
fn test_re2_dat_0000000649() -> Result<(), Error> {
    let regex: &'static str = r##"[@-A]+"##;
    let text: &'static str = r##"@AaB"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(649, regex)
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
fn test_re2_dat_0000000650() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[@-A]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(650, regex)
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
fn test_re2_dat_0000000651() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[@-A]+)$"##;
    let text: &'static str = r##"@AaB"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(651, regex)
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
fn test_re2_dat_0000000652() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[@-A]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(652, regex)
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
fn test_re2_dat_0000000653() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[@-A]+)"##;
    let text: &'static str = r##"@AaB"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(653, regex)
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
fn test_re2_dat_0000000654() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[@-A]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(654, regex)
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
fn test_re2_dat_0000000655() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[@-A]+)$"##;
    let text: &'static str = r##"@AaB"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(655, regex)
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
fn test_re2_dat_0000000656() -> Result<(), Error> {
    let regex: &'static str = r##"[A-Z]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(656, regex)
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
fn test_re2_dat_0000000657() -> Result<(), Error> {
    let regex: &'static str = r##"[A-Z]+"##;
    let text: &'static str = r##"aAzZ"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(1), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(657, regex)
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
fn test_re2_dat_0000000658() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[A-Z]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(658, regex)
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
fn test_re2_dat_0000000659() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[A-Z]+)$"##;
    let text: &'static str = r##"aAzZ"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(659, regex)
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
fn test_re2_dat_0000000660() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[A-Z]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(660, regex)
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
fn test_re2_dat_0000000661() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[A-Z]+)"##;
    let text: &'static str = r##"aAzZ"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(661, regex)
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
fn test_re2_dat_0000000662() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[A-Z]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(662, regex)
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
fn test_re2_dat_0000000663() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[A-Z]+)$"##;
    let text: &'static str = r##"aAzZ"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(663, regex)
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
fn test_re2_dat_0000000664() -> Result<(), Error> {
    let regex: &'static str = r##"[^\\]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(664, regex)
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
fn test_re2_dat_0000000665() -> Result<(), Error> {
    let regex: &'static str = r##"[^\\]+"##;
    let text: &'static str = r##"Aa\"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(665, regex)
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
fn test_re2_dat_0000000666() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^\\]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(666, regex)
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
fn test_re2_dat_0000000667() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^\\]+)$"##;
    let text: &'static str = r##"Aa\"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(667, regex)
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
fn test_re2_dat_0000000668() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^\\]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(668, regex)
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
fn test_re2_dat_0000000669() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[^\\]+)"##;
    let text: &'static str = r##"Aa\"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(2))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(669, regex)
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
fn test_re2_dat_0000000670() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[^\\]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(670, regex)
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
fn test_re2_dat_0000000671() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[^\\]+)$"##;
    let text: &'static str = r##"Aa\"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(671, regex)
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
fn test_re2_dat_0000000672() -> Result<(), Error> {
    let regex: &'static str = r##"[acegikmoqsuwy]+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(672, regex)
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
fn test_re2_dat_0000000673() -> Result<(), Error> {
    let regex: &'static str = r##"[acegikmoqsuwy]+"##;
    let text: &'static str = r##"acegikmoqsuwyACEGIKMOQSUWY"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(13))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(673, regex)
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
fn test_re2_dat_0000000674() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[acegikmoqsuwy]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(674, regex)
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
fn test_re2_dat_0000000675() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[acegikmoqsuwy]+)$"##;
    let text: &'static str = r##"acegikmoqsuwyACEGIKMOQSUWY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(675, regex)
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
fn test_re2_dat_0000000676() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[acegikmoqsuwy]+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(676, regex)
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
fn test_re2_dat_0000000677() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[acegikmoqsuwy]+)"##;
    let text: &'static str = r##"acegikmoqsuwyACEGIKMOQSUWY"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(13))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(677, regex)
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
fn test_re2_dat_0000000678() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[acegikmoqsuwy]+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(678, regex)
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
fn test_re2_dat_0000000679() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[acegikmoqsuwy]+)$"##;
    let text: &'static str = r##"acegikmoqsuwyACEGIKMOQSUWY"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(679, regex)
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
fn test_re2_dat_0000000680() -> Result<(), Error> {
    let regex: &'static str = r##"^abc"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(680, regex)
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
fn test_re2_dat_0000000681() -> Result<(), Error> {
    let regex: &'static str = r##"^abc"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(681, regex)
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
fn test_re2_dat_0000000682() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^abc)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(682, regex)
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
fn test_re2_dat_0000000683() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^abc)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(683, regex)
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
fn test_re2_dat_0000000684() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^abc)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(684, regex)
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
fn test_re2_dat_0000000685() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^abc)"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(685, regex)
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
fn test_re2_dat_0000000686() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^abc)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(686, regex)
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
fn test_re2_dat_0000000687() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^abc)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(687, regex)
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
fn test_re2_dat_0000000688() -> Result<(), Error> {
    let regex: &'static str = r##"^abc"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(688, regex)
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
fn test_re2_dat_0000000689() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^abc)$"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(689, regex)
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
fn test_re2_dat_0000000690() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^abc)"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(690, regex)
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
fn test_re2_dat_0000000691() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^abc)$"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(691, regex)
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
fn test_re2_dat_0000000692() -> Result<(), Error> {
    let regex: &'static str = r##"^[ay]*[bx]+c"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(692, regex)
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
fn test_re2_dat_0000000693() -> Result<(), Error> {
    let regex: &'static str = r##"^[ay]*[bx]+c"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(693, regex)
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
fn test_re2_dat_0000000694() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^[ay]*[bx]+c)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(694, regex)
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
fn test_re2_dat_0000000695() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^[ay]*[bx]+c)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(695, regex)
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
fn test_re2_dat_0000000696() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^[ay]*[bx]+c)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(696, regex)
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
fn test_re2_dat_0000000697() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^[ay]*[bx]+c)"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(697, regex)
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
fn test_re2_dat_0000000698() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^[ay]*[bx]+c)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(698, regex)
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
fn test_re2_dat_0000000699() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^[ay]*[bx]+c)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(699, regex)
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
fn test_re2_dat_0000000700() -> Result<(), Error> {
    let regex: &'static str = r##"^[ay]*[bx]+c"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(700, regex)
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
fn test_re2_dat_0000000701() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^[ay]*[bx]+c)$"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(701, regex)
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
fn test_re2_dat_0000000702() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^[ay]*[bx]+c)"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(702, regex)
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
fn test_re2_dat_0000000703() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^[ay]*[bx]+c)$"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(703, regex)
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
fn test_re2_dat_0000000704() -> Result<(), Error> {
    let regex: &'static str = r##"def$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(704, regex)
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
fn test_re2_dat_0000000705() -> Result<(), Error> {
    let regex: &'static str = r##"def$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(705, regex)
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
fn test_re2_dat_0000000706() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:def$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(706, regex)
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
fn test_re2_dat_0000000707() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:def$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(707, regex)
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
fn test_re2_dat_0000000708() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:def$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(708, regex)
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
fn test_re2_dat_0000000709() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:def$)"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(709, regex)
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
fn test_re2_dat_0000000710() -> Result<(), Error> {
    let regex: &'static str = r##"(?:def$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(710, regex)
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
fn test_re2_dat_0000000711() -> Result<(), Error> {
    let regex: &'static str = r##"(?:def$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(711, regex)
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
fn test_re2_dat_0000000712() -> Result<(), Error> {
    let regex: &'static str = r##"def$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(712, regex)
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
fn test_re2_dat_0000000713() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:def$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(713, regex)
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
fn test_re2_dat_0000000714() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:def$)"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(714, regex)
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
fn test_re2_dat_0000000715() -> Result<(), Error> {
    let regex: &'static str = r##"(?:def$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(715, regex)
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
fn test_re2_dat_0000000716() -> Result<(), Error> {
    let regex: &'static str = r##"d[ex][fy]$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(716, regex)
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
fn test_re2_dat_0000000717() -> Result<(), Error> {
    let regex: &'static str = r##"d[ex][fy]$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(717, regex)
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
fn test_re2_dat_0000000718() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:d[ex][fy]$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(718, regex)
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
fn test_re2_dat_0000000719() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:d[ex][fy]$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(719, regex)
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
fn test_re2_dat_0000000720() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:d[ex][fy]$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(720, regex)
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
fn test_re2_dat_0000000721() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:d[ex][fy]$)"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(721, regex)
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
fn test_re2_dat_0000000722() -> Result<(), Error> {
    let regex: &'static str = r##"(?:d[ex][fy]$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(722, regex)
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
fn test_re2_dat_0000000723() -> Result<(), Error> {
    let regex: &'static str = r##"(?:d[ex][fy]$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(723, regex)
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
fn test_re2_dat_0000000724() -> Result<(), Error> {
    let regex: &'static str = r##"d[ex][fy]$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(724, regex)
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
fn test_re2_dat_0000000725() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:d[ex][fy]$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(725, regex)
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
fn test_re2_dat_0000000726() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:d[ex][fy]$)"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(726, regex)
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
fn test_re2_dat_0000000727() -> Result<(), Error> {
    let regex: &'static str = r##"(?:d[ex][fy]$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(727, regex)
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
fn test_re2_dat_0000000728() -> Result<(), Error> {
    let regex: &'static str = r##"[dz][ex][fy]$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(728, regex)
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
fn test_re2_dat_0000000729() -> Result<(), Error> {
    let regex: &'static str = r##"[dz][ex][fy]$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(729, regex)
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
fn test_re2_dat_0000000730() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[dz][ex][fy]$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(730, regex)
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
fn test_re2_dat_0000000731() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[dz][ex][fy]$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(731, regex)
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
fn test_re2_dat_0000000732() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[dz][ex][fy]$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(732, regex)
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
fn test_re2_dat_0000000733() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[dz][ex][fy]$)"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(733, regex)
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
fn test_re2_dat_0000000734() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[dz][ex][fy]$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(734, regex)
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
fn test_re2_dat_0000000735() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[dz][ex][fy]$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(735, regex)
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
fn test_re2_dat_0000000736() -> Result<(), Error> {
    let regex: &'static str = r##"[dz][ex][fy]$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(736, regex)
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
fn test_re2_dat_0000000737() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[dz][ex][fy]$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(737, regex)
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
fn test_re2_dat_0000000738() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:[dz][ex][fy]$)"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(738, regex)
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
fn test_re2_dat_0000000739() -> Result<(), Error> {
    let regex: &'static str = r##"(?:[dz][ex][fy]$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(739, regex)
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
fn test_re2_dat_0000000740() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)^abc"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(740, regex)
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
fn test_re2_dat_0000000741() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)^abc"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(741, regex)
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
fn test_re2_dat_0000000742() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^abc)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(742, regex)
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
fn test_re2_dat_0000000743() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^abc)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(743, regex)
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
fn test_re2_dat_0000000744() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^abc)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(744, regex)
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
fn test_re2_dat_0000000745() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^abc)"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(745, regex)
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
fn test_re2_dat_0000000746() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)^abc)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(746, regex)
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
fn test_re2_dat_0000000747() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)^abc)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(747, regex)
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
fn test_re2_dat_0000000748() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)^abc"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(748, regex)
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
fn test_re2_dat_0000000749() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^abc)$"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(749, regex)
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
fn test_re2_dat_0000000750() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^abc)"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(750, regex)
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
fn test_re2_dat_0000000751() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)^abc)$"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(751, regex)
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
fn test_re2_dat_0000000752() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)^[ay]*[bx]+c"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(752, regex)
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
fn test_re2_dat_0000000753() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)^[ay]*[bx]+c"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(753, regex)
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
fn test_re2_dat_0000000754() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^[ay]*[bx]+c)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(754, regex)
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
fn test_re2_dat_0000000755() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^[ay]*[bx]+c)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(755, regex)
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
fn test_re2_dat_0000000756() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^[ay]*[bx]+c)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(756, regex)
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
fn test_re2_dat_0000000757() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^[ay]*[bx]+c)"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(757, regex)
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
fn test_re2_dat_0000000758() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)^[ay]*[bx]+c)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(758, regex)
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
fn test_re2_dat_0000000759() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)^[ay]*[bx]+c)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(759, regex)
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
fn test_re2_dat_0000000760() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)^[ay]*[bx]+c"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(760, regex)
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
fn test_re2_dat_0000000761() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^[ay]*[bx]+c)$"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(761, regex)
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
fn test_re2_dat_0000000762() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)^[ay]*[bx]+c)"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(4))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(762, regex)
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
fn test_re2_dat_0000000763() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)^[ay]*[bx]+c)$"##;
    let text: &'static str = r##"aabcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(763, regex)
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
fn test_re2_dat_0000000764() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)def$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(764, regex)
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
fn test_re2_dat_0000000765() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)def$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(765, regex)
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
fn test_re2_dat_0000000766() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)def$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(766, regex)
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
fn test_re2_dat_0000000767() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)def$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(767, regex)
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
fn test_re2_dat_0000000768() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)def$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(768, regex)
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
fn test_re2_dat_0000000769() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)def$)"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(769, regex)
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
fn test_re2_dat_0000000770() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)def$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(770, regex)
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
fn test_re2_dat_0000000771() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)def$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(771, regex)
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
fn test_re2_dat_0000000772() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)def$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(772, regex)
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
fn test_re2_dat_0000000773() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)def$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(773, regex)
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
fn test_re2_dat_0000000774() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)def$)"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(774, regex)
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
fn test_re2_dat_0000000775() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)def$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(775, regex)
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
fn test_re2_dat_0000000776() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)d[ex][fy]$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(776, regex)
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
fn test_re2_dat_0000000777() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)d[ex][fy]$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(777, regex)
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
fn test_re2_dat_0000000778() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)d[ex][fy]$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(778, regex)
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
fn test_re2_dat_0000000779() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)d[ex][fy]$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(779, regex)
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
fn test_re2_dat_0000000780() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)d[ex][fy]$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(780, regex)
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
fn test_re2_dat_0000000781() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)d[ex][fy]$)"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(781, regex)
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
fn test_re2_dat_0000000782() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)d[ex][fy]$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(782, regex)
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
fn test_re2_dat_0000000783() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)d[ex][fy]$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(783, regex)
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
fn test_re2_dat_0000000784() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)d[ex][fy]$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(784, regex)
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
fn test_re2_dat_0000000785() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)d[ex][fy]$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(785, regex)
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
fn test_re2_dat_0000000786() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)d[ex][fy]$)"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(786, regex)
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
fn test_re2_dat_0000000787() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)d[ex][fy]$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(787, regex)
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
fn test_re2_dat_0000000788() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)[dz][ex][fy]$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(788, regex)
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
fn test_re2_dat_0000000789() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)[dz][ex][fy]$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(789, regex)
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
fn test_re2_dat_0000000790() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)[dz][ex][fy]$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(790, regex)
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
fn test_re2_dat_0000000791() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)[dz][ex][fy]$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(791, regex)
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
fn test_re2_dat_0000000792() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)[dz][ex][fy]$)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(792, regex)
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
fn test_re2_dat_0000000793() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)[dz][ex][fy]$)"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(793, regex)
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
fn test_re2_dat_0000000794() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)[dz][ex][fy]$)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(794, regex)
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
fn test_re2_dat_0000000795() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)[dz][ex][fy]$)$"##;
    let text: &'static str = r##"abcdef"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(3), Some(6))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(795, regex)
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
fn test_re2_dat_0000000796() -> Result<(), Error> {
    let regex: &'static str = r##"(?m)[dz][ex][fy]$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(796, regex)
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
fn test_re2_dat_0000000797() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)[dz][ex][fy]$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(797, regex)
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
fn test_re2_dat_0000000798() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?m)[dz][ex][fy]$)"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(798, regex)
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
fn test_re2_dat_0000000799() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?m)[dz][ex][fy]$)$"##;
    let text: &'static str = r##"abcdeff"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(799, regex)
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
fn test_re2_dat_0000000800() -> Result<(), Error> {
    let regex: &'static str = r##"^"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(800, regex)
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
fn test_re2_dat_0000000801() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^)$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(801, regex)
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
fn test_re2_dat_0000000802() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^)"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(802, regex)
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
fn test_re2_dat_0000000803() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^)$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(803, regex)
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
fn test_re2_dat_0000000804() -> Result<(), Error> {
    let regex: &'static str = r##"^^"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(804, regex)
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
fn test_re2_dat_0000000805() -> Result<(), Error> {
    let regex: &'static str = r##"^^"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(805, regex)
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
fn test_re2_dat_0000000806() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(806, regex)
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
fn test_re2_dat_0000000807() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^)$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(807, regex)
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
fn test_re2_dat_0000000808() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(808, regex)
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
fn test_re2_dat_0000000809() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:^^)"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(809, regex)
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
fn test_re2_dat_0000000810() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(810, regex)
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
fn test_re2_dat_0000000811() -> Result<(), Error> {
    let regex: &'static str = r##"(?:^^)$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(811, regex)
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
fn test_re2_dat_0000000812() -> Result<(), Error> {
    let regex: &'static str = r##"ab*"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(812, regex)
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
fn test_re2_dat_0000000813() -> Result<(), Error> {
    let regex: &'static str = r##"ab*"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(813, regex)
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
fn test_re2_dat_0000000814() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ab*)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(814, regex)
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
fn test_re2_dat_0000000815() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ab*)$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(815, regex)
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
fn test_re2_dat_0000000816() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ab*)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(816, regex)
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
fn test_re2_dat_0000000817() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:ab*)"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(817, regex)
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
fn test_re2_dat_0000000818() -> Result<(), Error> {
    let regex: &'static str = r##"(?:ab*)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(818, regex)
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
fn test_re2_dat_0000000819() -> Result<(), Error> {
    let regex: &'static str = r##"(?:ab*)$"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(1))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(819, regex)
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
fn test_re2_dat_0000000820() -> Result<(), Error> {
    let regex: &'static str = r##"\w*I\w*"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(820, regex)
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
fn test_re2_dat_0000000821() -> Result<(), Error> {
    let regex: &'static str = r##"\w*I\w*"##;
    let text: &'static str = r##"Inc."##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(821, regex)
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
fn test_re2_dat_0000000822() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\w*I\w*)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(822, regex)
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
fn test_re2_dat_0000000823() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\w*I\w*)$"##;
    let text: &'static str = r##"Inc."##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(823, regex)
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
fn test_re2_dat_0000000824() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\w*I\w*)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(824, regex)
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
fn test_re2_dat_0000000825() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:\w*I\w*)"##;
    let text: &'static str = r##"Inc."##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(825, regex)
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
fn test_re2_dat_0000000826() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\w*I\w*)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(826, regex)
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
fn test_re2_dat_0000000827() -> Result<(), Error> {
    let regex: &'static str = r##"(?:\w*I\w*)$"##;
    let text: &'static str = r##"Inc."##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(827, regex)
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
fn test_re2_dat_0000000828() -> Result<(), Error> {
    let regex: &'static str = r##"(?:|a)*"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(828, regex)
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
fn test_re2_dat_0000000829() -> Result<(), Error> {
    let regex: &'static str = r##"(?:|a)*"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(829, regex)
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
fn test_re2_dat_0000000830() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?:|a)*)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(830, regex)
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
fn test_re2_dat_0000000831() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?:|a)*)$"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(831, regex)
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
fn test_re2_dat_0000000832() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?:|a)*)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(832, regex)
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
fn test_re2_dat_0000000833() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?:|a)*)"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(833, regex)
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
fn test_re2_dat_0000000834() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?:|a)*)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(834, regex)
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
fn test_re2_dat_0000000835() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?:|a)*)$"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(835, regex)
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
fn test_re2_dat_0000000836() -> Result<(), Error> {
    let regex: &'static str = r##"(?:|a)+"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(836, regex)
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
fn test_re2_dat_0000000837() -> Result<(), Error> {
    let regex: &'static str = r##"(?:|a)+"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(837, regex)
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
fn test_re2_dat_0000000838() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?:|a)+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(838, regex)
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
fn test_re2_dat_0000000839() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?:|a)+)$"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(839, regex)
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
fn test_re2_dat_0000000840() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?:|a)+)"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(840, regex)
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
fn test_re2_dat_0000000841() -> Result<(), Error> {
    let regex: &'static str = r##"^(?:(?:|a)+)"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(841, regex)
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
fn test_re2_dat_0000000842() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?:|a)+)$"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(842, regex)
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
fn test_re2_dat_0000000843() -> Result<(), Error> {
    let regex: &'static str = r##"(?:(?:|a)+)$"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(3))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(843, regex)
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
fn test_repetition_dat_0000000000() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(0, regex)
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
fn test_repetition_dat_0000000001() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(1, regex)
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
fn test_repetition_dat_0000000002() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))((..)|(.))"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(2, regex)
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
fn test_repetition_dat_0000000004() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){1}"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(4, regex)
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
fn test_repetition_dat_0000000005() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){2}"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(5, regex)
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
fn test_repetition_dat_0000000006() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){3}"##;
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
fn test_repetition_dat_0000000008() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))*"##;
    let text: &'static str = r##""##;
    let matches: Vec<Capture> = vec![Capture::new(Some(0), Some(0))];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(8, regex)
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
fn test_repetition_dat_0000000010() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(None, None),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(10, regex)
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
fn test_repetition_dat_0000000011() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(11, regex)
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
fn test_repetition_dat_0000000012() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(12, regex)
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
fn test_repetition_dat_0000000014() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){1}"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(None, None),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(14, regex)
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
fn test_repetition_dat_0000000015() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){2}"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(15, regex)
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
fn test_repetition_dat_0000000016() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){3}"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(16, regex)
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
fn test_repetition_dat_0000000018() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))*"##;
    let text: &'static str = r##"a"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(0), Some(1)),
        Capture::new(None, None),
        Capture::new(Some(0), Some(1)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(18, regex)
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
fn test_repetition_dat_0000000020() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(20, regex)
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
fn test_repetition_dat_0000000021() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(1)),
        Capture::new(None, None),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(1), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(21, regex)
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
fn test_repetition_dat_0000000022() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(22, regex)
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
fn test_repetition_dat_0000000024() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){1}"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(24, regex)
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
fn test_repetition_dat_0000000025() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){2}"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(1), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(1), Some(2)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(25, regex)
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
fn test_repetition_dat_0000000026() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){3}"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(26, regex)
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
fn test_repetition_dat_0000000028() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))*"##;
    let text: &'static str = r##"aa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(28, regex)
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
fn test_repetition_dat_0000000030() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(30, regex)
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
fn test_repetition_dat_0000000031() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(3)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(31, regex)
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
fn test_repetition_dat_0000000032() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(0), Some(1)),
        Capture::new(None, None),
        Capture::new(Some(0), Some(1)),
        Capture::new(Some(1), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(1), Some(2)),
        Capture::new(Some(2), Some(3)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(32, regex)
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
fn test_repetition_dat_0000000034() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){1}"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(34, regex)
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
fn test_repetition_dat_0000000037() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){3}"##;
    let text: &'static str = r##"aaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(3)),
        Capture::new(Some(2), Some(3)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(3)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(37, regex)
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
fn test_repetition_dat_0000000042() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))"##;
    let text: &'static str = r##"aaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(42, regex)
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
fn test_repetition_dat_0000000043() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"aaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(43, regex)
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
fn test_repetition_dat_0000000044() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"aaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(3)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(3)),
        Capture::new(Some(3), Some(4)),
        Capture::new(None, None),
        Capture::new(Some(3), Some(4)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(44, regex)
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
fn test_repetition_dat_0000000046() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){1}"##;
    let text: &'static str = r##"aaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(46, regex)
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
fn test_repetition_dat_0000000047() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){2}"##;
    let text: &'static str = r##"aaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(47, regex)
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
fn test_repetition_dat_0000000051() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))*"##;
    let text: &'static str = r##"aaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(51, regex)
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
fn test_repetition_dat_0000000053() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))"##;
    let text: &'static str = r##"aaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(53, regex)
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
fn test_repetition_dat_0000000054() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"aaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(54, regex)
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
fn test_repetition_dat_0000000055() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"aaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(5)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(None, None),
        Capture::new(Some(4), Some(5)),
        Capture::new(None, None),
        Capture::new(Some(4), Some(5)),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(55, regex)
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
fn test_repetition_dat_0000000057() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){1}"##;
    let text: &'static str = r##"aaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(57, regex)
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
fn test_repetition_dat_0000000058() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){2}"##;
    let text: &'static str = r##"aaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(58, regex)
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
fn test_repetition_dat_0000000065() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))"##;
    let text: &'static str = r##"aaaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(65, regex)
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
fn test_repetition_dat_0000000066() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"aaaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(66, regex)
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
fn test_repetition_dat_0000000067() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.))((..)|(.))((..)|(.))"##;
    let text: &'static str = r##"aaaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(6)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
        Capture::new(Some(2), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(None, None),
        Capture::new(Some(4), Some(6)),
        Capture::new(Some(4), Some(6)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(67, regex)
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
fn test_repetition_dat_0000000069() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){1}"##;
    let text: &'static str = r##"aaaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(Some(0), Some(2)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(69, regex)
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
fn test_repetition_dat_0000000070() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){2}"##;
    let text: &'static str = r##"aaaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(Some(2), Some(4)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(70, regex)
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
fn test_repetition_dat_0000000071() -> Result<(), Error> {
    let regex: &'static str = r##"((..)|(.)){3}"##;
    let text: &'static str = r##"aaaaaa"##;
    let matches: Vec<Capture> = vec![
        Capture::new(Some(0), Some(6)),
        Capture::new(Some(4), Some(6)),
        Capture::new(Some(4), Some(6)),
        Capture::new(None, None),
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(71, regex)
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
