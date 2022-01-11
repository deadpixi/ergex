use crate::*;

struct CollectingHandler {
    matches: Vec<Vec<Capture>>,
}

impl CollectingHandler {
    fn new() -> Self {
        Self {
            matches: Vec::new(),
        }
    }

    fn to_results(self) -> Vec<Vec<Capture>> {
        self.matches
    }
}

impl MatchHandler for CollectingHandler {
    fn on_match(&mut self, _id: usize, matches: &[Capture]) -> ContinueMatching {
        self.matches.push(matches.to_vec().clone());
        ContinueMatching::Yes
    }
}

#[test]
fn test_should_be_fast() -> Result<(), Error> {
    let regex: &'static str = r##"a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?a?aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"##;
    let text: &'static str =
        r##"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"##;
    let expected: Vec<Vec<Capture>> = vec![vec![Capture::new(Some(0), Some(64))]];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(823, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = CollectingHandler::new();
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();

    let results = handler.to_results();
    assert_eq!(expected, results);
    Ok(())
}

#[test]
fn test_repeated_matches() -> Result<(), Error> {
    let regex: &'static str = r##"a(b)"##;
    let text: &'static str = r##"ab ab ab"##;
    let expected: Vec<Vec<Capture>> = vec![
        vec![
            Capture::new(Some(0), Some(2)),
            Capture::new(Some(1), Some(2)),
        ],
        vec![
            Capture::new(Some(3), Some(5)),
            Capture::new(Some(4), Some(5)),
        ],
        vec![
            Capture::new(Some(6), Some(8)),
            Capture::new(Some(7), Some(8)),
        ],
    ];
    let database = DatabaseBuilder::new()
        .with_expression(
            Regex::new(823, regex)
                .encoding(Encoding::Byte)
                .case_sensitive(true)
                .mode(MatchMode::All(Submatch::All))
                .build()?,
        )
        .build();
    let mut handler = CollectingHandler::new();
    let mut scratch = database.make_scratch(&mut handler);
    scratch.push(text.as_bytes());
    scratch.finish();

    let results = handler.to_results();
    assert_eq!(expected, results);
    Ok(())
}
