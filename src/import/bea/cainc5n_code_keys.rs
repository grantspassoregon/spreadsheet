use serde::{Deserialize, Serialize};
use serde::de::Deserializer;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum Cainc5nCodeKey {
    CAINC5N_10,
    CAINC5N_100,
    CAINC5N_1000,
    CAINC5N_1001,
    CAINC5N_1002,
    CAINC5N_1003,
    CAINC5N_1004,
    CAINC5N_1005,
    CAINC5N_101,
    CAINC5N_102,
    CAINC5N_103,
    CAINC5N_1100,
    CAINC5N_1101,
    CAINC5N_1102,
    CAINC5N_1103,
    CAINC5N_1200,
    CAINC5N_1300,
    CAINC5N_1400,
    CAINC5N_1401,
    CAINC5N_1402,
    CAINC5N_1500,
    CAINC5N_1600,
    CAINC5N_1601,
    CAINC5N_1602,
    CAINC5N_1603,
    CAINC5N_1604,
    CAINC5N_1700,
    CAINC5N_1701,
    CAINC5N_1702,
    CAINC5N_1703,
    CAINC5N_1800,
    CAINC5N_1801,
    CAINC5N_1802,
    CAINC5N_1900,
    CAINC5N_1901,
    CAINC5N_1902,
    CAINC5N_1903,
    CAINC5N_1904,
    CAINC5N_20,
    CAINC5N_200,
    CAINC5N_2000,
    CAINC5N_2001,
    CAINC5N_2002,
    CAINC5N_201,
    CAINC5N_2010,
    CAINC5N_2011,
    CAINC5N_2012,
    CAINC5N_202,
    CAINC5N_203,
    CAINC5N_30,
    CAINC5N_300,
    CAINC5N_35,
    CAINC5N_36,
    CAINC5N_37,
    CAINC5N_38,
    CAINC5N_400,
    CAINC5N_401,
    CAINC5N_402,
    CAINC5N_403,
    CAINC5N_42,
    CAINC5N_45,
    CAINC5N_46,
    CAINC5N_47,
    CAINC5N_50,
    CAINC5N_500,
    CAINC5N_510,
    CAINC5N_511,
    CAINC5N_512,
    CAINC5N_513,
    CAINC5N_514,
    CAINC5N_515,
    CAINC5N_516,
    CAINC5N_517,
    CAINC5N_518,
    CAINC5N_519,
    CAINC5N_521,
    CAINC5N_522,
    CAINC5N_530,
    CAINC5N_531,
    CAINC5N_532,
    CAINC5N_533,
    CAINC5N_534,
    CAINC5N_535,
    CAINC5N_536,
    CAINC5N_537,
    CAINC5N_538,
    CAINC5N_539,
    CAINC5N_541,
    CAINC5N_542,
    CAINC5N_60,
    CAINC5N_600,
    CAINC5N_61,
    CAINC5N_62,
    CAINC5N_70,
    CAINC5N_700,
    CAINC5N_701,
    CAINC5N_702,
    CAINC5N_703,
    CAINC5N_704,
    CAINC5N_705,
    CAINC5N_706,
    CAINC5N_707,
    CAINC5N_708,
    CAINC5N_709,
    CAINC5N_71,
    CAINC5N_711,
    CAINC5N_712,
    CAINC5N_713,
    CAINC5N_72,
    CAINC5N_800,
    CAINC5N_801,
    CAINC5N_802,
    CAINC5N_803,
    CAINC5N_804,
    CAINC5N_805,
    CAINC5N_806,
    CAINC5N_807,
    CAINC5N_808,
    CAINC5N_809,
    CAINC5N_81,
    CAINC5N_811,
    CAINC5N_82,
    CAINC5N_90,
    CAINC5N_900,
    CAINC5N_901,
    CAINC5N_902,
    CAINC5N_903,
    CAINC5N_904,
    CAINC5N_905,
    CAINC5N_906,
    CAINC5N_907,
}

