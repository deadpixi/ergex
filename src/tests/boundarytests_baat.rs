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
fn test_boundary_test_nb40() -> Result<(), Error> {
    let regex: &'static str = r##"\bfoobar.*bobllama"##;
    let text: &'static str = r##"abcd&foobarsflkjsdlfkjsdlfffbobllama"##;
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    for block_size in [1, 2, 3, 4, 5, 6, 7] {
        let matches: Vec<Capture> = vec![Capture::new(Some(5), Some(36))];
        let mut handler = TestHandler::new(matches);
        let mut scratch = database.make_scratch(&mut handler);
        for chunk in text.as_bytes().chunks(block_size) {
            scratch.push(chunk);
        }
        scratch.finish();
    }
    Ok(())
}

#[test]
fn test_boundary_test_nb41() -> Result<(), Error> {
    let regex: &'static str = r##"abcd.*\bfoobar.*bobllama"##;
    let text: &'static str = r##"zzzzzabcdlkjsdflksdjf&foobarsflkjsdlfkjsdlfffbobllama"##;
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(6, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    for block_size in [1, 2, 3, 4, 5, 6, 7] {
        let matches: Vec<Capture> = vec![Capture::new(Some(5), Some(53))];
        let mut handler = TestHandler::new(matches);
        let mut scratch = database.make_scratch(&mut handler);
        for chunk in text.as_bytes().chunks(block_size) {
            scratch.push(chunk);
        }
        scratch.finish();
    }
    Ok(())
}
