use crate::*;
use alloc_counter::{count_alloc, AllocCounter};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

type MyAllocator = std::alloc::System;
const MY_ALLOCATOR: MyAllocator = std::alloc::System;

#[global_allocator]
static A: AllocCounter<MyAllocator> = AllocCounter(MY_ALLOCATOR);

struct TestHandler {
    match_count: usize,
}

impl TestHandler {
    fn new() -> Self {
        Self { match_count: 0 }
    }
}

impl MatchHandler for TestHandler {
    fn on_match(&mut self, id: usize, _matches: &[Capture]) -> ContinueMatching {
        if id != 99 {
            self.match_count += 1;
        }
        ContinueMatching::Yes
    }

    fn get_pulse_interval(&self) -> usize {
        1_000_000
    }
}

#[test]
fn test_shakespeare() -> Result<(), Error> {
    let mut builder = DatabaseBuilder::new();

    builder = builder.with_expression(
        Regex::new(0, r##"abusing\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(1, r##"abysm\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(2, r##"accepts\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(3, r##"accused\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(4, r##"accusing\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(5, r##"achievement\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(6, r##"aching\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(7, r##"acquittance\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(8, r##"acquitted\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(9, r##"actium\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(10, r##"addle\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(11, r##"adopt\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(12, r##"adorn\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(13, r##"affecting\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(14, r##"alien\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(15, r##"alighted\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(16, r##"altitude\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(17, r##"amounts\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(18, r##"amplify\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(19, r##"amply\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(20, r##"anatomize\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(21, r##"apparitions\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(22, r##"appeareth\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(23, r##"apprehends\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(24, r##"arabian\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(25, r##"armours\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(26, r##"arn\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(27, r##"ascribe\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(28, r##"assaults\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(29, r##"assisted\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(30, r##"attired\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    builder = builder.with_expression(
        Regex::new(31, r##"austerity\b"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    // Add something a bit more complex in, to make sure
    // we don't allocate even with repetitions and such.
    builder = builder.with_expression(
        Regex::new(99, r##"((a*)(b?)(b+)b{3}|a+bcdef)"##)
            .mode(MatchMode::First(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );

    let mut handler = TestHandler::new();
    let database = builder.build();
    let mut scratch = database.make_scratch(&mut handler);
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_data/shakespeare.txt");
    let mut file = File::open(d).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let ((allocs, reallocs, deallocs), v) = count_alloc(|| {
        scratch.push(contents.as_bytes());
        scratch.finish()
    });

    v.reset();
    assert_eq!(handler.match_count, 32);
    assert_eq!(allocs, 0);
    assert_eq!(reallocs, 0);
    assert_eq!(deallocs, 0);
    Ok(())
}