pub fn match_code_key(input: &str) -> Option<Cainc5nCodeKey> {
    match input {
        "CAINC5N-10" => Some(Cainc5nCodeKey::CAINC5N_10),
        "CAINC5N-100" => Some(Cainc5nCodeKey::CAINC5N_100),
        "CAINC5N-1000" => Some(Cainc5nCodeKey::CAINC5N_1000),
        "CAINC5N-1001" => Some(Cainc5nCodeKey::CAINC5N_1001),
        "CAINC5N-1002" => Some(Cainc5nCodeKey::CAINC5N_1002),
        "CAINC5N-1003" => Some(Cainc5nCodeKey::CAINC5N_1003),
        "CAINC5N-1004" => Some(Cainc5nCodeKey::CAINC5N_1004),
        "CAINC5N-1005" => Some(Cainc5nCodeKey::CAINC5N_1005),
        "CAINC5N-101" => Some(Cainc5nCodeKey::CAINC5N_101),
        "CAINC5N-102" => Some(Cainc5nCodeKey::CAINC5N_102),
        "CAINC5N-103" => Some(Cainc5nCodeKey::CAINC5N_103),
        "CAINC5N-1100" => Some(Cainc5nCodeKey::CAINC5N_1100),
        "CAINC5N-1101" => Some(Cainc5nCodeKey::CAINC5N_1101),
        "CAINC5N-1102" => Some(Cainc5nCodeKey::CAINC5N_1102),
        "CAINC5N-1103" => Some(Cainc5nCodeKey::CAINC5N_1103),
        "CAINC5N-1200" => Some(Cainc5nCodeKey::CAINC5N_1200),
        "CAINC5N-1300" => Some(Cainc5nCodeKey::CAINC5N_1300),
        "CAINC5N-1400" => Some(Cainc5nCodeKey::CAINC5N_1400),
        "CAINC5N-1401" => Some(Cainc5nCodeKey::CAINC5N_1401),
        "CAINC5N-1402" => Some(Cainc5nCodeKey::CAINC5N_1402),
        "CAINC5N-1500" => Some(Cainc5nCodeKey::CAINC5N_1500),
        "CAINC5N-1600" => Some(Cainc5nCodeKey::CAINC5N_1600),
        "CAINC5N-1601" => Some(Cainc5nCodeKey::CAINC5N_1601),
        "CAINC5N-1602" => Some(Cainc5nCodeKey::CAINC5N_1602),
        "CAINC5N-1603" => Some(Cainc5nCodeKey::CAINC5N_1603),
        "CAINC5N-1604" => Some(Cainc5nCodeKey::CAINC5N_1604),
        "CAINC5N-1700" => Some(Cainc5nCodeKey::CAINC5N_1700),
        "CAINC5N-1701" => Some(Cainc5nCodeKey::CAINC5N_1701),
        "CAINC5N-1702" => Some(Cainc5nCodeKey::CAINC5N_1702),
        "CAINC5N-1703" => Some(Cainc5nCodeKey::CAINC5N_1703),
        "CAINC5N-1800" => Some(Cainc5nCodeKey::CAINC5N_1800),
        "CAINC5N-1801" => Some(Cainc5nCodeKey::CAINC5N_1801),
        "CAINC5N-1802" => Some(Cainc5nCodeKey::CAINC5N_1802),
        "CAINC5N-1900" => Some(Cainc5nCodeKey::CAINC5N_1900),
        "CAINC5N-1901" => Some(Cainc5nCodeKey::CAINC5N_1901),
        "CAINC5N-1902" => Some(Cainc5nCodeKey::CAINC5N_1902),
        "CAINC5N-1903" => Some(Cainc5nCodeKey::CAINC5N_1903),
        "CAINC5N-1904" => Some(Cainc5nCodeKey::CAINC5N_1904),
        "CAINC5N-20" => Some(Cainc5nCodeKey::CAINC5N_20),
        "CAINC5N-200" => Some(Cainc5nCodeKey::CAINC5N_200),
        "CAINC5N-2000" => Some(Cainc5nCodeKey::CAINC5N_2000),
        "CAINC5N-2001" => Some(Cainc5nCodeKey::CAINC5N_2001),
        "CAINC5N-2002" => Some(Cainc5nCodeKey::CAINC5N_2002),
        "CAINC5N-201" => Some(Cainc5nCodeKey::CAINC5N_201),
        "CAINC5N-2010" => Some(Cainc5nCodeKey::CAINC5N_2010),
        "CAINC5N-2011" => Some(Cainc5nCodeKey::CAINC5N_2011),
        "CAINC5N-2012" => Some(Cainc5nCodeKey::CAINC5N_2012),
        "CAINC5N-202" => Some(Cainc5nCodeKey::CAINC5N_202),
        "CAINC5N-203" => Some(Cainc5nCodeKey::CAINC5N_203),
        "CAINC5N-30" => Some(Cainc5nCodeKey::CAINC5N_30),
        "CAINC5N-300" => Some(Cainc5nCodeKey::CAINC5N_300),
        "CAINC5N-35" => Some(Cainc5nCodeKey::CAINC5N_35),
        "CAINC5N-36" => Some(Cainc5nCodeKey::CAINC5N_36),
        "CAINC5N-37" => Some(Cainc5nCodeKey::CAINC5N_37),
        "CAINC5N-38" => Some(Cainc5nCodeKey::CAINC5N_38),
        "CAINC5N-400" => Some(Cainc5nCodeKey::CAINC5N_400),
        "CAINC5N-401" => Some(Cainc5nCodeKey::CAINC5N_401),
        "CAINC5N-402" => Some(Cainc5nCodeKey::CAINC5N_402),
        "CAINC5N-403" => Some(Cainc5nCodeKey::CAINC5N_403),
        "CAINC5N-42" => Some(Cainc5nCodeKey::CAINC5N_42),
        "CAINC5N-45" => Some(Cainc5nCodeKey::CAINC5N_45),
        "CAINC5N-46" => Some(Cainc5nCodeKey::CAINC5N_46),
        "CAINC5N-47" => Some(Cainc5nCodeKey::CAINC5N_47),
        "CAINC5N-50" => Some(Cainc5nCodeKey::CAINC5N_50),
        "CAINC5N-500" => Some(Cainc5nCodeKey::CAINC5N_500),
        "CAINC5N-510" => Some(Cainc5nCodeKey::CAINC5N_510),
        "CAINC5N-511" => Some(Cainc5nCodeKey::CAINC5N_511),
        "CAINC5N-512" => Some(Cainc5nCodeKey::CAINC5N_512),
        "CAINC5N-513" => Some(Cainc5nCodeKey::CAINC5N_513),
        "CAINC5N-514" => Some(Cainc5nCodeKey::CAINC5N_514),
        "CAINC5N-515" => Some(Cainc5nCodeKey::CAINC5N_515),
        "CAINC5N-516" => Some(Cainc5nCodeKey::CAINC5N_516),
        "CAINC5N-517" => Some(Cainc5nCodeKey::CAINC5N_517),
        "CAINC5N-518" => Some(Cainc5nCodeKey::CAINC5N_518),
        "CAINC5N-519" => Some(Cainc5nCodeKey::CAINC5N_519),
        "CAINC5N-521" => Some(Cainc5nCodeKey::CAINC5N_521),
        "CAINC5N-522" => Some(Cainc5nCodeKey::CAINC5N_522),
        "CAINC5N-530" => Some(Cainc5nCodeKey::CAINC5N_530),
        "CAINC5N-531" => Some(Cainc5nCodeKey::CAINC5N_531),
        "CAINC5N-532" => Some(Cainc5nCodeKey::CAINC5N_532),
        "CAINC5N-533" => Some(Cainc5nCodeKey::CAINC5N_533),
        "CAINC5N-534" => Some(Cainc5nCodeKey::CAINC5N_534),
        "CAINC5N-535" => Some(Cainc5nCodeKey::CAINC5N_535),
        "CAINC5N-536" => Some(Cainc5nCodeKey::CAINC5N_536),
        "CAINC5N-537" => Some(Cainc5nCodeKey::CAINC5N_537),
        "CAINC5N-538" => Some(Cainc5nCodeKey::CAINC5N_538),
        "CAINC5N-539" => Some(Cainc5nCodeKey::CAINC5N_539),
        "CAINC5N-541" => Some(Cainc5nCodeKey::CAINC5N_541),
        "CAINC5N-542" => Some(Cainc5nCodeKey::CAINC5N_542),
        "CAINC5N-60" => Some(Cainc5nCodeKey::CAINC5N_60),
        "CAINC5N-600" => Some(Cainc5nCodeKey::CAINC5N_600),
        "CAINC5N-61" => Some(Cainc5nCodeKey::CAINC5N_61),
        "CAINC5N-62" => Some(Cainc5nCodeKey::CAINC5N_62),
        "CAINC5N-70" => Some(Cainc5nCodeKey::CAINC5N_70),
        "CAINC5N-700" => Some(Cainc5nCodeKey::CAINC5N_700),
        "CAINC5N-701" => Some(Cainc5nCodeKey::CAINC5N_701),
        "CAINC5N-702" => Some(Cainc5nCodeKey::CAINC5N_702),
        "CAINC5N-703" => Some(Cainc5nCodeKey::CAINC5N_703),
        "CAINC5N-704" => Some(Cainc5nCodeKey::CAINC5N_704),
        "CAINC5N-705" => Some(Cainc5nCodeKey::CAINC5N_705),
        "CAINC5N-706" => Some(Cainc5nCodeKey::CAINC5N_706),
        "CAINC5N-707" => Some(Cainc5nCodeKey::CAINC5N_707),
        "CAINC5N-708" => Some(Cainc5nCodeKey::CAINC5N_708),
        "CAINC5N-709" => Some(Cainc5nCodeKey::CAINC5N_709),
        "CAINC5N-71" => Some(Cainc5nCodeKey::CAINC5N_71),
        "CAINC5N-711" => Some(Cainc5nCodeKey::CAINC5N_711),
        "CAINC5N-712" => Some(Cainc5nCodeKey::CAINC5N_712),
        "CAINC5N-713" => Some(Cainc5nCodeKey::CAINC5N_713),
        "CAINC5N-72" => Some(Cainc5nCodeKey::CAINC5N_72),
        "CAINC5N-800" => Some(Cainc5nCodeKey::CAINC5N_800),
        "CAINC5N-801" => Some(Cainc5nCodeKey::CAINC5N_801),
        "CAINC5N-802" => Some(Cainc5nCodeKey::CAINC5N_802),
        "CAINC5N-803" => Some(Cainc5nCodeKey::CAINC5N_803),
        "CAINC5N-804" => Some(Cainc5nCodeKey::CAINC5N_804),
        "CAINC5N-805" => Some(Cainc5nCodeKey::CAINC5N_805),
        "CAINC5N-806" => Some(Cainc5nCodeKey::CAINC5N_806),
        "CAINC5N-807" => Some(Cainc5nCodeKey::CAINC5N_807),
        "CAINC5N-808" => Some(Cainc5nCodeKey::CAINC5N_808),
        "CAINC5N-809" => Some(Cainc5nCodeKey::CAINC5N_809),
        "CAINC5N-81" => Some(Cainc5nCodeKey::CAINC5N_81),
        "CAINC5N-811" => Some(Cainc5nCodeKey::CAINC5N_811),
        "CAINC5N-82" => Some(Cainc5nCodeKey::CAINC5N_82),
        "CAINC5N-90" => Some(Cainc5nCodeKey::CAINC5N_90),
        "CAINC5N-900" => Some(Cainc5nCodeKey::CAINC5N_900),
        "CAINC5N-901" => Some(Cainc5nCodeKey::CAINC5N_901),
        "CAINC5N-902" => Some(Cainc5nCodeKey::CAINC5N_902),
        "CAINC5N-903" => Some(Cainc5nCodeKey::CAINC5N_903),
        "CAINC5N-904" => Some(Cainc5nCodeKey::CAINC5N_904),
        "CAINC5N-905" => Some(Cainc5nCodeKey::CAINC5N_905),
        "CAINC5N-906" => Some(Cainc5nCodeKey::CAINC5N_906),
        "CAINC5N-907" => Some(Cainc5nCodeKey::CAINC5N_907),
        _ => None,
    }
}

pub fn deserialize_code_keys<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Cainc5nCodeKey, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;
    Ok(match_code_key(intermediate).unwrap())
}