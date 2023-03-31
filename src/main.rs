use std::{fs::File, collections::{HashMap, HashSet}};
use crate::sax::Event;
use anyhow::{anyhow};

mod sax;

fn analyze_struct(xml_path: &str) -> anyhow::Result<HashMap<String, HashSet<String>>> {
    let f = File::open(xml_path)?;
    let mut el_attrs: HashMap<String, HashSet<String>> = HashMap::new();
    let mut p = sax::parser::Parser::from_reader(f);

    let mut i = 1;
    let mut el_stack = vec![];
    loop {
        let event = match p.read_event() {
            Ok(res) => res,
            Err(err) => {
                println!("parsing err => {:?}", err);
                return Err(anyhow!(err));
            }
        };
        match event {
            Event::EndDocument => break,
            Event::StartElement(el) => {
                //println!("{}: {}", i, el.local_name);
                if i % 1000 == 0 {
                    //println!("{} done.", i);
                }
                i += 1;
                if !el_attrs.contains_key(el.local_name) {
                    el_attrs.insert(el.local_name.to_owned(), HashSet::new());
                    println!("element => {} 新たに発生 (level => {})", el.local_name, el_stack.len() + 1);
                }
                let attr_store = el_attrs.get_mut(el.local_name).unwrap();
                for attr in el.attributes() {
                    if !attr_store.contains(attr.local_name) {
                        attr_store.insert(attr.local_name.to_owned());
                    }
                }
                if !el.is_empty {
                    el_stack.push(el.name.to_owned());
                }
            }
            Event::EndElement(el) => {
                let nm = el_stack.pop().unwrap_or("".to_string());
                if nm != el.local_name {
                    return Err(anyhow!("unexpected end element: expect: {}, actual: {}", el.local_name, nm));
                }
            }
            _ => {}
        }
    }
    Ok(el_attrs)
}

