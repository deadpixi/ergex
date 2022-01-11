use crate::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

struct TestHandler {
    matches: HashMap<usize, Vec<Vec<Capture>>>,
}

impl TestHandler {
    fn new() -> Self {
        Self {
            matches: HashMap::new(),
        }
    }

    fn to_results(self) -> HashMap<usize, Vec<Vec<Capture>>> {
        self.matches
    }
}

impl MatchHandler for TestHandler {
    fn on_match(&mut self, id: usize, matches: &[Capture]) -> ContinueMatching {
        if !self.matches.contains_key(&id) {
            self.matches.insert(id, Vec::new());
        }

        self.matches
            .get_mut(&id)
            .unwrap()
            .push(matches.to_vec().clone());
        ContinueMatching::Yes
    }

    fn get_pulse_interval(&self) -> usize {
        1_000_000
    }
}

#[test]
fn test_shakespeare() -> Result<(), Error> {
    let mut builder = DatabaseBuilder::new();
    let mut expected: HashMap<usize, Vec<Vec<Capture>>> = HashMap::new();

    builder = builder.with_expression(
        Regex::new(0, r##"abusing\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        0,
        vec![
            vec![Capture::new(Some(2212418), Some(2212425))],
            vec![Capture::new(Some(3331636), Some(3331643))],
            vec![Capture::new(Some(4134440), Some(4134447))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(1, r##"abysm\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        1,
        vec![
            vec![Capture::new(Some(85319), Some(85324))],
            vec![Capture::new(Some(368593), Some(368598))],
            vec![Capture::new(Some(4510074), Some(4510079))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(2, r##"accepts\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        2,
        vec![
            vec![Capture::new(Some(9180), Some(9187))],
            vec![Capture::new(Some(1117289), Some(1117296))],
            vec![Capture::new(Some(4621670), Some(4621677))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(3, r##"accused\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        3,
        vec![
            vec![Capture::new(Some(3655230), Some(3655237))],
            vec![Capture::new(Some(3872590), Some(3872597))],
            vec![Capture::new(Some(5338730), Some(5338737))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(4, r##"accusing\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        4,
        vec![
            vec![Capture::new(Some(49245), Some(49253))],
            vec![Capture::new(Some(1508127), Some(1508135))],
            vec![Capture::new(Some(3661829), Some(3661837))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(5, r##"achievement\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        5,
        vec![
            vec![Capture::new(Some(1509187), Some(1509198))],
            vec![Capture::new(Some(1613356), Some(1613367))],
            vec![Capture::new(Some(4893473), Some(4893484))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(6, r##"acquittance\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        6,
        vec![
            vec![Capture::new(Some(992730), Some(992741))],
            vec![Capture::new(Some(1171410), Some(1171421))],
            vec![Capture::new(Some(4136184), Some(4136195))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(7, r##"acquitted\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        7,
        vec![
            vec![Capture::new(Some(1476087), Some(1476096))],
            vec![Capture::new(Some(3290802), Some(3290811))],
            vec![Capture::new(Some(3301413), Some(3301422))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(8, r##"actium\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        8,
        vec![
            vec![Capture::new(Some(347216), Some(347222))],
            vec![Capture::new(Some(349996), Some(350002))],
            vec![Capture::new(Some(351835), Some(351841))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(9, r##"adopt\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        9,
        vec![
            vec![Capture::new(Some(761719), Some(761724))],
            vec![Capture::new(Some(2031704), Some(2031709))],
            vec![Capture::new(Some(3720481), Some(3720486))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(10, r##"adorn\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        10,
        vec![
            vec![Capture::new(Some(1849099), Some(1849104))],
            vec![Capture::new(Some(4038031), Some(4038036))],
            vec![Capture::new(Some(4758888), Some(4758893))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(11, r##"affecting\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        11,
        vec![
            vec![Capture::new(Some(799071), Some(799080))],
            vec![Capture::new(Some(3346387), Some(3346396))],
            vec![Capture::new(Some(4266000), Some(4266009))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(12, r##"alien\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        12,
        vec![
            vec![Capture::new(Some(62221), Some(62226))],
            vec![Capture::new(Some(1307810), Some(1307815))],
            vec![Capture::new(Some(3287762), Some(3287767))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(13, r##"alighted\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        13,
        vec![
            vec![Capture::new(Some(3236807), Some(3236815))],
            vec![Capture::new(Some(4452701), Some(4452709))],
            vec![Capture::new(Some(4638131), Some(4638139))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(14, r##"altitude\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        14,
        vec![
            vec![Capture::new(Some(664143), Some(664151))],
            vec![Capture::new(Some(1091023), Some(1091031))],
            vec![Capture::new(Some(2738724), Some(2738732))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(15, r##"amounts\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        15,
        vec![
            vec![Capture::new(Some(222178), Some(222185))],
            vec![Capture::new(Some(2797069), Some(2797076))],
            vec![Capture::new(Some(4427389), Some(4427396))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(16, r##"amplify\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        16,
        vec![
            vec![Capture::new(Some(867830), Some(867837))],
            vec![Capture::new(Some(2772334), Some(2772341))],
            vec![Capture::new(Some(5452069), Some(5452076))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(17, r##"amply\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        17,
        vec![
            vec![Capture::new(Some(1553244), Some(1553249))],
            vec![Capture::new(Some(4546950), Some(4546955))],
            vec![Capture::new(Some(4938589), Some(4938594))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(18, r##"anatomize\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        18,
        vec![
            vec![Capture::new(Some(439237), Some(439246))],
            vec![Capture::new(Some(1373660), Some(1373669))],
            vec![Capture::new(Some(2710917), Some(2710926))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(19, r##"apparitions\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        19,
        vec![
            vec![Capture::new(Some(843670), Some(843681))],
            vec![Capture::new(Some(2922290), Some(2922301))],
            vec![Capture::new(Some(3651820), Some(3651831))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(20, r##"appeareth\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        20,
        vec![
            vec![Capture::new(Some(1085232), Some(1085241))],
            vec![Capture::new(Some(3282867), Some(3282876))],
            vec![Capture::new(Some(3873057), Some(3873066))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(21, r##"apprehends\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        21,
        vec![
            vec![Capture::new(Some(1244117), Some(1244127))],
            vec![Capture::new(Some(3133371), Some(3133381))],
            vec![Capture::new(Some(3173759), Some(3173769))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(22, r##"arabian\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        22,
        vec![
            vec![Capture::new(Some(332869), Some(332876))],
            vec![Capture::new(Some(872572), Some(872579))],
            vec![Capture::new(Some(3868299), Some(3868306))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(23, r##"armours\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        23,
        vec![
            vec![Capture::new(Some(2181909), Some(2181916))],
            vec![Capture::new(Some(2375627), Some(2375634))],
            vec![Capture::new(Some(5027284), Some(5027291))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(24, r##"ascribe\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        24,
        vec![
            vec![Capture::new(Some(125699), Some(125706))],
            vec![Capture::new(Some(1680509), Some(1680516))],
            vec![Capture::new(Some(4934884), Some(4934891))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(25, r##"assaults\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        25,
        vec![
            vec![Capture::new(Some(912144), Some(912152))],
            vec![Capture::new(Some(3609284), Some(3609292))],
            vec![Capture::new(Some(4612456), Some(4612464))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(26, r##"assisted\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        26,
        vec![
            vec![Capture::new(Some(2926212), Some(2926220))],
            vec![Capture::new(Some(5418460), Some(5418468))],
            vec![Capture::new(Some(5438689), Some(5438697))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(27, r##"attired\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        27,
        vec![
            vec![Capture::new(Some(985624), Some(985631))],
            vec![Capture::new(Some(3424717), Some(3424724))],
            vec![Capture::new(Some(4794865), Some(4794872))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(28, r##"austerity\b"##)
            .mode(MatchMode::All(Submatch::All))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        28,
        vec![
            vec![Capture::new(Some(808123), Some(808132))],
            vec![Capture::new(Some(3458177), Some(3458186))],
            vec![Capture::new(Some(4474535), Some(4474544))],
        ],
    );

    let database = builder.build();

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("test_data/shakespeare.txt");
    let mut file = File::open(d).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for block_size in [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 37, 42, 99, 1200, 4096, 9000, 65536,
        5458200,
    ] {
        let mut handler = TestHandler::new();
        let mut scratch = database.make_scratch(&mut handler);

        for chunk in contents.as_bytes().chunks(block_size) {
            scratch.push(chunk);
        }
        scratch.finish();

        let results = handler.to_results();
        for key in expected.keys() {
            assert!(results.contains_key(key));
            assert_eq!(expected[key], results[key]);
        }
    }
    Ok(())
}
