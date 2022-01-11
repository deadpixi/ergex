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
fn test_teakettle() -> Result<(), Error> {
    let mut builder = DatabaseBuilder::new();
    let mut expected: HashMap<usize, Vec<Vec<Capture>>> = HashMap::new();

    builder = builder.with_expression(
        Regex::new(1, r##"loofas.+stuffer[^\n]*interparty[^\n]*godwit"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(1, vec![]);

    builder = builder.with_expression(
        Regex::new(2, r##"procurers.*arsons"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(2, vec![]);

    builder = builder.with_expression(
        Regex::new(3, r##"^authoress[^\r\n]*typewriter[^\r\n]*disservices"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(3, vec![]);

    builder = builder.with_expression(
        Regex::new(4, r##"praesidiadyeweedisonomic.*reactivating"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(4, vec![]);

    builder = builder.with_expression(
        Regex::new(5, r##"times"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        5,
        vec![
            vec![Capture::new(Some(14189), Some(14194))],
            vec![Capture::new(Some(14228), Some(14233))],
            vec![Capture::new(Some(14293), Some(14298))],
            vec![Capture::new(Some(17560), Some(17565))],
            vec![Capture::new(Some(35394), Some(35399))],
            vec![Capture::new(Some(35826), Some(35831))],
            vec![Capture::new(Some(43969), Some(43974))],
            vec![Capture::new(Some(48300), Some(48305))],
            vec![Capture::new(Some(48987), Some(48992))],
            vec![Capture::new(Some(50780), Some(50785))],
            vec![Capture::new(Some(115630), Some(115635))],
            vec![Capture::new(Some(121587), Some(121592))],
            vec![Capture::new(Some(128775), Some(128780))],
            vec![Capture::new(Some(151841), Some(151846))],
            vec![Capture::new(Some(157788), Some(157793))],
            vec![Capture::new(Some(161384), Some(161389))],
            vec![Capture::new(Some(217089), Some(217094))],
            vec![Capture::new(Some(217178), Some(217183))],
            vec![Capture::new(Some(238760), Some(238765))],
            vec![Capture::new(Some(266123), Some(266128))],
            vec![Capture::new(Some(273551), Some(273556))],
            vec![Capture::new(Some(304695), Some(304700))],
            vec![Capture::new(Some(305880), Some(305885))],
            vec![Capture::new(Some(309753), Some(309758))],
            vec![Capture::new(Some(314865), Some(314870))],
            vec![Capture::new(Some(319702), Some(319707))],
            vec![Capture::new(Some(372697), Some(372702))],
            vec![Capture::new(Some(378654), Some(378659))],
            vec![Capture::new(Some(469077), Some(469082))],
            vec![Capture::new(Some(519635), Some(519640))],
            vec![Capture::new(Some(556158), Some(556163))],
            vec![Capture::new(Some(556183), Some(556188))],
            vec![Capture::new(Some(559496), Some(559501))],
            vec![Capture::new(Some(588951), Some(588956))],
            vec![Capture::new(Some(605859), Some(605864))],
            vec![Capture::new(Some(614993), Some(614998))],
            vec![Capture::new(Some(700818), Some(700823))],
            vec![Capture::new(Some(702386), Some(702391))],
            vec![Capture::new(Some(703095), Some(703100))],
            vec![Capture::new(Some(790788), Some(790793))],
            vec![Capture::new(Some(791435), Some(791440))],
            vec![Capture::new(Some(814309), Some(814314))],
            vec![Capture::new(Some(863388), Some(863393))],
            vec![Capture::new(Some(870279), Some(870284))],
            vec![Capture::new(Some(874726), Some(874731))],
            vec![Capture::new(Some(918715), Some(918720))],
            vec![Capture::new(Some(964281), Some(964286))],
            vec![Capture::new(Some(978664), Some(978669))],
            vec![Capture::new(Some(1024405), Some(1024410))],
            vec![Capture::new(Some(1062854), Some(1062859))],
            vec![Capture::new(Some(1078512), Some(1078517))],
            vec![Capture::new(Some(1080854), Some(1080859))],
            vec![Capture::new(Some(1118209), Some(1118214))],
            vec![Capture::new(Some(1118373), Some(1118378))],
            vec![Capture::new(Some(1126477), Some(1126482))],
            vec![Capture::new(Some(1142321), Some(1142326))],
            vec![Capture::new(Some(1165500), Some(1165505))],
            vec![Capture::new(Some(1193386), Some(1193391))],
            vec![Capture::new(Some(1206935), Some(1206940))],
            vec![Capture::new(Some(1239038), Some(1239043))],
            vec![Capture::new(Some(1239069), Some(1239074))],
            vec![Capture::new(Some(1273980), Some(1273985))],
            vec![Capture::new(Some(1292568), Some(1292573))],
            vec![Capture::new(Some(1294530), Some(1294535))],
            vec![Capture::new(Some(1296260), Some(1296265))],
            vec![Capture::new(Some(1300096), Some(1300101))],
            vec![Capture::new(Some(1301615), Some(1301620))],
            vec![Capture::new(Some(1301718), Some(1301723))],
            vec![Capture::new(Some(1315498), Some(1315503))],
            vec![Capture::new(Some(1315625), Some(1315630))],
            vec![Capture::new(Some(1333497), Some(1333502))],
            vec![Capture::new(Some(1345290), Some(1345295))],
            vec![Capture::new(Some(1375758), Some(1375763))],
            vec![Capture::new(Some(1376467), Some(1376472))],
            vec![Capture::new(Some(1381813), Some(1381818))],
            vec![Capture::new(Some(1394617), Some(1394622))],
            vec![Capture::new(Some(1402269), Some(1402274))],
            vec![Capture::new(Some(1403741), Some(1403746))],
            vec![Capture::new(Some(1420546), Some(1420551))],
            vec![Capture::new(Some(1422928), Some(1422933))],
            vec![Capture::new(Some(1437122), Some(1437127))],
            vec![Capture::new(Some(1448264), Some(1448269))],
            vec![Capture::new(Some(1448384), Some(1448389))],
            vec![Capture::new(Some(1449882), Some(1449887))],
            vec![Capture::new(Some(1472646), Some(1472651))],
            vec![Capture::new(Some(1472767), Some(1472772))],
            vec![Capture::new(Some(1481583), Some(1481588))],
            vec![Capture::new(Some(1495943), Some(1495948))],
            vec![Capture::new(Some(1528180), Some(1528185))],
            vec![Capture::new(Some(1543116), Some(1543121))],
            vec![Capture::new(Some(1583432), Some(1583437))],
            vec![Capture::new(Some(1589644), Some(1589649))],
            vec![Capture::new(Some(1619154), Some(1619159))],
            vec![Capture::new(Some(1619177), Some(1619182))],
            vec![Capture::new(Some(1661880), Some(1661885))],
            vec![Capture::new(Some(1711439), Some(1711444))],
            vec![Capture::new(Some(1747833), Some(1747838))],
            vec![Capture::new(Some(1747918), Some(1747923))],
            vec![Capture::new(Some(1889416), Some(1889421))],
            vec![Capture::new(Some(1898659), Some(1898664))],
            vec![Capture::new(Some(1914736), Some(1914741))],
            vec![Capture::new(Some(1916921), Some(1916926))],
            vec![Capture::new(Some(1917780), Some(1917785))],
            vec![Capture::new(Some(1917832), Some(1917837))],
            vec![Capture::new(Some(1935541), Some(1935546))],
            vec![Capture::new(Some(1950540), Some(1950545))],
            vec![Capture::new(Some(1953817), Some(1953822))],
            vec![Capture::new(Some(1958145), Some(1958150))],
            vec![Capture::new(Some(1958257), Some(1958262))],
            vec![Capture::new(Some(1958304), Some(1958309))],
            vec![Capture::new(Some(1987187), Some(1987192))],
            vec![Capture::new(Some(2020544), Some(2020549))],
            vec![Capture::new(Some(2020592), Some(2020597))],
            vec![Capture::new(Some(2021108), Some(2021113))],
            vec![Capture::new(Some(2046447), Some(2046452))],
            vec![Capture::new(Some(2053955), Some(2053960))],
            vec![Capture::new(Some(2081000), Some(2081005))],
            vec![Capture::new(Some(2082653), Some(2082658))],
            vec![Capture::new(Some(2083161), Some(2083166))],
            vec![Capture::new(Some(2085088), Some(2085093))],
            vec![Capture::new(Some(2154579), Some(2154584))],
            vec![Capture::new(Some(2161486), Some(2161491))],
            vec![Capture::new(Some(2168597), Some(2168602))],
            vec![Capture::new(Some(2205821), Some(2205826))],
            vec![Capture::new(Some(2235457), Some(2235462))],
            vec![Capture::new(Some(2247533), Some(2247538))],
            vec![Capture::new(Some(2255268), Some(2255273))],
            vec![Capture::new(Some(2276111), Some(2276116))],
            vec![Capture::new(Some(2297356), Some(2297361))],
            vec![Capture::new(Some(2307913), Some(2307918))],
            vec![Capture::new(Some(2311869), Some(2311874))],
            vec![Capture::new(Some(2311958), Some(2311963))],
            vec![Capture::new(Some(2335467), Some(2335472))],
            vec![Capture::new(Some(2335496), Some(2335501))],
            vec![Capture::new(Some(2419145), Some(2419150))],
            vec![Capture::new(Some(2429934), Some(2429939))],
            vec![Capture::new(Some(2444757), Some(2444762))],
            vec![Capture::new(Some(2491073), Some(2491078))],
            vec![Capture::new(Some(2513472), Some(2513477))],
            vec![Capture::new(Some(2521631), Some(2521636))],
            vec![Capture::new(Some(2525839), Some(2525844))],
            vec![Capture::new(Some(2540831), Some(2540836))],
            vec![Capture::new(Some(2547856), Some(2547861))],
            vec![Capture::new(Some(2586630), Some(2586635))],
            vec![Capture::new(Some(2602838), Some(2602843))],
            vec![Capture::new(Some(2626344), Some(2626349))],
            vec![Capture::new(Some(2643537), Some(2643542))],
            vec![Capture::new(Some(2788064), Some(2788069))],
            vec![Capture::new(Some(2813591), Some(2813596))],
            vec![Capture::new(Some(2819584), Some(2819589))],
            vec![Capture::new(Some(2829490), Some(2829495))],
            vec![Capture::new(Some(2876373), Some(2876378))],
            vec![Capture::new(Some(2892730), Some(2892735))],
            vec![Capture::new(Some(2899418), Some(2899423))],
            vec![Capture::new(Some(2899614), Some(2899619))],
            vec![Capture::new(Some(2927967), Some(2927972))],
            vec![Capture::new(Some(2932943), Some(2932948))],
            vec![Capture::new(Some(2971397), Some(2971402))],
            vec![Capture::new(Some(2986796), Some(2986801))],
            vec![Capture::new(Some(3001424), Some(3001429))],
            vec![Capture::new(Some(3013124), Some(3013129))],
            vec![Capture::new(Some(3044346), Some(3044351))],
            vec![Capture::new(Some(3089371), Some(3089376))],
            vec![Capture::new(Some(3091507), Some(3091512))],
            vec![Capture::new(Some(3121662), Some(3121667))],
            vec![Capture::new(Some(3133732), Some(3133737))],
            vec![Capture::new(Some(3139079), Some(3139084))],
            vec![Capture::new(Some(3146607), Some(3146612))],
            vec![Capture::new(Some(3147347), Some(3147352))],
            vec![Capture::new(Some(3147893), Some(3147898))],
            vec![Capture::new(Some(3152145), Some(3152150))],
            vec![Capture::new(Some(3152267), Some(3152272))],
            vec![Capture::new(Some(3155072), Some(3155077))],
            vec![Capture::new(Some(3173966), Some(3173971))],
            vec![Capture::new(Some(3186542), Some(3186547))],
            vec![Capture::new(Some(3201801), Some(3201806))],
            vec![Capture::new(Some(3222501), Some(3222506))],
            vec![Capture::new(Some(3228512), Some(3228517))],
            vec![Capture::new(Some(3234941), Some(3234946))],
            vec![Capture::new(Some(3235762), Some(3235767))],
            vec![Capture::new(Some(3235795), Some(3235800))],
            vec![Capture::new(Some(3239012), Some(3239017))],
            vec![Capture::new(Some(3245241), Some(3245246))],
            vec![Capture::new(Some(3249172), Some(3249177))],
            vec![Capture::new(Some(3251708), Some(3251713))],
            vec![Capture::new(Some(3251737), Some(3251742))],
            vec![Capture::new(Some(3251767), Some(3251772))],
            vec![Capture::new(Some(3258173), Some(3258178))],
            vec![Capture::new(Some(3259117), Some(3259122))],
            vec![Capture::new(Some(3261333), Some(3261338))],
            vec![Capture::new(Some(3281101), Some(3281106))],
            vec![Capture::new(Some(3324890), Some(3324895))],
            vec![Capture::new(Some(3329143), Some(3329148))],
            vec![Capture::new(Some(3329190), Some(3329195))],
            vec![Capture::new(Some(3350065), Some(3350070))],
            vec![Capture::new(Some(3352131), Some(3352136))],
            vec![Capture::new(Some(3354966), Some(3354971))],
            vec![Capture::new(Some(3446009), Some(3446014))],
            vec![Capture::new(Some(3473124), Some(3473129))],
            vec![Capture::new(Some(3524399), Some(3524404))],
            vec![Capture::new(Some(3567653), Some(3567658))],
            vec![Capture::new(Some(3609957), Some(3609962))],
            vec![Capture::new(Some(3634259), Some(3634264))],
            vec![Capture::new(Some(3634943), Some(3634948))],
            vec![Capture::new(Some(3648633), Some(3648638))],
            vec![Capture::new(Some(3704931), Some(3704936))],
            vec![Capture::new(Some(3704967), Some(3704972))],
            vec![Capture::new(Some(3726444), Some(3726449))],
            vec![Capture::new(Some(3729730), Some(3729735))],
            vec![Capture::new(Some(3758760), Some(3758765))],
            vec![Capture::new(Some(3760644), Some(3760649))],
            vec![Capture::new(Some(3765165), Some(3765170))],
            vec![Capture::new(Some(3779465), Some(3779470))],
            vec![Capture::new(Some(3787491), Some(3787496))],
            vec![Capture::new(Some(3794664), Some(3794669))],
            vec![Capture::new(Some(3805935), Some(3805940))],
            vec![Capture::new(Some(3817769), Some(3817774))],
            vec![Capture::new(Some(3860636), Some(3860641))],
            vec![Capture::new(Some(3863853), Some(3863858))],
            vec![Capture::new(Some(3880316), Some(3880321))],
            vec![Capture::new(Some(3880434), Some(3880439))],
            vec![Capture::new(Some(3884469), Some(3884474))],
            vec![Capture::new(Some(3896454), Some(3896459))],
            vec![Capture::new(Some(3906464), Some(3906469))],
            vec![Capture::new(Some(3906492), Some(3906497))],
            vec![Capture::new(Some(3969786), Some(3969791))],
            vec![Capture::new(Some(3986904), Some(3986909))],
            vec![Capture::new(Some(3995537), Some(3995542))],
            vec![Capture::new(Some(4006437), Some(4006442))],
            vec![Capture::new(Some(4007696), Some(4007701))],
            vec![Capture::new(Some(4008592), Some(4008597))],
            vec![Capture::new(Some(4033263), Some(4033268))],
            vec![Capture::new(Some(4057524), Some(4057529))],
            vec![Capture::new(Some(4104322), Some(4104327))],
            vec![Capture::new(Some(4117160), Some(4117165))],
            vec![Capture::new(Some(4134448), Some(4134453))],
            vec![Capture::new(Some(4167421), Some(4167426))],
            vec![Capture::new(Some(4168360), Some(4168365))],
            vec![Capture::new(Some(4169558), Some(4169563))],
            vec![Capture::new(Some(4169803), Some(4169808))],
            vec![Capture::new(Some(4173910), Some(4173915))],
            vec![Capture::new(Some(4174824), Some(4174829))],
            vec![Capture::new(Some(4186563), Some(4186568))],
            vec![Capture::new(Some(4236136), Some(4236141))],
            vec![Capture::new(Some(4237843), Some(4237848))],
            vec![Capture::new(Some(4258012), Some(4258017))],
            vec![Capture::new(Some(4258077), Some(4258082))],
            vec![Capture::new(Some(4273884), Some(4273889))],
            vec![Capture::new(Some(4274414), Some(4274419))],
            vec![Capture::new(Some(4274847), Some(4274852))],
            vec![Capture::new(Some(4306143), Some(4306148))],
            vec![Capture::new(Some(4307654), Some(4307659))],
            vec![Capture::new(Some(4320891), Some(4320896))],
            vec![Capture::new(Some(4377845), Some(4377850))],
            vec![Capture::new(Some(4416726), Some(4416731))],
            vec![Capture::new(Some(4418054), Some(4418059))],
            vec![Capture::new(Some(4430795), Some(4430800))],
            vec![Capture::new(Some(4437484), Some(4437489))],
            vec![Capture::new(Some(4438122), Some(4438127))],
            vec![Capture::new(Some(4495473), Some(4495478))],
            vec![Capture::new(Some(4559126), Some(4559131))],
            vec![Capture::new(Some(4560829), Some(4560834))],
            vec![Capture::new(Some(4571973), Some(4571978))],
            vec![Capture::new(Some(4572050), Some(4572055))],
            vec![Capture::new(Some(4643915), Some(4643920))],
            vec![Capture::new(Some(4651420), Some(4651425))],
            vec![Capture::new(Some(4715033), Some(4715038))],
            vec![Capture::new(Some(4716277), Some(4716282))],
            vec![Capture::new(Some(4724178), Some(4724183))],
            vec![Capture::new(Some(4739778), Some(4739783))],
            vec![Capture::new(Some(4774418), Some(4774423))],
            vec![Capture::new(Some(4811883), Some(4811888))],
            vec![Capture::new(Some(4862788), Some(4862793))],
            vec![Capture::new(Some(4868773), Some(4868778))],
            vec![Capture::new(Some(4924024), Some(4924029))],
            vec![Capture::new(Some(4941186), Some(4941191))],
            vec![Capture::new(Some(4973218), Some(4973223))],
            vec![Capture::new(Some(4990337), Some(4990342))],
            vec![Capture::new(Some(5026953), Some(5026958))],
            vec![Capture::new(Some(5054660), Some(5054665))],
            vec![Capture::new(Some(5055991), Some(5055996))],
            vec![Capture::new(Some(5079332), Some(5079337))],
            vec![Capture::new(Some(5079646), Some(5079651))],
            vec![Capture::new(Some(5086049), Some(5086054))],
            vec![Capture::new(Some(5088781), Some(5088786))],
            vec![Capture::new(Some(5164242), Some(5164247))],
            vec![Capture::new(Some(5191687), Some(5191692))],
            vec![Capture::new(Some(5197341), Some(5197346))],
            vec![Capture::new(Some(5225335), Some(5225340))],
            vec![Capture::new(Some(5257679), Some(5257684))],
            vec![Capture::new(Some(5266230), Some(5266235))],
            vec![Capture::new(Some(5266688), Some(5266693))],
            vec![Capture::new(Some(5295168), Some(5295173))],
            vec![Capture::new(Some(5302547), Some(5302552))],
            vec![Capture::new(Some(5351371), Some(5351376))],
            vec![Capture::new(Some(5355079), Some(5355084))],
            vec![Capture::new(Some(5358460), Some(5358465))],
            vec![Capture::new(Some(5405568), Some(5405573))],
            vec![Capture::new(Some(5408571), Some(5408576))],
            vec![Capture::new(Some(5419980), Some(5419985))],
            vec![Capture::new(Some(5443572), Some(5443577))],
            vec![Capture::new(Some(5443734), Some(5443739))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(6, r##"anticatalyst.*razer"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(6, vec![]);

    builder = builder.with_expression(
        Regex::new(7, r##"statable"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(7, vec![]);

    builder = builder.with_expression(
        Regex::new(8, r##"vicarious.*synalepha[^\n]+helistop"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(8, vec![]);

    builder = builder.with_expression(
        Regex::new(9, r##"^counterdemonstratedsownlithicreexpelmeniscus"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(9, vec![]);

    builder = builder.with_expression(
        Regex::new(10, r##"dissolutely.*imbeciles"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(10, vec![]);

    builder = builder.with_expression(
        Regex::new(11, r##"debride.{1,3}personated.{3,7}sorer"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(11, vec![]);

    builder = builder.with_expression(
        Regex::new(12, r##"^cambial.*mashes.*whiffets.*ascites"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(12, vec![]);

    builder = builder.with_expression(
        Regex::new(13, r##"inviolabilities"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(13, vec![]);

    builder = builder.with_expression(Regex::new(14, r##"raphae[^\r\n]{6,}pattie[^\r\n]{5,}bewormed[^\r\n]{10,}reseize[^\r\n]{4,}underestimate"##).
        mode(MatchMode::All(Submatch::Expression)).encoding(Encoding::Byte).case_sensitive(false).build()?);
    expected.insert(14, vec![]);

    builder = builder.with_expression(
        Regex::new(15, r##"unsuitable.*raphis[^\n]{9,}rattansflocculent"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(15, vec![]);

    builder = builder.with_expression(
        Regex::new(16, r##"subgenera"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(16, vec![]);

    builder = builder.with_expression(
        Regex::new(17, r##"microtubules[^\r\n]+pipes.*desalinizing.*fuze"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(17, vec![]);

    builder = builder.with_expression(
        Regex::new(18, r##"intercropping.+squadronsgormand"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(18, vec![]);

    builder = builder.with_expression(
        Regex::new(19, r##"esprits.+vavasour.+morionsvitellins"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(19, vec![]);

    builder = builder.with_expression(
        Regex::new(20, r##"guacos.{7,}encased.{5,}proctored.{1,}ancestries"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(20, vec![]);

    builder = builder.with_expression(
        Regex::new(21, r##"mincingly.*fruticose[^\r\n]+cheekfuls"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(21, vec![]);

    builder = builder.with_expression(
        Regex::new(
            22,
            r##"pricierpresorted.{1,9}timelier.{10,20}prenotification[^\n]*hiddenly"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(22, vec![]);

    builder = builder.with_expression(
        Regex::new(23, r##"decadents"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(23, vec![]);

    builder = builder.with_expression(
        Regex::new(
            24,
            r##"subsonically.*telephotographies.*pedestals.*undeveloped.*downier"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(24, vec![]);

    builder = builder.with_expression(
        Regex::new(
            25,
            r##"goitrogenicity.*ermineddiobol.*weening.{8,15}skidooing"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(25, vec![]);

    builder = builder.with_expression(
        Regex::new(26, r##"herder.{10,15}flatfoots"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(26, vec![]);

    builder = builder.with_expression(
        Regex::new(27, r##"snitchers.*ignobleness.*lanais.*kugels.*kappas"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(27, vec![]);

    builder = builder.with_expression(
        Regex::new(28, r##"evermore.*irreligionists"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(28, vec![]);

    builder = builder.with_expression(
        Regex::new(29, r##"subnets.*monocrystals"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(29, vec![]);

    builder = builder.with_expression(
        Regex::new(30, r##"titration"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(30, vec![]);

    builder = builder.with_expression(
        Regex::new(
            31,
            r##"paintbrush.{4,}foddering.{4,}impostors.{2,}habdalahs.{7,}simplexes"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(31, vec![]);

    builder = builder.with_expression(
        Regex::new(32, r##"endomitotic.+crenellation.+metazoic.*allogamies"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(32, vec![]);

    builder = builder.with_expression(
        Regex::new(33, r##"tussore.{9,}misadjusted.*welter"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(33, vec![]);

    builder = builder.with_expression(
        Regex::new(
            34,
            r##"rapturousnesses.*skaters.*psalmbook.*parlaying.*peebeens"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(34, vec![]);

    builder = builder.with_expression(
        Regex::new(35, r##"foresightednesses"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(35, vec![]);

    builder = builder.with_expression(
        Regex::new(
            36,
            r##"unsegregated[^\n]{2,}canticle[^\n]{3,}antiobscenity"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(36, vec![]);

    builder = builder.with_expression(
        Regex::new(37, r##"neurologist"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(37, vec![]);

    builder = builder.with_expression(
        Regex::new(
            38,
            r##"daughterless.*sacrilegiousnesses.*novelising.{3,}lauding"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(38, vec![]);

    builder = builder.with_expression(
        Regex::new(
            39,
            r##"monotone.*repairmen.*teapoy.*fullerene.*descensions"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(39, vec![]);

    builder = builder.with_expression(
        Regex::new(40, r##"maladies[^\n]+sweeties"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(40, vec![]);

    builder = builder.with_expression(
        Regex::new(41, r##"creditablenessbonteboks.*heliostat"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(41, vec![]);

    builder = builder.with_expression(
        Regex::new(
            42,
            r##"zoospore.*endoplasmic.*skirted.*underflow.*overissuing"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(42, vec![]);

    builder = builder.with_expression(
        Regex::new(43, r##"pinocytoses.{3,13}outsizesanuses"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(43, vec![]);

    builder = builder.with_expression(
        Regex::new(44, r##"autogamous[^\n]*humbugging"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(44, vec![]);

    builder = builder.with_expression(
        Regex::new(45, r##"clenchers.*noncarrier"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(45, vec![]);

    builder = builder.with_expression(
        Regex::new(46, r##"autography"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(46, vec![]);

    builder = builder.with_expression(
        Regex::new(47, r##"piggierassuagement.{9,}gatefolds.{2,4}knew"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(47, vec![]);

    builder = builder.with_expression(
        Regex::new(48, r##"thunderstruck.*corpora.*vermouths"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(48, vec![]);

    builder = builder.with_expression(
        Regex::new(49, r##"cushily[^\r\n]{2,}loafing"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(49, vec![]);

    builder = builder.with_expression(
        Regex::new(50, r##"extollerlops"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(50, vec![]);

    builder = builder.with_expression(
        Regex::new(
            51,
            r##"^((showrooms)|(initializes))[^\r\n]+vaticinates[^\r\n]+beefed"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(51, vec![]);

    builder = builder.with_expression(
        Regex::new(52, r##"executory.*remotenesses"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(52, vec![]);

    builder = builder.with_expression(
        Regex::new(53, r##"^elemis.{5,}triggerfish"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(53, vec![]);

    builder = builder.with_expression(
        Regex::new(
            54,
            r##"gamboges.{6,15}gipunderbudded[^\n]{7,}phagocytized"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(54, vec![]);

    builder = builder.with_expression(
        Regex::new(
            55,
            r##"longsomenesses.{10,20}predoctoral.{1,2}precluding.{3,3}regurgitate.*blowback"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(55, vec![]);

    builder = builder.with_expression(
        Regex::new(56, r##"dienecamelias"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(56, vec![]);

    builder = builder.with_expression(
        Regex::new(57, r##"woodbin.+horsefly"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(57, vec![]);

    builder = builder.with_expression(
        Regex::new(
            58,
            r##"((regrowing)|(mudslinger)).{10,19}frequents.{1,11}polygonies"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(58, vec![]);

    builder = builder.with_expression(
        Regex::new(
            59,
            r##"leads.{10,14}aliquant.{9,16}endodermal.*overcounts"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(59, vec![]);

    builder = builder.with_expression(
        Regex::new(60, r##"retitling"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(60, vec![]);

    builder = builder.with_expression(
        Regex::new(61, r##"dezinced.*localised"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(61, vec![]);

    builder = builder.with_expression(
        Regex::new(
            62,
            r##"rash.{7,}refilms[^\n]*centile[^\n]*queens.{9,13}chummier"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(62, vec![]);

    builder = builder.with_expression(
        Regex::new(63, r##"mulleys.{9,}redistricted.*overspecializations"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(63, vec![]);

    builder = builder.with_expression(
        Regex::new(64, r##"intermediates.{3,}discredit.+leavier"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(64, vec![]);

    builder = builder.with_expression(
        Regex::new(65, r##"haze.{7,}remilitarized.{7,}makebates.*gingersnaps"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(65, vec![]);

    builder = builder.with_expression(
        Regex::new(
            66,
            r##"feudalitiessalvarsanprotecting.*overbalanced.{4,}ait"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(66, vec![]);

    builder = builder.with_expression(
        Regex::new(67, r##"sonsycosmetologiststunners"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(67, vec![]);

    builder = builder.with_expression(
        Regex::new(68, r##"^gangplanksubapoetisecabbalah[^\r\n]*asteroid"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(68, vec![]);

    builder = builder.with_expression(
        Regex::new(69, r##"stonyhearted.{1,4}sweetnesses.*subventionary"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(69, vec![]);

    builder = builder.with_expression(
        Regex::new(70, r##"baggiecoloniallysoared"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(70, vec![]);

    builder = builder.with_expression(
        Regex::new(
            71,
            r##"hectograph.*guanidin.*reregulating.*toploftiest.*gliming"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(71, vec![]);

    builder = builder.with_expression(
        Regex::new(72, r##"valorised.*rediscussing.{7,}corked.{4,}cuticulae"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(72, vec![]);

    builder = builder.with_expression(
        Regex::new(73, r##"iniquities"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(73, vec![vec![Capture::new(Some(1640668), Some(1640678))]]);

    builder = builder.with_expression(
        Regex::new(74, r##"aboundstwiddlespedimentteleviewdominated"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(74, vec![]);

    builder = builder.with_expression(
        Regex::new(75, r##"disfigures.*epitasis"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(75, vec![]);

    builder = builder.with_expression(
        Regex::new(
            76,
            r##"aeronauts[^\r\n]+hyperextension[^\r\n]+sourdines[^\r\n]+dimwitted"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(76, vec![]);

    builder = builder.with_expression(
        Regex::new(77, r##"thermography.*exactable.*gyrase"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(77, vec![]);

    builder = builder.with_expression(
        Regex::new(
            78,
            r##"outburst[^\r\n]{4,4}squirrel.{5,}dipoles.{9,}margined.{3,}kukri"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(78, vec![]);

    builder = builder.with_expression(
        Regex::new(79, r##"^blackpollssauterneaccorder"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(79, vec![]);

    builder = builder.with_expression(
        Regex::new(80, r##"liquorices.*condones[^\n]*vitta"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(80, vec![]);

    builder = builder.with_expression(
        Regex::new(81, r##"bohemianism.*biform"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(81, vec![]);

    builder = builder.with_expression(
        Regex::new(
            82,
            r##"^outscolded[^\n]*trawlerman[^\n]*tzaddik[^\r\n]{7,16}thatchy[^\r\n]{7,7}collins"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(82, vec![]);

    builder = builder.with_expression(
        Regex::new(
            83,
            r##"^beadily[^\r\n]+speils[^\r\n]+constellationconfiteors"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(83, vec![]);

    builder = builder.with_expression(
        Regex::new(84, r##"ambivalencepizzazzwadersbacitracins"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(84, vec![]);

    builder = builder.with_expression(
        Regex::new(85, r##"freehandednesses"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(85, vec![]);

    builder = builder.with_expression(
        Regex::new(86, r##"hurtle"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        86,
        vec![
            vec![Capture::new(Some(2525324), Some(2525330))],
            vec![Capture::new(Some(2744096), Some(2744102))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(87, r##"whistles.{7,}upsoar.{3,}constipate"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(87, vec![]);

    builder = builder.with_expression(
        Regex::new(88, r##"bronchiinviable.{3,}skags.{5,}berhyming"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(88, vec![]);

    builder = builder.with_expression(
        Regex::new(89, r##"starchinessalmanditesgoannas"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(89, vec![]);

    builder = builder.with_expression(
        Regex::new(90, r##"imagings"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(90, vec![]);

    builder = builder.with_expression(
        Regex::new(
            91,
            r##"resignations.+premillennialism.*retied.*incorruptness.*mauds"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(91, vec![]);

    builder = builder.with_expression(
        Regex::new(92, r##"uphills[^\r\n]{5,}adulate"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(92, vec![]);

    builder = builder.with_expression(
        Regex::new(93, r##"spanscrenels"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(93, vec![]);

    builder = builder.with_expression(
        Regex::new(94, r##"holist[^\n]*doofus[^\n]*rude[^\n]*uncrossing"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(94, vec![]);

    builder = builder.with_expression(
        Regex::new(
            95,
            r##"intergalactic.{1,}macrocytoses.{7,}foredatedsquishy"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(95, vec![]);

    builder = builder.with_expression(
        Regex::new(96, r##"hums"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        96,
        vec![
            vec![Capture::new(Some(2977141), Some(2977145))],
            vec![Capture::new(Some(2991278), Some(2991282))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(97, r##"stalwartness"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(97, vec![]);

    builder = builder.with_expression(
        Regex::new(
            98,
            r##"striate[^\r\n]{1,}electrokinetic[^\r\n]{3,}chernozemic.*whoopie.*radiomen"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(98, vec![]);

    builder = builder.with_expression(
        Regex::new(99, r##"sporocarps.*unpunctuality.*expand.*refinisher"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(99, vec![]);

    builder = builder.with_expression(
        Regex::new(100, r##"rectifiability.*populations[^\r\n]+crock"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(100, vec![]);

    builder = builder.with_expression(
        Regex::new(
            101,
            r##"lengthier[^\n]*aluminize[^\n]*repayment[^\n]*evonymuses.*protonation"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(101, vec![]);

    builder = builder.with_expression(
        Regex::new(102, r##"transfiguredthixotropyreaddingbetta"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(102, vec![]);

    builder = builder.with_expression(
        Regex::new(103, r##"^slimmed"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(103, vec![]);

    builder = builder.with_expression(
        Regex::new(104, r##"brakes"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(104, vec![vec![Capture::new(Some(3482330), Some(3482336))]]);

    builder = builder.with_expression(
        Regex::new(105, r##"asperges"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(105, vec![]);

    builder = builder.with_expression(
        Regex::new(
            106,
            r##"decolonizing.{5,13}uninitiate[^\n]+foremanship[^\n]+stewing"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(106, vec![]);

    builder = builder.with_expression(
        Regex::new(107, r##"ophthalmoscopes.+slays.+tires"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(107, vec![]);

    builder = builder.with_expression(
        Regex::new(
            108,
            r##"dragonets[^\n]*namers.{2,2}empaling.{4,6}confrere[^\r\n]*phosphene"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(108, vec![]);

    builder = builder.with_expression(
        Regex::new(
            109,
            r##"overspreadsradiopaqueguillotinedecadency[^\n]*sulkers"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(109, vec![]);

    builder = builder.with_expression(
        Regex::new(110, r##"airhead.{8,9}acidoses"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(110, vec![]);

    builder = builder.with_expression(
        Regex::new(111, r##"perineumalchemy"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(111, vec![]);

    builder = builder.with_expression(
        Regex::new(
            112,
            r##"vestally.*conjugality.*exteriorising.*glims.*blinding"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(112, vec![]);

    builder = builder.with_expression(
        Regex::new(113, r##"toped"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(113, vec![]);

    builder = builder.with_expression(
        Regex::new(114, r##"skedaddled"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(114, vec![]);

    builder = builder.with_expression(
        Regex::new(115, r##"basseting.*conelrads.*denominationalisms"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(115, vec![]);

    builder = builder.with_expression(
        Regex::new(116, r##"crocheters[^\r\n]*heteroecious[^\r\n]*overthinks"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(116, vec![]);

    builder = builder.with_expression(
        Regex::new(117, r##"currycombedboskier.*bollixes.*muskiest"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(117, vec![]);

    builder = builder.with_expression(
        Regex::new(
            118,
            r##"platinizes.*necrophilisms.*dimeter.*defrosting.*lepidopterist"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(118, vec![]);

    builder = builder.with_expression(
        Regex::new(119, r##"thunderers"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(119, vec![]);

    builder = builder.with_expression(
        Regex::new(120, r##"backrushes.*czarevitches"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(120, vec![]);

    builder = builder.with_expression(
        Regex::new(121, r##"eloigned"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(121, vec![]);

    builder = builder.with_expression(
        Regex::new(122, r##"sashed.*knells.*graciouscircumstellar"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(122, vec![]);

    builder = builder.with_expression(
        Regex::new(123, r##"^farseeing.*skeletons.*secularise"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(123, vec![]);

    builder = builder.with_expression(
        Regex::new(124, r##"finerwhiskschangeless[^\r\n]+biders"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(124, vec![]);

    builder = builder.with_expression(
        Regex::new(125, r##"druse"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(125, vec![]);

    builder = builder.with_expression(
        Regex::new(126, r##"heths.*heterochromatic.*unsaddleisoenzyme"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(126, vec![]);

    builder = builder.with_expression(
        Regex::new(
            127,
            r##"kilomole.{5,}rundle.{3,}costards.{6,}oniony.{10,}epistles"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(127, vec![]);

    builder = builder.with_expression(
        Regex::new(
            128,
            r##"toweringspectroheliogramsemulousnesseskindergartners"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(128, vec![]);

    builder = builder.with_expression(
        Regex::new(
            129,
            r##"incorporable.*hostelries.*syngases.{1,3}deductions.*poacher"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(129, vec![]);

    builder = builder.with_expression(
        Regex::new(130, r##"^sailplanermisdraws"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(130, vec![]);

    builder = builder.with_expression(
        Regex::new(131, r##"circumvallates"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(131, vec![]);

    builder = builder.with_expression(
        Regex::new(132, r##"pyrrhotites"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(132, vec![]);

    builder = builder.with_expression(
        Regex::new(
            133,
            r##"immunise[^\r\n]{7,8}podophyllin[^\r\n]{5,12}scarifierspalatine.*semiporcelains"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(133, vec![]);

    builder = builder.with_expression(
        Regex::new(
            134,
            r##"kellies[^\n]*acclimatizers[^\n]*footboard[^\n]*gremial[^\n]*armours"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(134, vec![]);

    builder = builder.with_expression(
        Regex::new(
            135,
            r##"counterinstitutions[^\n]*mulattos[^\r\n]+overtires"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(135, vec![]);

    builder = builder.with_expression(
        Regex::new(136, r##"feudists[^\r\n]+followups[^\r\n]+grandmotherly"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(136, vec![]);

    builder = builder.with_expression(
        Regex::new(137, r##"guild"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        137,
        vec![
            vec![Capture::new(Some(569394), Some(569399))],
            vec![Capture::new(Some(616408), Some(616413))],
            vec![Capture::new(Some(1020849), Some(1020854))],
            vec![Capture::new(Some(1070287), Some(1070292))],
            vec![Capture::new(Some(1070351), Some(1070356))],
            vec![Capture::new(Some(1071862), Some(1071867))],
            vec![Capture::new(Some(1071893), Some(1071898))],
            vec![Capture::new(Some(1072211), Some(1072216))],
            vec![Capture::new(Some(1081340), Some(1081345))],
            vec![Capture::new(Some(1081678), Some(1081683))],
            vec![Capture::new(Some(1084575), Some(1084580))],
            vec![Capture::new(Some(1089027), Some(1089032))],
            vec![Capture::new(Some(1097033), Some(1097038))],
            vec![Capture::new(Some(1100333), Some(1100338))],
            vec![Capture::new(Some(1101681), Some(1101686))],
            vec![Capture::new(Some(1112279), Some(1112284))],
            vec![Capture::new(Some(1114546), Some(1114551))],
            vec![Capture::new(Some(1124956), Some(1124961))],
            vec![Capture::new(Some(1129589), Some(1129594))],
            vec![Capture::new(Some(1146022), Some(1146027))],
            vec![Capture::new(Some(1146273), Some(1146278))],
            vec![Capture::new(Some(1147569), Some(1147574))],
            vec![Capture::new(Some(1147619), Some(1147624))],
            vec![Capture::new(Some(1147935), Some(1147940))],
            vec![Capture::new(Some(1148628), Some(1148633))],
            vec![Capture::new(Some(1150708), Some(1150713))],
            vec![Capture::new(Some(1150766), Some(1150771))],
            vec![Capture::new(Some(1152826), Some(1152831))],
            vec![Capture::new(Some(1154421), Some(1154426))],
            vec![Capture::new(Some(1170465), Some(1170470))],
            vec![Capture::new(Some(1199009), Some(1199014))],
            vec![Capture::new(Some(1215042), Some(1215047))],
            vec![Capture::new(Some(2184495), Some(2184500))],
            vec![Capture::new(Some(2214479), Some(2214484))],
            vec![Capture::new(Some(2214866), Some(2214871))],
            vec![Capture::new(Some(2214879), Some(2214884))],
            vec![Capture::new(Some(2215426), Some(2215431))],
            vec![Capture::new(Some(4122054), Some(4122059))],
            vec![Capture::new(Some(4123455), Some(4123460))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(
            138,
            r##"modulate[^\r\n]*frivolousnesses[^\r\n]*oversimplify"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(138, vec![]);

    builder = builder.with_expression(
        Regex::new(139, r##"gaslights"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(139, vec![]);

    builder = builder.with_expression(
        Regex::new(140, r##"thunderstroke"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(140, vec![vec![Capture::new(Some(4556028), Some(4556041))]]);

    builder = builder.with_expression(
        Regex::new(
            141,
            r##"wirers[^\n]*hexokinase[^\n]*trachoma[^\n]*unclips"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(141, vec![]);

    builder = builder.with_expression(
        Regex::new(142, r##"leishmanial.{5,12}gangsterish"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(142, vec![]);

    builder = builder.with_expression(
        Regex::new(143, r##"kiefs"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(143, vec![]);

    builder = builder.with_expression(
        Regex::new(144, r##"eroses.*removals.*rodmen.*solarizing"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(144, vec![]);

    builder = builder.with_expression(
        Regex::new(145, r##"communicative"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(145, vec![]);

    builder = builder.with_expression(
        Regex::new(146, r##"unbecoming"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(146, vec![vec![Capture::new(Some(2968169), Some(2968179))]]);

    builder = builder.with_expression(
        Regex::new(147, r##"phonotactic[^\r\n]*pitfall"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(147, vec![]);

    builder = builder.with_expression(
        Regex::new(148, r##"alohasdower"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(148, vec![]);

    builder = builder.with_expression(
        Regex::new(149, r##"clasptambushlehayimblastemaratels"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(149, vec![]);

    builder = builder.with_expression(
        Regex::new(
            150,
            r##"steppingstone[^\n]{9,16}pistaches[^\n]{7,10}electrolyte[^\n]{9,}dad.*networkings"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(150, vec![]);

    builder = builder.with_expression(
        Regex::new(
            151,
            r##"beggarweeds.*avoiding.*irately.{9,17}underfinanced"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(151, vec![]);

    builder = builder.with_expression(
        Regex::new(152, r##"coverslips.*stewpandisciplinersstentors"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(152, vec![]);

    builder = builder.with_expression(
        Regex::new(153, r##"idolators.*antiwearinterpenetrating"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(153, vec![]);

    builder = builder.with_expression(
        Regex::new(154, r##"oogamete[^\n]{5,14}gaolerlye.*pronucleuses"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(154, vec![]);

    builder = builder.with_expression(
        Regex::new(155, r##"nesters[^\n]*ameliorator.*syncretistic"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(155, vec![]);

    builder = builder.with_expression(
        Regex::new(156, r##"catcherspoltergeistdrainage"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(156, vec![]);

    builder = builder.with_expression(
        Regex::new(
            157,
            r##"outkissed[^\r\n]{3,}ankerite[^\r\n]{3,}stopper[^\n]+degenerateness"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(157, vec![]);

    builder = builder.with_expression(
        Regex::new(158, r##"upswelled"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(158, vec![]);

    builder = builder.with_expression(
        Regex::new(159, r##"recolored"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(159, vec![]);

    builder = builder.with_expression(
        Regex::new(
            160,
            r##"((nonobscene)|(ginnier)|(garblers)|(insouciantly))"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(160, vec![]);

    builder = builder.with_expression(
        Regex::new(
            161,
            r##"auriculate.*bowling.*inholding.*exilic.*anthropologists"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(161, vec![]);

    builder = builder.with_expression(
        Regex::new(162, r##"deaden.*cottages.*antiwoman"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(162, vec![]);

    builder = builder.with_expression(
        Regex::new(
            163,
            r##"tailleur.*acknowledgement.*illimitably.*carboxylation"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(163, vec![]);

    builder = builder.with_expression(
        Regex::new(164, r##"gilberts"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(164, vec![]);

    builder = builder.with_expression(
        Regex::new(165, r##"radicalising.+pemmican"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(165, vec![]);

    builder = builder.with_expression(
        Regex::new(166, r##"cayenned"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(166, vec![]);

    builder = builder.with_expression(
        Regex::new(
            167,
            r##"demolitionists[^\r\n]*supplementing.+zootiest.+psychosurgeons"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(167, vec![]);

    builder = builder.with_expression(
        Regex::new(168, r##"dicier.*guanosine.*shy"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(168, vec![]);

    builder = builder.with_expression(
        Regex::new(169, r##"calmest"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(169, vec![vec![Capture::new(Some(1447374), Some(1447381))]]);

    builder = builder.with_expression(
        Regex::new(170, r##"plinks"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(170, vec![]);

    builder = builder.with_expression(
        Regex::new(
            171,
            r##"envoy.*boxboards.*reinsures.*impermanences[^\r\n]{7,7}sleuthhounds"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(171, vec![]);

    builder = builder.with_expression(
        Regex::new(172, r##"keyed[^\n]{4,14}streamier"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(172, vec![]);

    builder = builder.with_expression(
        Regex::new(
            173,
            r##"backhoes.{8,}anlage.{6,}rewakens.*afferent.*tasty"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(173, vec![]);

    builder = builder.with_expression(
        Regex::new(174, r##"iterativedownstrokes[^\r\n]{5,13}fielder"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(174, vec![]);

    builder = builder.with_expression(
        Regex::new(175, r##"hereditarian.*chapmen"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(175, vec![]);

    builder = builder.with_expression(
        Regex::new(176, r##"ferritin.*ranknessgoddamns.*waffies"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(176, vec![]);

    builder = builder.with_expression(
        Regex::new(
            177,
            r##"((nuclide.*thermoelements.*phonotactic.*microporosity)|(silts))"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(177, vec![]);

    builder = builder.with_expression(
        Regex::new(
            178,
            r##"peremptorinessesbourgeoisifyingstringencydeodorized"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(178, vec![]);

    builder = builder.with_expression(
        Regex::new(179, r##"bittings.{3,}finalizations"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(179, vec![]);

    builder = builder.with_expression(
        Regex::new(180, r##"nester"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        180,
        vec![
            vec![Capture::new(Some(195217), Some(195223))],
            vec![Capture::new(Some(787640), Some(787646))],
            vec![Capture::new(Some(1445128), Some(1445134))],
            vec![Capture::new(Some(1988529), Some(1988535))],
            vec![Capture::new(Some(3641516), Some(3641522))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(
            181,
            r##"breakouts.+motivate.*surcingles.*smudgiest.*nominative"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(181, vec![]);

    builder = builder.with_expression(
        Regex::new(182, r##"politicalizes.*pensive.{1,8}fishpoles"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(182, vec![]);

    builder = builder.with_expression(
        Regex::new(183, r##"falsification"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(183, vec![]);

    builder = builder.with_expression(
        Regex::new(184, r##"ridiculing.*neoclassicist"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(184, vec![]);

    builder = builder.with_expression(
        Regex::new(185, r##"forsaker.*erns.*allegiance.*atypically"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(185, vec![]);

    builder = builder.with_expression(
        Regex::new(186, r##"analcime[^\n]*bounders[^\n]*bendayed"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(186, vec![]);

    builder = builder.with_expression(
        Regex::new(187, r##"incommensurate[^\r\n]*cryptonyms"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(187, vec![]);

    builder = builder.with_expression(
        Regex::new(188, r##"((pitching.*sheeters)|(canzona)|(fizzier))demote"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(188, vec![]);

    builder = builder.with_expression(
        Regex::new(189, r##"bungled.*retroact"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(189, vec![]);

    builder = builder.with_expression(
        Regex::new(
            190,
            r##"acyclic.*sublicense.*ploys.*leptotene.*stigmatized"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(190, vec![]);

    builder = builder.with_expression(
        Regex::new(191, r##"paleobotany.+grinners.+thereunder.+pantothenates"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(191, vec![]);

    builder = builder.with_expression(
        Regex::new(192, r##"^conifer"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(192, vec![]);

    builder = builder.with_expression(
        Regex::new(193, r##"impolite"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(193, vec![]);

    builder = builder.with_expression(
        Regex::new(194, r##"paginations.*chives.*kraals.*chipping.*hustler"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(194, vec![]);

    builder = builder.with_expression(
        Regex::new(195, r##"localities.*redear"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(195, vec![]);

    builder = builder.with_expression(
        Regex::new(196, r##"^incorporate.*billionths"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(196, vec![]);

    builder = builder.with_expression(
        Regex::new(197, r##"feoffee.+relied.+spans.+ragtag.+joypoppers"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(197, vec![]);

    builder = builder.with_expression(
        Regex::new(
            198,
            r##"superheated.*stored.*stactes[^\n]*softback[^\n]*decidability"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(198, vec![]);

    builder = builder.with_expression(
        Regex::new(199, r##"((overs)|(detoxicating))"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        199,
        vec![
            vec![Capture::new(Some(31223), Some(31228))],
            vec![Capture::new(Some(47513), Some(47518))],
            vec![Capture::new(Some(94355), Some(94360))],
            vec![Capture::new(Some(211869), Some(211874))],
            vec![Capture::new(Some(472464), Some(472469))],
            vec![Capture::new(Some(510278), Some(510283))],
            vec![Capture::new(Some(510355), Some(510360))],
            vec![Capture::new(Some(516904), Some(516909))],
            vec![Capture::new(Some(528047), Some(528052))],
            vec![Capture::new(Some(543338), Some(543343))],
            vec![Capture::new(Some(555016), Some(555021))],
            vec![Capture::new(Some(640620), Some(640625))],
            vec![Capture::new(Some(689129), Some(689134))],
            vec![Capture::new(Some(707753), Some(707758))],
            vec![Capture::new(Some(708048), Some(708053))],
            vec![Capture::new(Some(867426), Some(867431))],
            vec![Capture::new(Some(898081), Some(898086))],
            vec![Capture::new(Some(913512), Some(913517))],
            vec![Capture::new(Some(962721), Some(962726))],
            vec![Capture::new(Some(962739), Some(962744))],
            vec![Capture::new(Some(990894), Some(990899))],
            vec![Capture::new(Some(1087739), Some(1087744))],
            vec![Capture::new(Some(1425034), Some(1425039))],
            vec![Capture::new(Some(1466154), Some(1466159))],
            vec![Capture::new(Some(1486405), Some(1486410))],
            vec![Capture::new(Some(1495748), Some(1495753))],
            vec![Capture::new(Some(1499508), Some(1499513))],
            vec![Capture::new(Some(1590819), Some(1590824))],
            vec![Capture::new(Some(1590867), Some(1590872))],
            vec![Capture::new(Some(1628306), Some(1628311))],
            vec![Capture::new(Some(2057908), Some(2057913))],
            vec![Capture::new(Some(2327419), Some(2327424))],
            vec![Capture::new(Some(2347702), Some(2347707))],
            vec![Capture::new(Some(2401814), Some(2401819))],
            vec![Capture::new(Some(2407557), Some(2407562))],
            vec![Capture::new(Some(2488460), Some(2488465))],
            vec![Capture::new(Some(2550643), Some(2550648))],
            vec![Capture::new(Some(2592190), Some(2592195))],
            vec![Capture::new(Some(2798817), Some(2798822))],
            vec![Capture::new(Some(2810195), Some(2810200))],
            vec![Capture::new(Some(2810234), Some(2810239))],
            vec![Capture::new(Some(2815446), Some(2815451))],
            vec![Capture::new(Some(2851808), Some(2851813))],
            vec![Capture::new(Some(2856614), Some(2856619))],
            vec![Capture::new(Some(2876124), Some(2876129))],
            vec![Capture::new(Some(3044550), Some(3044555))],
            vec![Capture::new(Some(3167227), Some(3167232))],
            vec![Capture::new(Some(3222450), Some(3222455))],
            vec![Capture::new(Some(3224112), Some(3224117))],
            vec![Capture::new(Some(3278294), Some(3278299))],
            vec![Capture::new(Some(3461044), Some(3461049))],
            vec![Capture::new(Some(3464055), Some(3464060))],
            vec![Capture::new(Some(3464579), Some(3464584))],
            vec![Capture::new(Some(3520076), Some(3520081))],
            vec![Capture::new(Some(3520920), Some(3520925))],
            vec![Capture::new(Some(3530944), Some(3530949))],
            vec![Capture::new(Some(3535131), Some(3535136))],
            vec![Capture::new(Some(3540231), Some(3540236))],
            vec![Capture::new(Some(3540355), Some(3540360))],
            vec![Capture::new(Some(3541519), Some(3541524))],
            vec![Capture::new(Some(3546932), Some(3546937))],
            vec![Capture::new(Some(3547182), Some(3547187))],
            vec![Capture::new(Some(3547814), Some(3547819))],
            vec![Capture::new(Some(3548328), Some(3548333))],
            vec![Capture::new(Some(3548576), Some(3548581))],
            vec![Capture::new(Some(3556518), Some(3556523))],
            vec![Capture::new(Some(3557806), Some(3557811))],
            vec![Capture::new(Some(3608938), Some(3608943))],
            vec![Capture::new(Some(3720919), Some(3720924))],
            vec![Capture::new(Some(3805869), Some(3805874))],
            vec![Capture::new(Some(4210717), Some(4210722))],
            vec![Capture::new(Some(4221070), Some(4221075))],
            vec![Capture::new(Some(4221122), Some(4221127))],
            vec![Capture::new(Some(4237281), Some(4237286))],
            vec![Capture::new(Some(4247883), Some(4247888))],
            vec![Capture::new(Some(4254860), Some(4254865))],
            vec![Capture::new(Some(4258647), Some(4258652))],
            vec![Capture::new(Some(4291779), Some(4291784))],
            vec![Capture::new(Some(4315872), Some(4315877))],
            vec![Capture::new(Some(4487052), Some(4487057))],
            vec![Capture::new(Some(4584355), Some(4584360))],
            vec![Capture::new(Some(4603595), Some(4603600))],
            vec![Capture::new(Some(4755171), Some(4755176))],
            vec![Capture::new(Some(4766538), Some(4766543))],
            vec![Capture::new(Some(4787153), Some(4787158))],
            vec![Capture::new(Some(4907263), Some(4907268))],
            vec![Capture::new(Some(4947600), Some(4947605))],
            vec![Capture::new(Some(4949298), Some(4949303))],
            vec![Capture::new(Some(4953624), Some(4953629))],
            vec![Capture::new(Some(5081287), Some(5081292))],
            vec![Capture::new(Some(5089267), Some(5089272))],
            vec![Capture::new(Some(5101947), Some(5101952))],
            vec![Capture::new(Some(5164333), Some(5164338))],
            vec![Capture::new(Some(5201432), Some(5201437))],
            vec![Capture::new(Some(5208770), Some(5208775))],
            vec![Capture::new(Some(5240827), Some(5240832))],
            vec![Capture::new(Some(5270264), Some(5270269))],
            vec![Capture::new(Some(5435090), Some(5435095))],
            vec![Capture::new(Some(5447545), Some(5447550))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(
            200,
            r##"sobering.{1,}recoilless.{8,}ionic.{8,}loreuncultured"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(200, vec![]);

    builder = builder.with_expression(
        Regex::new(201, r##"suppler.{3,}neoclassicisms.{10,}deciduousnesses"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(201, vec![]);

    builder = builder.with_expression(
        Regex::new(202, r##"accidences"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(202, vec![]);

    builder = builder.with_expression(
        Regex::new(203, r##"trumpingskepticismengaginglyrangersupstir"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(203, vec![]);

    builder = builder.with_expression(
        Regex::new(
            204,
            r##"lastly[^\n]{2,}guan[^\n]{1,}adages[^\n]{4,}tenons"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(204, vec![]);

    builder = builder.with_expression(
        Regex::new(205, r##"strategicbiodegradation"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(205, vec![]);

    builder = builder.with_expression(
        Regex::new(206, r##"contestants[^\r\n]+lams.*proa[^\r\n]{8,}playtime"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(206, vec![]);

    builder = builder.with_expression(
        Regex::new(
            207,
            r##"autoworker.{7,}eucaryote.{8,}occiputs.{6,}megaloblasts"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(207, vec![]);

    builder = builder.with_expression(Regex::new(208, r##"deoxidizing[^\n]{5,}mineralizations[^\n]{3,}erelong[^\n]{6,11}homelessness.*catacomb"##).
        mode(MatchMode::All(Submatch::Expression)).encoding(Encoding::Byte).case_sensitive(false).build()?);
    expected.insert(208, vec![]);

    builder = builder.with_expression(
        Regex::new(209, r##"dines.*resentences.+malt.+furloughs"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(209, vec![]);

    builder = builder.with_expression(
        Regex::new(210, r##"accompanies.*oasthouses.+giblet.+overscrupulous"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(210, vec![]);

    builder = builder.with_expression(
        Regex::new(
            211,
            r##"retch.{4,}prototypically.*recombed.*prongs[^\n]{5,15}jutties"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(211, vec![]);

    builder = builder.with_expression(
        Regex::new(212, r##"^generalityboisterous.*bureaucratizes.*marquise"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(212, vec![]);

    builder = builder.with_expression(
        Regex::new(213, r##"rotche[^\r\n]*peregrinates"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(213, vec![]);

    builder = builder.with_expression(
        Regex::new(214, r##"shudder"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(214, vec![vec![Capture::new(Some(4697483), Some(4697490))]]);

    builder = builder.with_expression(
        Regex::new(215, r##"youngest[^\n]{2,}beeswing.+splenomegalies"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(215, vec![]);

    builder = builder.with_expression(
        Regex::new(216, r##"ominous"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        216,
        vec![
            vec![Capture::new(Some(500672), Some(500679))],
            vec![Capture::new(Some(1092491), Some(1092498))],
            vec![Capture::new(Some(1808426), Some(1808433))],
            vec![Capture::new(Some(2092374), Some(2092381))],
            vec![Capture::new(Some(3682083), Some(3682090))],
            vec![Capture::new(Some(4111672), Some(4111679))],
            vec![Capture::new(Some(4139866), Some(4139873))],
            vec![Capture::new(Some(5025060), Some(5025067))],
            vec![Capture::new(Some(5028282), Some(5028289))],
            vec![Capture::new(Some(5038833), Some(5038840))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(217, r##"chlorella[^\n]*overprescriptionshunpiking"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(217, vec![]);

    builder = builder.with_expression(
        Regex::new(218, r##"sapota"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(218, vec![]);

    builder = builder.with_expression(
        Regex::new(
            219,
            r##"replicase.*doles.*synchrotrons.*persists.*acculturates"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(219, vec![]);

    builder = builder.with_expression(
        Regex::new(
            220,
            r##"bumblings.*lockers.*codgers.*folacins.*distensibilities"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(220, vec![]);

    builder = builder.with_expression(
        Regex::new(
            221,
            r##"gloated[^\n]*shooflies[^\n]*pikas[^\n]*berylliums"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(221, vec![]);

    builder = builder.with_expression(
        Regex::new(222, r##"miffedvinblastineelutionsexorbitancesooted"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(222, vec![]);

    builder = builder.with_expression(
        Regex::new(223, r##"bedtimes.{9,}convertibleness.{9,}verticillate"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(223, vec![]);

    builder = builder.with_expression(
        Regex::new(224, r##"^stabilizers.*wink.*impoliteness"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(224, vec![]);

    builder = builder.with_expression(
        Regex::new(225, r##"biochips"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(225, vec![]);

    builder = builder.with_expression(
        Regex::new(226, r##"flyable.*mignons.*intercut[^\n]*displacement"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(226, vec![]);

    builder = builder.with_expression(
        Regex::new(
            227,
            r##"outblazing.+transshapes.+globalising[^\n]+dilettantism"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(227, vec![]);

    builder = builder.with_expression(
        Regex::new(228, r##"^((scalages)|(quadrillionth)|(amphibiousness))"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(228, vec![]);

    builder = builder.with_expression(
        Regex::new(229, r##"amphetamine"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(229, vec![]);

    builder = builder.with_expression(
        Regex::new(230, r##"chokercopeddecoupagestablematemagnetometries"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(230, vec![]);

    builder = builder.with_expression(
        Regex::new(231, r##"slugdiamondingbhangs"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(231, vec![]);

    builder = builder.with_expression(
        Regex::new(232, r##"hardhead.*famed.*hartshorn.{7,}sardana"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(232, vec![]);

    builder = builder.with_expression(
        Regex::new(233, r##"nonesuchescodesign.*figuring"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(233, vec![]);

    builder = builder.with_expression(
        Regex::new(234, r##"outthinks.*hemes"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(234, vec![]);

    builder = builder.with_expression(
        Regex::new(235, r##"nurturances.*nebulising"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(235, vec![]);

    builder = builder.with_expression(
        Regex::new(236, r##"forechecked.*outarguespushed"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(236, vec![]);

    builder = builder.with_expression(
        Regex::new(237, r##"bastardizingabysmsaiglet"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(237, vec![]);

    builder = builder.with_expression(
        Regex::new(
            238,
            r##"johnnycake[^\n]*lionizing[^\n]*flyspecking[^\n]*clime"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(238, vec![]);

    builder = builder.with_expression(
        Regex::new(
            239,
            r##"olfactory[^\r\n]*statical[^\r\n]*stern[^\r\n]*indignant"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(239, vec![]);

    builder = builder.with_expression(
        Regex::new(
            240,
            r##"croupes.*munting.*gemmologies.*counselorships.*vitta"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(240, vec![]);

    builder = builder.with_expression(
        Regex::new(241, r##"thiaminase"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(241, vec![]);

    builder = builder.with_expression(
        Regex::new(242, r##"spavined"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(242, vec![]);

    builder = builder.with_expression(
        Regex::new(
            243,
            r##"wallboards[^\n]{1,10}understeers[^\n]{8,8}exorcisersboulders"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(243, vec![]);

    builder = builder.with_expression(
        Regex::new(244, r##"abortifacient.+laxatives"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(244, vec![]);

    builder = builder.with_expression(
        Regex::new(245, r##"mislearned[^\n]{6,}cleidoic"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(245, vec![]);

    builder = builder.with_expression(
        Regex::new(246, r##"crisscrosses.*dethroner.*hexamine"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(246, vec![]);

    builder = builder.with_expression(
        Regex::new(247, r##"sanenessesexodermsdispense"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(247, vec![]);

    builder = builder.with_expression(
        Regex::new(248, r##"postcardlike"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(248, vec![]);

    builder = builder.with_expression(
        Regex::new(
            249,
            r##"larvicide[^\r\n]+dicoumarin[^\r\n]+yapping.{5,}telexing"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(249, vec![]);

    builder = builder.with_expression(
        Regex::new(250, r##"((dandiest)|(bandsmen))"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(250, vec![]);

    builder = builder.with_expression(
        Regex::new(
            251,
            r##"plasmodesma.+bombarded.+sawtooth.+bravuras.+customise"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(251, vec![]);

    builder = builder.with_expression(
        Regex::new(252, r##"wrongdoing[^\n]+lifeboats[^\n]+dice.*velites"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(252, vec![]);

    builder = builder.with_expression(
        Regex::new(253, r##"feeblemindedlygaucher"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(253, vec![]);

    builder = builder.with_expression(
        Regex::new(254, r##"ligroines.{8,}bunts.{2,}luetic"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(254, vec![]);

    builder = builder.with_expression(
        Regex::new(
            255,
            r##"snugnesses[^\n]{3,}nonphysical[^\n]{2,}independentanviltops"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(255, vec![]);

    builder = builder.with_expression(
        Regex::new(
            256,
            r##"((moshing)|(cycadophytes)).*sexiest.{8,18}condylomaphonocardiograms"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(256, vec![]);

    builder = builder.with_expression(
        Regex::new(
            257,
            r##"transpiercedboyishnesstransmittableportersdements"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(257, vec![]);

    builder = builder.with_expression(
        Regex::new(
            258,
            r##"anthrax.*misrelates.*epenthetic[^\n]*precapitalists"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(258, vec![]);

    builder = builder.with_expression(
        Regex::new(259, r##"reason[^\r\n]+pilis[^\r\n]+inclosed"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(259, vec![]);

    builder = builder.with_expression(
        Regex::new(260, r##"skirret.*corban.*prisonerboschbokquivery"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(260, vec![]);

    builder = builder.with_expression(
        Regex::new(261, r##"jimsonweeds"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(261, vec![]);

    builder = builder.with_expression(
        Regex::new(262, r##"^peripherals"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(262, vec![]);

    builder = builder.with_expression(
        Regex::new(263, r##"begazevorticistsbiotoxinsbiopics.*hamadryades"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(263, vec![]);

    builder = builder.with_expression(
        Regex::new(264, r##"immortelle.*spokeshave.*snores"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(264, vec![]);

    builder = builder.with_expression(
        Regex::new(265, r##"backlashersrememberscarecrowpayeespropelled"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(265, vec![]);

    builder = builder.with_expression(
        Regex::new(
            266,
            r##"ticals.{6,}anilins.{7,}orchestraters.+superelevations.*shadblow"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(266, vec![]);

    builder = builder.with_expression(
        Regex::new(
            267,
            r##"unsexy.*apotheosize.*rhizoidal.*phototropism.*sprawled"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(267, vec![]);

    builder = builder.with_expression(
        Regex::new(268, r##"copartner"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(268, vec![]);

    builder = builder.with_expression(
        Regex::new(269, r##"matriarchy.+coadjutrices"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(269, vec![]);

    builder = builder.with_expression(
        Regex::new(270, r##"hemangioma[^\n]+laud[^\n]+abstain.*yield"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(270, vec![]);

    builder = builder.with_expression(
        Regex::new(271, r##"prosaisms.*unexplainable.*forego"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(271, vec![]);

    builder = builder.with_expression(
        Regex::new(
            272,
            r##"milkwortschiralitydisciplinarian.+vanguards[^\n]{4,7}sorority"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(272, vec![]);

    builder = builder.with_expression(
        Regex::new(273, r##"knish"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(273, vec![]);

    builder = builder.with_expression(
        Regex::new(274, r##"^indignantly"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(274, vec![]);

    builder = builder.with_expression(
        Regex::new(275, r##"frivollerstriadics"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(275, vec![]);

    builder = builder.with_expression(
        Regex::new(276, r##"infraspecificinviolable"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(276, vec![]);

    builder = builder.with_expression(
        Regex::new(277, r##"parlours.*splining"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(277, vec![]);

    builder = builder.with_expression(
        Regex::new(278, r##"premaxillarysandinesses"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(278, vec![]);

    builder = builder.with_expression(
        Regex::new(279, r##"indicate"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(279, vec![vec![Capture::new(Some(6421), Some(6429))]]);

    builder = builder.with_expression(Regex::new(280, r##"artificially[^\r\n]{9,}destabilizations[^\n]*discords[^\n]*prehistorical[^\n]*indow"##).
        mode(MatchMode::All(Submatch::Expression)).encoding(Encoding::Byte).case_sensitive(false).build()?);
    expected.insert(280, vec![]);

    builder = builder.with_expression(
        Regex::new(281, r##"laryngeals[^\r\n]*refreshment[^\r\n]*starting"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(281, vec![]);

    builder = builder.with_expression(
        Regex::new(282, r##"aurei"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(282, vec![]);

    builder = builder.with_expression(
        Regex::new(
            283,
            r##"altogether.*raciness[^\r\n]{1,}subjugators[^\r\n]{9,}ostensorium.{9,}statocyst"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(283, vec![]);

    builder = builder.with_expression(
        Regex::new(284, r##"((necropsies)|(getup)).{10,}scraigh"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(284, vec![]);

    builder = builder.with_expression(
        Regex::new(285, r##"reafforestation"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(285, vec![]);

    builder = builder.with_expression(
        Regex::new(286, r##"fringingdullest"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(286, vec![]);

    builder = builder.with_expression(
        Regex::new(287, r##"jeopardizing"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(287, vec![]);

    builder = builder.with_expression(
        Regex::new(288, r##"^preoccupancy"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(288, vec![]);

    builder = builder.with_expression(
        Regex::new(
            289,
            r##"budlike[^\r\n]*untranslated[^\n]+radiators[^\n]+endearing"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(289, vec![]);

    builder = builder.with_expression(
        Regex::new(290, r##"outrunschalcogenideshempencathodallylocal"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(290, vec![]);

    builder = builder.with_expression(
        Regex::new(291, r##"misunderstand.*sedimentologist"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(291, vec![]);

    builder = builder.with_expression(
        Regex::new(292, r##"dampen.*remembrancer"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(292, vec![]);

    builder = builder.with_expression(
        Regex::new(293, r##"smoky"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        293,
        vec![
            vec![Capture::new(Some(187048), Some(187053))],
            vec![Capture::new(Some(877033), Some(877038))],
            vec![Capture::new(Some(1300689), Some(1300694))],
            vec![Capture::new(Some(1330745), Some(1330750))],
            vec![Capture::new(Some(1446507), Some(1446512))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(294, r##"reg.{10,}institutional.{5,}flannelette"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(294, vec![]);

    builder = builder.with_expression(
        Regex::new(295, r##"filarian.+detoxify"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(295, vec![]);

    builder = builder.with_expression(
        Regex::new(296, r##"attachers[^\n]*mbiras[^\n]*knottiness"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(296, vec![]);

    builder = builder.with_expression(
        Regex::new(
            297,
            r##"connectedness.{6,}trivializing.{7,}stranglers.*remotivate[^\r\n]*preeminent"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(297, vec![]);

    builder = builder.with_expression(
        Regex::new(298, r##"mitises"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(298, vec![]);

    builder = builder.with_expression(
        Regex::new(
            299,
            r##"ptyalins.{6,}splurge[^\n]*subcentral[^\n]*healer.{6,14}caryotin"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(299, vec![]);

    builder = builder.with_expression(
        Regex::new(300, r##"cuing"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(300, vec![vec![Capture::new(Some(2046141), Some(2046146))]]);

    builder = builder.with_expression(
        Regex::new(301, r##"pec[^\r\n]{10,13}multiethnic"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(301, vec![]);

    builder = builder.with_expression(
        Regex::new(302, r##"regressor.+rootages.*reremindingeructations"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(302, vec![]);

    builder = builder.with_expression(Regex::new(303, r##"unecological[^\n]{8,}bathhouse[^\n]{4,}irrevocablenesses[^\n]{10,}jurisprudential"##).
        mode(MatchMode::All(Submatch::Expression)).encoding(Encoding::Byte).case_sensitive(false).build()?);
    expected.insert(303, vec![]);

    builder = builder.with_expression(
        Regex::new(304, r##"rumpliermiswordingunenlightened"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(304, vec![]);

    builder = builder.with_expression(
        Regex::new(
            305,
            r##"philadelphus.*subdevelopment.*cruisers.*scrutinies"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(305, vec![]);

    builder = builder.with_expression(
        Regex::new(306, r##"sully.+hotel.+chaffiest.+waterlog"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(306, vec![]);

    builder = builder.with_expression(
        Regex::new(307, r##"grilles.*overnourishing.{9,15}interferometers"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(307, vec![]);

    builder = builder.with_expression(
        Regex::new(
            308,
            r##"recrystallized.*shamanisms.*chivvying.*unfree.*phon"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(308, vec![]);

    builder = builder.with_expression(
        Regex::new(309, r##"sulu"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(309, vec![]);

    builder = builder.with_expression(
        Regex::new(
            310,
            r##"barghests.*hardhacks.*propended[^\r\n]+bedriveling"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(310, vec![]);

    builder = builder.with_expression(
        Regex::new(311, r##"muscids[^\n]*statements.*marbleizes.*scandalled"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(311, vec![]);

    builder = builder.with_expression(
        Regex::new(312, r##"^indirectness.*bourgeoise[^\r\n]{3,12}hypopyon"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(312, vec![]);

    builder = builder.with_expression(
        Regex::new(313, r##"haptene"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(313, vec![]);

    builder = builder.with_expression(
        Regex::new(314, r##"cementer.*headhunts.*cryptologists"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(314, vec![]);

    builder = builder.with_expression(
        Regex::new(315, r##"reemploytweetersbaldachinunrequitedpygmies"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(315, vec![]);

    builder = builder.with_expression(
        Regex::new(
            316,
            r##"chumminesses[^\n]{6,14}cramp.{10,}citrullines.{7,}rabat.{10,}brawest"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(316, vec![]);

    builder = builder.with_expression(
        Regex::new(
            317,
            r##"^autotype.*centrioles.*hibernations.*endocrinology.*agapanthuses"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(317, vec![]);

    builder = builder.with_expression(Regex::new(318, r##"colloquiality[^\r\n]{9,}cockatoos[^\r\n]{3,}synoptically[^\n]+biasing[^\n]+debeaking"##).
        mode(MatchMode::All(Submatch::Expression)).encoding(Encoding::Byte).case_sensitive(false).build()?);
    expected.insert(318, vec![]);

    builder = builder.with_expression(
        Regex::new(
            319,
            r##"humanised[^\r\n]{7,16}articulation[^\n]{5,13}rained"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(319, vec![]);

    builder = builder.with_expression(
        Regex::new(
            320,
            r##"nonflammabilitiespericopegypsied.*mantlets.*vulgarer"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(320, vec![]);

    builder = builder.with_expression(
        Regex::new(
            321,
            r##"immenser.{10,}udders.*sebaceous[^\n]{6,13}polemoniums.*jerry"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(321, vec![]);

    builder = builder.with_expression(
        Regex::new(322, r##"motocross.*boobing"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(322, vec![]);

    builder = builder.with_expression(
        Regex::new(323, r##"loathness.*paniers.*rebottle.*prosciuttiairns"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(323, vec![]);

    builder = builder.with_expression(
        Regex::new(324, r##"^pitchpoled.+unfaith"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(324, vec![]);

    builder = builder.with_expression(
        Regex::new(325, r##"institutionalising.*margraviates.*unaccented"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(325, vec![]);

    builder = builder.with_expression(
        Regex::new(326, r##"butylate"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(326, vec![]);

    builder = builder.with_expression(
        Regex::new(327, r##"^quantify.{7,}daringness.{5,}entrancements"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(327, vec![]);

    builder = builder.with_expression(
        Regex::new(328, r##"malar.*urtications.*hospitalising.*beam.*luging"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(328, vec![]);

    builder = builder.with_expression(
        Regex::new(329, r##"aidman[^\n]{9,12}tousle.*shopgirl"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(329, vec![]);

    builder = builder.with_expression(
        Regex::new(330, r##"bleater.*ferule"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(330, vec![]);

    builder = builder.with_expression(
        Regex::new(
            331,
            r##"renouncementsresewndeviationismrallinerecompensed"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(331, vec![]);

    builder = builder.with_expression(
        Regex::new(
            332,
            r##"pupil.*reproducible.*malleabilities.{4,}overleaped"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(332, vec![]);

    builder = builder.with_expression(
        Regex::new(
            333,
            r##"^ornately.*flushes.*tenable.*sufficer.*enunciations"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(333, vec![]);

    builder = builder.with_expression(Regex::new(334, r##"digital.{7,13}mammalian.{10,14}paleopathologies[^\r\n]{5,8}cookstoves[^\n]+overkilling"##).
        mode(MatchMode::All(Submatch::Expression)).encoding(Encoding::Byte).case_sensitive(false).build()?);
    expected.insert(334, vec![]);

    builder = builder.with_expression(
        Regex::new(335, r##"antiauthority.*keelhauls"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(335, vec![]);

    builder = builder.with_expression(
        Regex::new(336, r##"sensitometric.{9,}syllabifiesprizefighter"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(336, vec![]);

    builder = builder.with_expression(
        Regex::new(337, r##"^((idocrases.*overgild.*bluffer)|(shivering))"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(337, vec![]);

    builder = builder.with_expression(
        Regex::new(
            338,
            r##"broideries[^\n]{5,14}psephologists[^\n]{4,8}deucedly[^\n]{1,7}electrophoresing"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(338, vec![]);

    builder = builder.with_expression(
        Regex::new(
            339,
            r##"muklukschinbone[^\r\n]{4,14}reaffixes.{9,15}psaltries.{4,12}inferring"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(339, vec![]);

    builder = builder.with_expression(
        Regex::new(
            340,
            r##"cislunarcodependencepreeclamptic.*creped.*hyperpituitarism"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(340, vec![]);

    builder = builder.with_expression(
        Regex::new(341, r##"engravers"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(341, vec![]);

    builder = builder.with_expression(
        Regex::new(342, r##"abominated[^\n]+barramundi"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(342, vec![]);

    builder = builder.with_expression(
        Regex::new(343, r##"hoops.*prolog"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(343, vec![]);

    builder = builder.with_expression(
        Regex::new(
            344,
            r##"plumed[^\n]+damsons[^\n]+disproved[^\n]+leishmania.*literatim"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(344, vec![]);

    builder = builder.with_expression(
        Regex::new(345, r##"monometallist[^\n]+superprofits[^\n]+chimera"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(345, vec![]);

    builder = builder.with_expression(
        Regex::new(346, r##"hydras[^\n]{5,}semifitted[^\n]{1,}sclerometers"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(346, vec![]);

    builder = builder.with_expression(
        Regex::new(347, r##"microsurgicalcontextrandansredefiesbimanually"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(347, vec![]);

    builder = builder.with_expression(
        Regex::new(348, r##"drawer.*redcap.*mesothelial"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(348, vec![]);

    builder = builder.with_expression(
        Regex::new(349, r##"declining.*chopine.*childbirth"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(349, vec![]);

    builder = builder.with_expression(
        Regex::new(
            350,
            r##"spicebushestrespassesvegetationaltruncatedquasiperiodicities"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(350, vec![]);

    builder = builder.with_expression(
        Regex::new(351, r##"interrogationintombviolently"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(351, vec![]);

    builder = builder.with_expression(
        Regex::new(352, r##"bottommost.*recharting.{6,}yelps.{3,}emend"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(352, vec![]);

    builder = builder.with_expression(
        Regex::new(353, r##"maloti.*counterviolences"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(353, vec![]);

    builder = builder.with_expression(
        Regex::new(354, r##"perphenazine[^\r\n]{2,}smoothest"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(354, vec![]);

    builder = builder.with_expression(
        Regex::new(355, r##"baloney.*sailboarding"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(355, vec![]);

    builder = builder.with_expression(
        Regex::new(356, r##"reafforesting"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(356, vec![]);

    builder = builder.with_expression(
        Regex::new(357, r##"albite[^\n]+continuant"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(357, vec![]);

    builder = builder.with_expression(
        Regex::new(358, r##"stuttered.{8,9}enantiomeric"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(358, vec![]);

    builder = builder.with_expression(
        Regex::new(359, r##"encompassmentsfitpostelectionglamorousnesses"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(359, vec![]);

    builder = builder.with_expression(
        Regex::new(360, r##"dishwater.*sulfonylureas"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(360, vec![]);

    builder = builder.with_expression(
        Regex::new(361, r##"oppositeness.+lutherns"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(361, vec![]);

    builder = builder.with_expression(
        Regex::new(362, r##"dyskinesia"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(362, vec![]);

    builder = builder.with_expression(Regex::new(363, r##"^ultracompetent[^\r\n]{9,9}beige.*microphotographs[^\r\n]{5,8}recto[^\r\n]{5,12}blackguarding"##).
        mode(MatchMode::All(Submatch::Expression)).encoding(Encoding::Byte).case_sensitive(false).build()?);
    expected.insert(363, vec![]);

    builder = builder.with_expression(
        Regex::new(364, r##"exsertingtruncheoned"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(364, vec![]);

    builder = builder.with_expression(
        Regex::new(
            365,
            r##"differentia.*garnishment.*terai.*regency.*filiate"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(365, vec![]);

    builder = builder.with_expression(
        Regex::new(366, r##"dawned"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(366, vec![]);

    builder = builder.with_expression(
        Regex::new(367, r##"hyena.*unprofitably.*tildes"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(367, vec![]);

    builder = builder.with_expression(
        Regex::new(368, r##"libidinally.*ulcering.*ablate.*reata"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(368, vec![]);

    builder = builder.with_expression(Regex::new(369, r##"gatelesssublibrarian[^\r\n]{4,}chromatids[^\r\n]{10,}alkalinize[^\r\n]{2,}eyebrows"##).
        mode(MatchMode::All(Submatch::Expression)).encoding(Encoding::Byte).case_sensitive(false).build()?);
    expected.insert(369, vec![]);

    builder = builder.with_expression(
        Regex::new(370, r##"fielded.*playgirl.{9,}sallied"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(370, vec![]);

    builder = builder.with_expression(
        Regex::new(371, r##"inconscient"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(371, vec![]);

    builder = builder.with_expression(
        Regex::new(372, r##"glossator"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(372, vec![]);

    builder = builder.with_expression(
        Regex::new(373, r##"noncircularcytophotometric"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(373, vec![]);

    builder = builder.with_expression(
        Regex::new(374, r##"gnomons[^\n]{9,}sots.*chuckled"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(374, vec![]);

    builder = builder.with_expression(
        Regex::new(375, r##"liverworts.*hashed.*cannellonibeleaguer"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(375, vec![]);

    builder = builder.with_expression(
        Regex::new(376, r##"mentorship.*xenophobically.*mercilessnesses"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(376, vec![]);

    builder = builder.with_expression(
        Regex::new(377, r##"seadrome.*catechized.*counterweights.*uprated"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(377, vec![]);

    builder = builder.with_expression(
        Regex::new(378, r##"strategizingmusersiolite.{9,}cookoffs"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(378, vec![]);

    builder = builder.with_expression(
        Regex::new(379, r##"thermoregulators"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(379, vec![]);

    builder = builder.with_expression(
        Regex::new(380, r##"reteamed.{1,5}rainspout.+cupid"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(380, vec![]);

    builder = builder.with_expression(
        Regex::new(381, r##"retransforming[^\n]{5,14}nervules"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(381, vec![]);

    builder = builder.with_expression(Regex::new(382, r##"deoxygenates[^\r\n]{10,20}feted[^\r\n]{9,18}videodisk[^\n]{2,}plasterings.*pumpkinseeds"##).
        mode(MatchMode::All(Submatch::Expression)).encoding(Encoding::Byte).case_sensitive(false).build()?);
    expected.insert(382, vec![]);

    builder = builder.with_expression(
        Regex::new(383, r##"mojoes.*idyll"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(383, vec![]);

    builder = builder.with_expression(
        Regex::new(384, r##"funerals"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        384,
        vec![
            vec![Capture::new(Some(2599782), Some(2599790))],
            vec![Capture::new(Some(3454565), Some(3454573))],
            vec![Capture::new(Some(4758478), Some(4758486))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(385, r##"balkinesses.*chayote"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(385, vec![]);

    builder = builder.with_expression(
        Regex::new(386, r##"lycopods.*holloas"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(386, vec![]);

    builder = builder.with_expression(
        Regex::new(387, r##"besottedzwitterions.*paddocked.*shocks"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(387, vec![]);

    builder = builder.with_expression(
        Regex::new(388, r##"taxers"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(388, vec![]);

    builder = builder.with_expression(
        Regex::new(389, r##"pedologistsfathered.*undramatically"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(389, vec![]);

    builder = builder.with_expression(
        Regex::new(
            390,
            r##"incepts[^\r\n]{2,}indulgent.*whereupon.*chelicera"##,
        )
        .mode(MatchMode::All(Submatch::Expression))
        .encoding(Encoding::Byte)
        .case_sensitive(false)
        .build()?,
    );
    expected.insert(390, vec![]);

    builder = builder.with_expression(
        Regex::new(391, r##"acidulate"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(391, vec![]);

    builder = builder.with_expression(
        Regex::new(392, r##"sourest.*amaurosis.*gourmet.*phototypesetters"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(392, vec![]);

    builder = builder.with_expression(
        Regex::new(393, r##"cayman.{4,}frogeyed"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(393, vec![]);

    builder = builder.with_expression(
        Regex::new(394, r##"diverseness"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(394, vec![]);

    builder = builder.with_expression(
        Regex::new(395, r##"transudations"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(395, vec![]);

    builder = builder.with_expression(
        Regex::new(396, r##"louvred.*traipses.*propagandizing"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(396, vec![]);

    builder = builder.with_expression(
        Regex::new(397, r##"photogeology.*signed.*containerisation.*buboes"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(397, vec![]);

    builder = builder.with_expression(Regex::new(398, r##"struthious[^\r\n]{5,}tranquilize[^\r\n]{5,}cryptorchid[^\r\n]{1,}cloxacillins.+diabetologist"##).
        mode(MatchMode::All(Submatch::Expression)).encoding(Encoding::Byte).case_sensitive(false).build()?);
    expected.insert(398, vec![]);

    builder = builder.with_expression(
        Regex::new(399, r##"sterling"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(
        399,
        vec![
            vec![Capture::new(Some(1048505), Some(1048513))],
            vec![Capture::new(Some(1410827), Some(1410835))],
            vec![Capture::new(Some(3980797), Some(3980805))],
        ],
    );

    builder = builder.with_expression(
        Regex::new(400, r##"unsound[^\n]{8,}outsert"##)
            .mode(MatchMode::All(Submatch::Expression))
            .encoding(Encoding::Byte)
            .case_sensitive(false)
            .build()?,
    );
    expected.insert(400, vec![]);

    let database = builder.build();

    // The block sizes here are every size from 1 to 16 (i.e. the size of the rolling window),
    // 17 to catch off-by-ones near the window size, 42 is obvious, 768-1200 for network packets,
    // 9000 for jumbograms, and 65536 because of course. The final 5458200 is to see how we do with
    // the file all at once.
    for block_size in [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 42, 768, 1200, 4096, 9000,
        65536, 5458200,
    ] {
        let mut handler = TestHandler::new();
        let mut scratch = database.make_scratch(&mut handler);
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test_data/shakespeare.txt");
        let mut file = File::open(d).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        for chunk in contents.as_bytes().chunks(block_size) {
            scratch.push(chunk);
        }
        scratch.finish();

        let results = handler.to_results();
        for k in expected.keys() {
            if expected[k].is_empty() {
                assert!(!results.contains_key(k));
            } else {
                //assert_eq!(expected[k].len(), results[k].len());
                for i in 0..expected[k].len() {
                    assert_eq!(expected[k][i], results[k][i]);
                }
                assert_eq!(expected[k], results[k]);
            }
        }
    }
    Ok(())
}