fn main() {

    /*{"noncharacter": {"RI", "EMod", "bpb", "AHex", "NChar", "Ideo", "Term", "bc", "Math", "JSN", "Hex", "OGr_Ext", "Gr_Link", "Dia", "OIDS", "sc", "ccc", "lc", "VS", "XIDS", "Bidi_M", "scf", "Dash", "IDS", "Pat_WS", "XO_NFD", "WB", "OAlpha", "Gr_Base", "Dep", "Cased", "Alpha", "tc", "cf", "nt", "IDST", "hst", "DI", "OUpper", "QMark", "NFC_QC", "NFKC_QC", "XO_NFKC", "Ext", "ea", "UIdeo", "ODI", "OIDC", "Radical", "first-cp", "GCB", "CWCF", "dm", "NFD_QC", "FC_NFKC", "CWKCF", "na", "CWL", "age", "XIDC", "SD", "SB", "XO_NFC", "CWU", "Upper", "Pat_Syn", "NFKC_CF", "vo", "Lower", "PCM", "lb", "LOE", "Comp_Ex", "uc", "isc", "STerm", "EBase", "InPC", "ExtPict", "na1", "stc", "IDC", "InSC", "Join_C", "OLower", "NFKD_QC", "Emoji", "jg", "EPres", "EComp", "dt", "Hyphen", "scx", "CI", "suc", "WSpace", "gc", "OMath", "slc", "bpt", "Bidi_C", "Gr_Ext", "CWCM", "CWT", "IDSB", "XO_NFKD", "blk", "last-cp", "jt", "CE", "bmg", "nv"}, "standardized-variant": {"desc", "when", "cps"}, "surrogate": {"OIDS", "age", "OIDC", "XIDC", "UIdeo", "LOE", "ODI", "jg", "WB", "XO_NFC", "Dep", "XO_NFD", "ccc", "tc", "sc", "Hex", "Gr_Ext", "dm", "Ideo", "OUpper", "Ext", "Dia", "NChar", "Pat_WS", "WSpace", "scf", "OGr_Ext", "CE", "Comp_Ex", "NFKD_QC", "lc", "dt", "scx", "Radical", "OAlpha", "hst", "CWCF", "CWCM", "Emoji", "EPres", "gc", "slc", "STerm", "CI", "Upper", "CWL", "CWT", "Bidi_M", "Gr_Base", "CWU", "Lower", "Math", "PCM", "GCB", "RI", "na1", "EComp", "Pat_Syn", "IDC", "cf", "CWKCF", "EMod", "EBase", "jt", "NFKC_QC", "JSN", "bpt", "Alpha", "Cased", "XO_NFKC", "ExtPict", "bc", "bmg", "XIDS", "ea", "Term", "DI", "AHex", "XO_NFKD", "InPC", "NFD_QC", "lb", "NFC_QC", "NFKC_CF", "FC_NFKC", "isc", "uc", "OMath", "na", "Join_C", "VS", "IDST", "stc", "nv", "Gr_Link", "IDSB", "IDS", "SD", "SB", "InSC", "nt", "vo", "blk", "QMark", "first-cp", "Bidi_C", "bpb", "suc", "Hyphen", "OLower", "last-cp", "Dash"}, "emoji-source": {"softbank", "docomo", "kddi", "unicode"}, "cjk-radical": {"number", "radical", "ideograph"}, "emoji-sources": {}, "named-sequences": {}, "named-sequence": {"cps", "name"}, "repertoire": {}, "blocks": {}, "ucd": {"xmlns"}, "description": {}, "standardized-variants": {}, "name-alias": {"type", "alias"}, "cjk-radicals": {}, "normalization-correction": {"cp", "version", "new", "old"}, "reserved": {"QMark", "IDC", "NFKD_QC", "NFD_QC", "na1", "nv", "CWT", "dt", "nt", "first-cp", "GCB", "stc", "Comp_Ex", "RI", "na", "uc", "NFKC_QC", "ccc", "LOE", "bmg", "Dash", "EMod", "Gr_Link", "suc", "lc", "Lower", "AHex", "STerm", "Dia", "scf", "SB", "XO_NFKC", "EPres", "Math", "VS", "IDS", "scx", "Bidi_M", "WB", "CWCF", "DI", "cf", "Ideo", "OUpper", "Cased", "ODI", "XO_NFKD", "EComp", "Pat_Syn", "hst", "NChar", "XO_NFC", "CI", "CWKCF", "PCM", "Gr_Ext", "EBase", "NFKC_CF", "bc", "CWU", "OIDC", "lb", "bpb", "XIDS", "Hyphen", "jg", "Pat_WS", "isc", "sc", "CWL", "OLower", "Ext", "dm", "slc", "NFC_QC", "Alpha", "InPC", "WSpace", "tc", "ea", "blk", "gc", "Term", "XIDC", "UIdeo", "Bidi_C", "Hex", "OAlpha", "Upper", "OMath", "Gr_Base", "OGr_Ext", "SD", "IDSB", "Radical", "age", "Emoji", "jt", "Dep", "last-cp", "CE", "XO_NFD", "IDST", "CWCM", "ExtPict", "InSC", "cp", "OIDS", "JSN", "vo", "bpt", "FC_NFKC", "Join_C"}, "block": {"first-cp", "name", "last-cp"}, "normalization-corrections": {}, "char": {"kHKGlyph", "kRSUnicode", "XIDC", "Join_C", "GCB", "IDC", "ccc", "Gr_Base", "kNelson", "kHangul", "bpt", "ODI", "Math", "SD", "kIRG_GSource", "kMandarin", "kKarlgren", "kIRGHanyuDaZidian", "kPrimaryNumeric", "kJoyoKanji", "WB", "last-cp", "kXHC1983", "kHanyuPinyin", "kIRGDaiKanwaZiten", "kMorohashi", "STerm", "OLower", "OGr_Ext", "CI", "kCowles", "DI", "JSN", "IDS", "isc", "Lower", "kIRG_MSource", "CWT", "NFC_QC", "na", "slc", "kJis0", "OIDC", "kCheungBauerIndex", "kXerox", "dt", "dm", "kSemanticVariant", "Term", "OIDS", "Comp_Ex", "kSBGY", "kFennIndex", "AHex", "CWCM", "kIICore", "kGB0", "first-cp", "blk", "FC_NFKC", "kSrc_NushuDuben", "nv", "NFD_QC", "kTGHZ2013", "XO_NFC", "kHanyuPinlu", "kGB3", "scx", "kSpecializedSemanticVariant", "LOE", "kCCCII", "kDaeJaweon", "Ideo", "Alpha", "kGradeLevel", "kIRG_KPSource", "suc", "lc", "Radical", "kMeyerWempe", "NFKD_QC", "XIDS", "kIRG_HSource", "cf", "UIdeo", "NFKC_QC", "kIRG_UKSource", "kJIS0213", "tc", "kRSAdobe_Japan1_6", "kHanYu", "kOtherNumeric", "kCNS1992", "EPres", "kIBMJapan", "kCantonese", "kHDZRadBreak", "IDST", "Ext", "InPC", "gc", "Upper", "RI", "kJa", "CWL", "kLau", "jt", "Hyphen", "kIRG_SSource", "kKangXi", "kTang", "kKSC0", "kPseudoGB1", "kDefinition", "kKSC1", "kKorean", "kGB1", "kReading", "hst", "kGB5", "kFourCornerCode", "kKoreanEducationHanja", "kIRGDaeJaweon", "IDSB", "XO_NFKC", "kJinmeiyoKanji", "kRSKangXi", "lb", "Dep", "kTraditionalVariant", "OAlpha", "bpb", "QMark", "kStrange", "kIRG_KSource", "kMatthews", "WSpace", "kJapaneseOn", "CWCF", "kCangjie", "uc", "age", "kEACC", "kGB7", "kTGT_MergedSrc", "Gr_Ext", "CWKCF", "kTaiwanTelegraph", "kCompatibilityVariant", "Bidi_C", "EComp", "Cased", "scf", "cp", "kIRG_JSource", "OUpper", "XO_NFD", "kCheungBauer", "kIRG_VSource", "kSpoofingVariant", "kTotalStrokes", "kBigFive", "bc", "ea", "jg", "XO_NFKD", "kIRGKangXi", "kPhonetic", "Hex", "NFKC_CF", "kIRG_TSource", "kTGH", "kVietnamese", "kKPS0", "kFrequency", "stc", "kJis1", "Bidi_M", "Pat_WS", "EMod", "EqUIdeo", "NChar", "SB", "kAccountingNumeric", "kGB8", "kIRG_USource", "bmg", "kCihaiT", "CE", "InSC", "vo", "Emoji", "kKPS1", "kCNS1986", "kMainlandTelegraph", "Dash", "kRSTUnicode", "Gr_Link", "EBase", "VS", "Pat_Syn", "Dia", "ExtPict", "kJapaneseKun", "kUnihanCore2020", "nt", "OMath", "na1", "PCM", "kGSR", "kSimplifiedVariant", "CWU", "kHKSCS", "kKoreanName", "kZVariant", "sc", "kFenn"}}    
     */

    let mj = "123456林龍太アイウエオabcdefg亜bcでｆｇ";
    let mj_buf = mj.as_bytes();
    let mj_slice = &mj_buf[6..9];
    let char_idx = match std::str::from_utf8(mj_slice) {
        Ok(c) => c,
        Err(_) => ""
    };
    println!("mj_slice: {}", char_idx);

    let xml_path = "./dat/ucd.all.flat.xml";
    let el_attrs = match analyze_struct(xml_path) {
        Ok(el) => el,
        Err(e) => {
            panic!("{:?}", e);
        }
    };
    println!("{:?}", el_attrs);
}
