//! Lazilly evaluated `static`'s, holding an [`Encoding`] for each encoding
//!
//! Note: These originate from the [`lazy_static!`] macro, so you'll have to use the `&*` trick to
//! dereference the static type to get the original [`Encoding`] type. Most actions cause a
//! dereference.
use super::{Encoding, request_mapping_file, parser::parse as parse_ucm};

use lazy_static::lazy_static;

// These blocks are brought to you by this Vim script, run on `resources/mappings.txt`:
//
// " Initial conversion of encoding names to parse_ucm runs
// :'<,'>s/.*/static ref \0: Encoding = parse_ucm(\&request_mapping_file("\0").unwrap()).unwrap();
// " Replaces characters that are legal in names but illegal in Rust statics with `_`
// :%s/\(pub static ref \)[a-zA-Z0-9._-]\+/\=substitute(submatch(0), '[._-]', '_', 'g')/
// " Adds `lazy_static!` blocks to each (can't use one big block, recursion limit overflow)
// :'<,'>s/.*/lazy_static! {\r    \0\r}
// " Capitalizes Rust variable names
// :%s/\(pub static ref \)\([a-zA-Z0-9._-]\+\)/\1\U\2\e/
lazy_static! {
    pub static ref AIX_BIG5_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-big5-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_CNS11643_1986_1_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-CNS11643.1986_1-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_CNS11643_1986_2_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-CNS11643.1986_2-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_1046_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_1046-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_1124_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_1124-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_1129_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_1129-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_1252_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_1252-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_850_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_850-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_856_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_856-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_858_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_858-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_932_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_932-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_943_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_943-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_EUCJP_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_eucJP-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_EUCKR_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_eucKR-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_EUCTW_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_eucTW-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_UDCJP_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_udcJP-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_IBM_UDCJP_GR_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-IBM_udcJP_GR-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_ISO8859_1_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-ISO8859_1-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_ISO8859_15_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-ISO8859_15-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_ISO8859_2_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-ISO8859_2-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_ISO8859_3_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-ISO8859_3-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_ISO8859_4_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-ISO8859_4-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_ISO8859_5_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-ISO8859_5-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_ISO8859_6_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-ISO8859_6-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_ISO8859_7_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-ISO8859_7-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_ISO8859_8_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-ISO8859_8-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_ISO8859_9_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-ISO8859_9-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_JISX0201_1976_0_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-JISX0201.1976_0-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_JISX0201_1976_GR_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-JISX0201.1976_GR-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_JISX0208_1983_0_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-JISX0208.1983_0-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_JISX0208_1983_GR_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-JISX0208.1983_GR-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_KSC5601_1987_0_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-KSC5601.1987_0-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref AIX_TIS_620_4_3_6: Encoding = parse_ucm(&request_mapping_file("aix-TIS_620-4.3.6").unwrap()).unwrap();
}
lazy_static! {
    pub static ref CNS_11643_1992: Encoding = parse_ucm(&request_mapping_file("cns-11643-1992").unwrap()).unwrap();
}
lazy_static! {
    pub static ref EUC_JP_2007: Encoding = parse_ucm(&request_mapping_file("euc-jp-2007").unwrap()).unwrap();
}
lazy_static! {
    pub static ref EUC_TW_2014: Encoding = parse_ucm(&request_mapping_file("euc-tw-2014").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GB_18030_2000: Encoding = parse_ucm(&request_mapping_file("gb-18030-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GB_18030_2005: Encoding = parse_ucm(&request_mapping_file("gb-18030-2005").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ANSI_X3_110_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ANSI_X3.110-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ANSI_X3_110_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ANSI_X3.110-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ARMSCII_8_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ARMSCII_8-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ASMO_449_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ASMO_449-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_BALTIC_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-BALTIC-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_BIG5_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-BIG5-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_BIG5_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-BIG5-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_BIG5HKSCS_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-BIG5HKSCS-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_BS_4730_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-BS_4730-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP10007_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-CP10007-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP1125_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-CP1125-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP1250_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP1250-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP1251_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP1251-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP1252_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP1252-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP1253_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP1253-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP1254_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP1254-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP1255_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP1255-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP1256_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP1256-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP1257_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP1257-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP1258_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP1258-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP737_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP737-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP775_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CP775-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CP932_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-CP932-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CSA_Z243_4_1985_1_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-CSA_Z243.4_1985_1-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CSA_Z243_4_1985_2_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-CSA_Z243.4_1985_2-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CSN_369103_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CSN_369103-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_CWI_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-CWI-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_DEC_MCS_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-DEC_MCS-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_DIN_66003_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-DIN_66003-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_DS_2089_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-DS_2089-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_AT_DE_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_AT_DE-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_AT_DE_A_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_AT_DE_A-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_CA_FR_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_CA_FR-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_DK_NO_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_DK_NO-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_DK_NO_A_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_DK_NO_A-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_ES_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_ES-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_ES_A_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_ES_A-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_ES_S_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_ES_S-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_FI_SE_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_FI_SE-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_FI_SE_A_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_FI_SE_A-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_FR_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_FR-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_IS_FRISS_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_IS_FRISS-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_IT_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_IT-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_PT_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_PT-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_UK_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_UK-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EBCDIC_US_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EBCDIC_US-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ECMA_CYRILLIC_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ECMA_CYRILLIC-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ES2_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ES2-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ES_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ES-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EUC_CN_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EUC_CN-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EUC_CN_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-EUC_CN-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EUC_JP_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EUC_JP-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EUC_JP_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-EUC_JP-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EUC_JP_MS_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-EUC_JP_MS-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EUC_KR_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EUC_KR-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EUC_KR_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-EUC_KR-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_EUC_TW_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-EUC_TW-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_GB_1988_80_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-GB_1988_80-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_GBK_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-GBK-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_GEORGIAN_ACADEMY_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-GEORGIAN_ACADEMY-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_GEORGIAN_PS_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-GEORGIAN_PS-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_GOST_19768_74_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-GOST_19768_74-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_GREEK7_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-GREEK7-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_GREEK7_OLD_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-GREEK7_OLD-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_GREEK_CCITT_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-GREEK_CCITT-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_HP_ROMAN8_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-HP_ROMAN8-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM037_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM037-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM038_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM038-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1004_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1004-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1026_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1026-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1046_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1046-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1047_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1047-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1124_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1124-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1129_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1129-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1132_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1132-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1133_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1133-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1160_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1160-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1161_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1161-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1162_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1162-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1163_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1163-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM1164_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM1164-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM256_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM256-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM273_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM273-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM274_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM274-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM275_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM275-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM277_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM277-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM278_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM278-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM280_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM280-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM281_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM281-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM284_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM284-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM285_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM285-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM290_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM290-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM297_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM297-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM420_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM420-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM423_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM423-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM424_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM424-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM437_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM437-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM500_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM500-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM850_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM850-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM851_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM851-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM852_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM852-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM855_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM855-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM856_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM856-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM857_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM857-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM860_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM860-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM861_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM861-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM862_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM862-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM863_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM863-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM864_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM864-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM864_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM864-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM865_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM865-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM866_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM866-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM866NAV_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM866NAV-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM868_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM868-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM869_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM869-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM870_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM870-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM870_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM870-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM871_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM871-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM874_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM874-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM874_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM874-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM875_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM875-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM880_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM880-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM891_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM891-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM903_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM903-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM904_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM904-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM905_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM905-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM918_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IBM918-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM922_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM922-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IBM943_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IBM943-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IEC_P27_1_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-IEC_P27_1-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_INIS_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-INIS-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_INIS_8_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-INIS_8-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_INIS_CYRILLIC_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-INIS_CYRILLIC-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISIRI_3342_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ISIRI_3342-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_10367_BOX_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_10367_BOX-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_5427_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_5427-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_5427_EXT_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_5427_EXT-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_5428_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_5428-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_5428_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_5428-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO646_US_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO646_US-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_6937_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_6937-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_10_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_10-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_11_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_11-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_1_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_1-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_13_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_13-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_13_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_13-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_14_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_14-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_15_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_15-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_16_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_16-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_2_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_2-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_3_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_3-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_4_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_4-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_5_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_5-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_6_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_6-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_7_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_7-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_7_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_7-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_8_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_8-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_8_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_8-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_8859_9_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_8859_9-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_IR_197_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_IR_197-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_ISO_IR_209_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-ISO_IR_209-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_IT_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-IT-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_JIS_C6220_1969_RO_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-JIS_C6220_1969_RO-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_JIS_C6229_1984_B_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-JIS_C6229_1984_B-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_JOHAB_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-JOHAB-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_JUS_I_B1_002_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-JUS_I.B1.002-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_KOI_8_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-KOI_8-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_KOI8_R_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-KOI8_R-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_KOI8_R_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-KOI8_R-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_KOI8_T_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-KOI8_T-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_KOI8_U_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-KOI8_U-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_KOI8_U_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-KOI8_U-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_KSC5636_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-KSC5636-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_LATIN_GREEK_1_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-LATIN_GREEK_1-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_LATIN_GREEK_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-LATIN_GREEK-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_MACINTOSH_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-MACINTOSH-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_MACINTOSH_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-MACINTOSH-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_MAC_IS_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-MAC_IS-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_MAC_SAMI_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-MAC_SAMI-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_MAC_UK_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-MAC_UK-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_MSZ_7795_3_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-MSZ_7795.3-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_NATS_DANO_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-NATS_DANO-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_NATS_SEFI_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-NATS_SEFI-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_NC_NC00_10_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-NC_NC00_10-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_NF_Z_62_010_1973_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-NF_Z_62_010_1973-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_NF_Z_62_010_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-NF_Z_62_010-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_NS_4551_1_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-NS_4551_1-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_NS_4551_2_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-NS_4551_2-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_PT154_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-PT154-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_PT2_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-PT2-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_PT_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-PT-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_RK1048_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-RK1048-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_SEN_850200_B_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-SEN_850200_B-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_SEN_850200_C_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-SEN_850200_C-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_SJIS_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-SJIS-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_SJIS_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-SJIS-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_T_61_8BIT_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-T.61_8BIT-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_TIS_620_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-TIS_620-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_UHC_2_1_2: Encoding = parse_ucm(&request_mapping_file("glibc-UHC-2.1.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_UHC_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-UHC-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_VISCII_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-VISCII-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref GLIBC_WIN_SAMI_2_2_3_3: Encoding = parse_ucm(&request_mapping_file("glibc-WIN_SAMI_2-2.3.3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_BIG5_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-big5-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1140_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1140-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1141_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1141-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1142_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1142-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1143_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1143-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1144_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1144-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1145_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1145-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1146_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1146-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1147_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1147-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1148_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1148-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1149_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1149-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1250_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1250-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1251_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1251-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1252_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1252-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1253_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1253-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1254_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1254-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1255_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1255-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1256_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1256-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1257_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1257-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP1258_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp1258-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP437_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp437-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP737_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp737-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP775_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp775-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP850_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp850-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP852_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp852-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP855_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp855-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP857_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp857-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP860_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp860-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP861_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp861-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP862_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp862-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP863_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp863-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP864_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp864-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP865_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp865-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP866_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp866-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP869_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp869-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_CP874_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-cp874-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_EUCJP0201_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-eucJP0201-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_EUCJP_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-eucJP-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_EUCJPMS_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-eucJPMS-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_EUCKR_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-eucKR-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_EUCTW_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-eucTW-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_GREEE_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-greee-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_HKBIG5_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-hkbig5-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_HP15CN_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-hp15CN-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ISO81_11_0: Encoding = parse_ucm(&request_mapping_file("hpux-iso81-11.0").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ISO815_11_0: Encoding = parse_ucm(&request_mapping_file("hpux-iso815-11.0").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ISO82_11_0: Encoding = parse_ucm(&request_mapping_file("hpux-iso82-11.0").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ISO85_11_0: Encoding = parse_ucm(&request_mapping_file("hpux-iso85-11.0").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ISO86_11_0: Encoding = parse_ucm(&request_mapping_file("hpux-iso86-11.0").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ISO87_11_0: Encoding = parse_ucm(&request_mapping_file("hpux-iso87-11.0").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ISO87_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-iso87-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ISO88_11_0: Encoding = parse_ucm(&request_mapping_file("hpux-iso88-11.0").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ISO89_11_0: Encoding = parse_ucm(&request_mapping_file("hpux-iso89-11.0").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ROC15_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-roc15-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_ROMA8_11_0: Encoding = parse_ucm(&request_mapping_file("hpux-roma8-11.0").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_SJIS0201_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-sjis0201-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_SJIS_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-sjis-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_SJISMS_11_11: Encoding = parse_ucm(&request_mapping_file("hpux-sjisMS-11.11").unwrap()).unwrap();
}
lazy_static! {
    pub static ref HPUX_THAI8_11_0: Encoding = parse_ucm(&request_mapping_file("hpux-thai8-11.0").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1004_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1004_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1006_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1006_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1006_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1006_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1008_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1008_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1008_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1008_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1009_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1009_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1010_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1010_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1011_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1011_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1012_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1012_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1013_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1013_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1014_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1014_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1015_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1015_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1016_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1016_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1017_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1017_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1018_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1018_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1019_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1019_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1020_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1020_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1021_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1021_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1023_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1023_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1025_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1025_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1026_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1026_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1027_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1027_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1040_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1040_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1041_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1041_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1042_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1042_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1043_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1043_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1046_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1046_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1047_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1047_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1051_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1051_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1088_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1088_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1089_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1089_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1097_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1097_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1097_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1097_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1098_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1098_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1098_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1098_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1100_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1100_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1101_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1101_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1102_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1102_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1103_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1103_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1104_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1104_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1105_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1105_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1106_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1106_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1107_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1107_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1112_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1112_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1114_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1114_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1114_P100_2001: Encoding = parse_ucm(&request_mapping_file("ibm-1114_P100-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1115_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1115_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1122_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1122_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1123_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1123_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1124_X100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-1124_X100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1125_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1125_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1126_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1126_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1126_P100_P100_1997_U3: Encoding = parse_ucm(&request_mapping_file("ibm-1126_P100_P100-1997_U3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1127_P100_2004: Encoding = parse_ucm(&request_mapping_file("ibm-1127_P100-2004").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1129_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1129_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1130_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1130_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1131_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1131_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1132_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1132_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1132_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-1132_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1133_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1133_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1137_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1137_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1137_PMOD_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1137_PMOD-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1140_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1140_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1141_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1141_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1142_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1142_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1143_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1143_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1144_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1144_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1145_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1145_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1146_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1146_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1147_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1147_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1148_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1148_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1149_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1149_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1153_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1153_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1154_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1154_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1155_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1155_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1156_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1156_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1157_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1157_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1158_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1158_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1159_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1159_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1160_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1160_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1161_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1161_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1162_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1162_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1163_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1163_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1164_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1164_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1165_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-1165_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1166_P100_2002: Encoding = parse_ucm(&request_mapping_file("ibm-1166_P100-2002").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1167_P100_2002: Encoding = parse_ucm(&request_mapping_file("ibm-1167_P100-2002").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1168_P100_2002: Encoding = parse_ucm(&request_mapping_file("ibm-1168_P100-2002").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1174_X100_2007: Encoding = parse_ucm(&request_mapping_file("ibm-1174_X100-2007").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1250_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1250_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1251_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1251_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1252_P100_2000: Encoding = parse_ucm(&request_mapping_file("ibm-1252_P100-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1253_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1253_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1254_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1254_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1255_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1255_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1256_P110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1256_P110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1257_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1257_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1258_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1258_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_12712_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-12712_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1275_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1275_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1275_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1275_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1276_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1276_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1277_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1277_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1280_P100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-1280_P100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1281_P100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-1281_P100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1282_P100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-1282_P100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1283_P100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-1283_P100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1284_P100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-1284_P100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1285_P100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-1285_P100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13121_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-13121_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13124_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-13124_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13124_P10A_1995: Encoding = parse_ucm(&request_mapping_file("ibm-13124_P10A-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13125_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-13125_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13140_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-13140_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13143_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-13143_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13145_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-13145_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13156_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-13156_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13157_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-13157_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13162_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-13162_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13218_P100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-13218_P100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1350_P110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1350_P110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1351_P110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1351_P110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1362_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1362_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1362_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1362_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1363_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1363_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1363_P10A_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1363_P10A-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1363_P10B_1998: Encoding = parse_ucm(&request_mapping_file("ibm-1363_P10B-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1363_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1363_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1363_P11A_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1363_P11A-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1363_P11B_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1363_P11B-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1363_P11C_2006: Encoding = parse_ucm(&request_mapping_file("ibm-1363_P11C-2006").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1364_P100_2007: Encoding = parse_ucm(&request_mapping_file("ibm-1364_P100-2007").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1364_P110_2007: Encoding = parse_ucm(&request_mapping_file("ibm-1364_P110-2007").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_13676_P102_2001: Encoding = parse_ucm(&request_mapping_file("ibm-13676_P102-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1370_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1370_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1370_X100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1370_X100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1371_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1371_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1371_X100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1371_X100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1373_P100_2002: Encoding = parse_ucm(&request_mapping_file("ibm-1373_P100-2002").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1374_P100_2005: Encoding = parse_ucm(&request_mapping_file("ibm-1374_P100-2005").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1374_P100_P100_2005_MS: Encoding = parse_ucm(&request_mapping_file("ibm-1374_P100_P100-2005_MS").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1375_P100_2004: Encoding = parse_ucm(&request_mapping_file("ibm-1375_P100-2004").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1375_P100_2006: Encoding = parse_ucm(&request_mapping_file("ibm-1375_P100-2006").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1375_P100_2007: Encoding = parse_ucm(&request_mapping_file("ibm-1375_P100-2007").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1375_P100_2008: Encoding = parse_ucm(&request_mapping_file("ibm-1375_P100-2008").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1375_X100_2004: Encoding = parse_ucm(&request_mapping_file("ibm-1375_X100-2004").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1377_P100_2006: Encoding = parse_ucm(&request_mapping_file("ibm-1377_P100-2006").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1377_P100_2008: Encoding = parse_ucm(&request_mapping_file("ibm-1377_P100-2008").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1377_P100_P100_2006_U3: Encoding = parse_ucm(&request_mapping_file("ibm-1377_P100_P100-2006_U3").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1380_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1380_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1380_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1380_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1381_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1381_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1381_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1381_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1382_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1382_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1382_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-1382_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1383_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1383_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1383_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1383_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1385_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1385_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1385_P100_2005: Encoding = parse_ucm(&request_mapping_file("ibm-1385_P100-2005").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1386_P100_2001: Encoding = parse_ucm(&request_mapping_file("ibm-1386_P100-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1386_P110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-1386_P110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1388_P103_2001: Encoding = parse_ucm(&request_mapping_file("ibm-1388_P103-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1388_P110_2000: Encoding = parse_ucm(&request_mapping_file("ibm-1388_P110-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1390_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1390_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1390_P110_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1390_P110-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1399_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-1399_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_1399_P110_2003: Encoding = parse_ucm(&request_mapping_file("ibm-1399_P110-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_16684_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-16684_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_16684_P110_2003: Encoding = parse_ucm(&request_mapping_file("ibm-16684_P110-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_16804_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-16804_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_17221_P100_2001: Encoding = parse_ucm(&request_mapping_file("ibm-17221_P100-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_17240_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-17240_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_17248_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-17248_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_20780_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-20780_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_21344_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-21344_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_21427_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-21427_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_21427_X100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-21427_X100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_25546_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-25546_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_256_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-256_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_259_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-259_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_259_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-259_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_273_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-273_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_274_P100_2000: Encoding = parse_ucm(&request_mapping_file("ibm-274_P100-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_275_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-275_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_277_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-277_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_278_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-278_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_280_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-280_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_282_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-282_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_284_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-284_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_285_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-285_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_286_P100_2003: Encoding = parse_ucm(&request_mapping_file("ibm-286_P100-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_28709_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-28709_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_290_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-290_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_293_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-293_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_293_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-293_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_297_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-297_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_300_P110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-300_P110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_300_P120_2006: Encoding = parse_ucm(&request_mapping_file("ibm-300_P120-2006").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_300_X110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-300_X110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_301_P110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-301_P110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_301_X110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-301_X110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_33058_P100_2000: Encoding = parse_ucm(&request_mapping_file("ibm-33058_P100-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_33722_P120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-33722_P120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_33722_P12A_1999: Encoding = parse_ucm(&request_mapping_file("ibm-33722_P12A-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_33722_P12A_P12A_2004_U2: Encoding = parse_ucm(&request_mapping_file("ibm-33722_P12A_P12A-2004_U2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_33722_P12A_P12A_2009_U2: Encoding = parse_ucm(&request_mapping_file("ibm-33722_P12A_P12A-2009_U2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_367_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-367_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_37_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-37_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_420_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-420_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_420_X120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-420_X120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_423_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-423_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_424_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-424_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_425_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-425_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_437_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-437_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4517_P100_2005: Encoding = parse_ucm(&request_mapping_file("ibm-4517_P100-2005").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4899_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-4899_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4904_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4904_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4909_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-4909_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4930_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-4930_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4930_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-4930_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4933_P100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-4933_P100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4933_P100_2002: Encoding = parse_ucm(&request_mapping_file("ibm-4933_P100-2002").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4944_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4944_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4945_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4945_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4948_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-4948_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4951_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-4951_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4952_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-4952_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4954_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4954_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4955_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4955_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4956_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4956_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4957_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4957_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4958_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4958_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4959_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4959_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4960_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-4960_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4960_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-4960_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4961_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4961_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4962_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4962_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4963_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-4963_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_4971_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-4971_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_500_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-500_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5012_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5012_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5026_P120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5026_P120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5026_X120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5026_X120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5035_P120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5035_P120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5035_P120_P12A_2005_U2: Encoding = parse_ucm(&request_mapping_file("ibm-5035_P120_P12A-2005_U2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5035_X120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5035_X120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5039_P110_1996: Encoding = parse_ucm(&request_mapping_file("ibm-5039_P110-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5039_P11A_1998: Encoding = parse_ucm(&request_mapping_file("ibm-5039_P11A-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5048_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-5048_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5049_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-5049_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5050_P120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5050_P120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5050_P12A_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5050_P12A-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5067_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-5067_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5104_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5104_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5123_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5123_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5142_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-5142_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5210_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5210_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5233_P100_2011: Encoding = parse_ucm(&request_mapping_file("ibm-5233_P100-2011").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5346_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-5346_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5347_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-5347_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5348_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-5348_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5349_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-5349_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5350_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-5350_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5351_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-5351_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5352_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-5352_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5353_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-5353_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5354_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-5354_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_53685_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-53685_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_54191_P100_2006: Encoding = parse_ucm(&request_mapping_file("ibm-54191_P100-2006").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5470_P100_2005: Encoding = parse_ucm(&request_mapping_file("ibm-5470_P100-2005").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5470_P100_P100_2005_MS: Encoding = parse_ucm(&request_mapping_file("ibm-5470_P100_P100-2005_MS").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5471_P100_2006: Encoding = parse_ucm(&request_mapping_file("ibm-5471_P100-2006").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5471_P100_2007: Encoding = parse_ucm(&request_mapping_file("ibm-5471_P100-2007").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5473_P100_2006: Encoding = parse_ucm(&request_mapping_file("ibm-5473_P100-2006").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5478_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-5478_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5486_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5486_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5487_P100_2001: Encoding = parse_ucm(&request_mapping_file("ibm-5487_P100-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5488_P100_2001: Encoding = parse_ucm(&request_mapping_file("ibm-5488_P100-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_5495_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-5495_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_62383_P100_2007: Encoding = parse_ucm(&request_mapping_file("ibm-62383_P100-2007").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_720_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-720_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_737_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-737_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_775_P100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-775_P100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_803_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-803_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_806_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-806_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_808_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-808_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_813_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-813_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_819_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-819_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_833_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-833_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_834_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-834_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_834_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-834_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_835_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-835_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_835_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-835_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_836_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-836_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_837_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-837_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_837_P100_2011: Encoding = parse_ucm(&request_mapping_file("ibm-837_P100-2011").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_837_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-837_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_838_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-838_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_8482_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-8482_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_848_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-848_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_849_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-849_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_850_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-850_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_851_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-851_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_852_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-852_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_855_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-855_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_856_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-856_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_857_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-857_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_858_P100_1997: Encoding = parse_ucm(&request_mapping_file("ibm-858_P100-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_859_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-859_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_860_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-860_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_8612_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-8612_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_8612_X110_1995: Encoding = parse_ucm(&request_mapping_file("ibm-8612_X110-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_861_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-861_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_862_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-862_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_863_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-863_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_864_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-864_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_864_X120_2012: Encoding = parse_ucm(&request_mapping_file("ibm-864_X120-2012").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_865_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-865_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_866_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-866_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_867_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-867_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_868_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-868_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_868_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-868_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_869_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-869_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_870_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-870_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_871_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-871_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_872_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-872_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_874_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-874_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_875_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-875_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_878_P100_1996: Encoding = parse_ucm(&request_mapping_file("ibm-878_P100-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_880_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-880_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_891_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-891_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_895_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-895_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_896_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-896_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_897_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-897_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9005_X100_2005: Encoding = parse_ucm(&request_mapping_file("ibm-9005_X100-2005").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9005_X110_2007: Encoding = parse_ucm(&request_mapping_file("ibm-9005_X110-2007").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_901_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-901_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9027_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-9027_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9027_X100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-9027_X100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_902_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-902_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9030_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-9030_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_903_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-903_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9042_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-9042_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9044_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-9044_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9048_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-9048_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9049_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-9049_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_904_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-904_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9056_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-9056_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_905_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-905_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9061_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-9061_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9064_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-9064_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9066_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-9066_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9067_X100_2005: Encoding = parse_ucm(&request_mapping_file("ibm-9067_X100-2005").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_912_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-912_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_913_P100_2000: Encoding = parse_ucm(&request_mapping_file("ibm-913_P100-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9145_P110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-9145_P110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9145_X110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-9145_X110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_914_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-914_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_915_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-915_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_916_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-916_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_918_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-918_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_918_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-918_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_920_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-920_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_921_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-921_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_922_P100_1999: Encoding = parse_ucm(&request_mapping_file("ibm-922_P100-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9238_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-9238_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_923_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-923_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_924_P100_1998: Encoding = parse_ucm(&request_mapping_file("ibm-924_P100-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_926_P100_2000: Encoding = parse_ucm(&request_mapping_file("ibm-926_P100-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_927_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-927_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_927_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-927_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_928_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-928_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9306_P101_2000: Encoding = parse_ucm(&request_mapping_file("ibm-9306_P101-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_930_P120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-930_P120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_930_P120_P12A_2006_U2: Encoding = parse_ucm(&request_mapping_file("ibm-930_P120_P12A-2006_U2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_930_X120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-930_X120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_931_P120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-931_P120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_931_X120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-931_X120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_932_P120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-932_P120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_932_P12A_1999: Encoding = parse_ucm(&request_mapping_file("ibm-932_P12A-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_932_P12A_P12A_2000_U2: Encoding = parse_ucm(&request_mapping_file("ibm-932_P12A_P12A-2000_U2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_933_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-933_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_933_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-933_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_935_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-935_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_935_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-935_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_937_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-937_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_937_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-937_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_939_P120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-939_P120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_939_P120_P12A_2005_U2: Encoding = parse_ucm(&request_mapping_file("ibm-939_P120_P12A-2005_U2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_939_X120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-939_X120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_941_P120_1996: Encoding = parse_ucm(&request_mapping_file("ibm-941_P120-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_941_P12A_1996: Encoding = parse_ucm(&request_mapping_file("ibm-941_P12A-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_941_P130_2001: Encoding = parse_ucm(&request_mapping_file("ibm-941_P130-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_941_P13A_2001: Encoding = parse_ucm(&request_mapping_file("ibm-941_P13A-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_941_X110_1996: Encoding = parse_ucm(&request_mapping_file("ibm-941_X110-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_941_X11A_1996: Encoding = parse_ucm(&request_mapping_file("ibm-941_X11A-1996").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_942_P120_1999: Encoding = parse_ucm(&request_mapping_file("ibm-942_P120-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_942_P12A_1999: Encoding = parse_ucm(&request_mapping_file("ibm-942_P12A-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_942_P12A_P12A_2000_U2: Encoding = parse_ucm(&request_mapping_file("ibm-942_P12A_P12A-2000_U2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_943_P130_1999: Encoding = parse_ucm(&request_mapping_file("ibm-943_P130-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_943_P14A_1999: Encoding = parse_ucm(&request_mapping_file("ibm-943_P14A-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_943_P15A_2003: Encoding = parse_ucm(&request_mapping_file("ibm-943_P15A-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9444_P100_2001: Encoding = parse_ucm(&request_mapping_file("ibm-9444_P100-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9444_P100_2005: Encoding = parse_ucm(&request_mapping_file("ibm-9444_P100-2005").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9444_P100_P100_2005_MS: Encoding = parse_ucm(&request_mapping_file("ibm-9444_P100_P100-2005_MS").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9447_P100_2002: Encoding = parse_ucm(&request_mapping_file("ibm-9447_P100-2002").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9448_X100_2005: Encoding = parse_ucm(&request_mapping_file("ibm-9448_X100-2005").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9449_P100_2002: Encoding = parse_ucm(&request_mapping_file("ibm-9449_P100-2002").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_944_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-944_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_944_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-944_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_946_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-946_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_947_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-947_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_947_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-947_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_948_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-948_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_948_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-948_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_949_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-949_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_949_P11A_1999: Encoding = parse_ucm(&request_mapping_file("ibm-949_P11A-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_949_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-949_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_950_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-950_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_950_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-950_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_951_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-951_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_951_X100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-951_X100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_952_P110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-952_P110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_953_P100_2000: Encoding = parse_ucm(&request_mapping_file("ibm-953_P100-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_954_P101_2007: Encoding = parse_ucm(&request_mapping_file("ibm-954_P101-2007").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_955_P110_1997: Encoding = parse_ucm(&request_mapping_file("ibm-955_P110-1997").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9577_P100_2001: Encoding = parse_ucm(&request_mapping_file("ibm-9577_P100-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_9580_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-9580_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_960_P100_2000: Encoding = parse_ucm(&request_mapping_file("ibm-960_P100-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_963_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-963_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_964_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-964_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_964_X110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-964_X110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_970_P110_1999: Encoding = parse_ucm(&request_mapping_file("ibm-970_P110-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_970_P110_P110_2006_U2: Encoding = parse_ucm(&request_mapping_file("ibm-970_P110_P110-2006_U2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref IBM_971_P100_1995: Encoding = parse_ucm(&request_mapping_file("ibm-971_P100-1995").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_10_1998: Encoding = parse_ucm(&request_mapping_file("iso-8859_10-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_11_2001: Encoding = parse_ucm(&request_mapping_file("iso-8859_11-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_1_1998: Encoding = parse_ucm(&request_mapping_file("iso-8859_1-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_13_1998: Encoding = parse_ucm(&request_mapping_file("iso-8859_13-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_14_1998: Encoding = parse_ucm(&request_mapping_file("iso-8859_14-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_15_1999: Encoding = parse_ucm(&request_mapping_file("iso-8859_15-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_16_2001: Encoding = parse_ucm(&request_mapping_file("iso-8859_16-2001").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_2_1999: Encoding = parse_ucm(&request_mapping_file("iso-8859_2-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_3_1999: Encoding = parse_ucm(&request_mapping_file("iso-8859_3-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_4_1998: Encoding = parse_ucm(&request_mapping_file("iso-8859_4-1998").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_5_1999: Encoding = parse_ucm(&request_mapping_file("iso-8859_5-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_6_1999: Encoding = parse_ucm(&request_mapping_file("iso-8859_6-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_7_1987: Encoding = parse_ucm(&request_mapping_file("iso-8859_7-1987").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_7_2003: Encoding = parse_ucm(&request_mapping_file("iso-8859_7-2003").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_8_1999: Encoding = parse_ucm(&request_mapping_file("iso-8859_8-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref ISO_8859_9_1999: Encoding = parse_ucm(&request_mapping_file("iso-8859_9-1999").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ASCII_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ASCII-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_BIG5_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Big5-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP037_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp037-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1006_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1006-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1025_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1025-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1026_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1026-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1097_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1097-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1098_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1098-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1112_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1112-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1122_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1122-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1123_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1123-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1124_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1124-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1250_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1250-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1251_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1251-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1252_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1252-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1253_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1253-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1254_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1254-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1255_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1255-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1256_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1256-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1257_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1257-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1258_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1258-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1381_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1381-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1383_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1383-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1390A_1_6_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1390A-1.6_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP1399A_1_6_P: Encoding = parse_ucm(&request_mapping_file("java-Cp1399A-1.6_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP273_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp273-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP277_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp277-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP278_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp278-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP280_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp280-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP284_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp284-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP285_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp285-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP297_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp297-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP33722_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp33722-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP420_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp420-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP420S_1_6_P: Encoding = parse_ucm(&request_mapping_file("java-Cp420s-1.6_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP424_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp424-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP437_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp437-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP500_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp500-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP737_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp737-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP775_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp775-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP838_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp838-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP850_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp850-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP852_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp852-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP855_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp855-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP856_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp856-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP857_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp857-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP860_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp860-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP861_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp861-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP862_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp862-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP863_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp863-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP864_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp864-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP865_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp865-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP866_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp866-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP868_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp868-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP869_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp869-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP870_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp870-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP871_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp871-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP874_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp874-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP875_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp875-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP918_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp918-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP921_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp921-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP922_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp922-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP930_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp930-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP933_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp933-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP935_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp935-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP937_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp937-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP939_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp939-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP942_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp942-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP942C_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp942C-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP943_1_2_2: Encoding = parse_ucm(&request_mapping_file("java-Cp943-1.2.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP943C_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp943C-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP948_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp948-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP949_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp949-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP949C_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp949C-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP950_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp950-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP964_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp964-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_CP970_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Cp970-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_EUC_CN_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-EUC_CN-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_EUC_JP_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-EUC_JP-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_EUC_JP_LINUX_1_6_P: Encoding = parse_ucm(&request_mapping_file("java-euc_jp_linux-1.6_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_EUC_KR_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-EUC_KR-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_EUC_TW_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-EUC_TW-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO2022JP_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO2022JP-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO2022KR_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO2022KR-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO8859_1_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO8859_1-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO8859_13_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO8859_13-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO8859_2_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO8859_2-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO8859_3_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO8859_3-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO8859_4_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO8859_4-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO8859_5_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO8859_5-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO8859_6_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO8859_6-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO8859_7_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO8859_7-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO8859_8_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO8859_8-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_ISO8859_9_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-ISO8859_9-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_JOHAB_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-Johab-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_KOI8_R_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-KOI8_R-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_MS874_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-MS874-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_MS932_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-MS932-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_MS949_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-MS949-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_SJIS_0213_1_6_P: Encoding = parse_ucm(&request_mapping_file("java-sjis_0213-1.6_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_SJIS_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-SJIS-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref JAVA_TIS620_1_3_P: Encoding = parse_ucm(&request_mapping_file("java-TIS620-1.3_P").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_0_1_10_2: Encoding = parse_ucm(&request_mapping_file("macos-0_1-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_0_2_10_2: Encoding = parse_ucm(&request_mapping_file("macos-0_2-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1024_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1024-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1040_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1040-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1049_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1049-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1057_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1057-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1059_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1059-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1280_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1280-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1281_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1281-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1282_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1282-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1283_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1283-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1284_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1284-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1285_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1285-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1286_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1286-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1287_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1287-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1288_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1288-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_1536_10_2: Encoding = parse_ucm(&request_mapping_file("macos-1536-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_21_10_5: Encoding = parse_ucm(&request_mapping_file("macos-21-10.5").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_2562_10_2: Encoding = parse_ucm(&request_mapping_file("macos-2562-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_2563_10_2: Encoding = parse_ucm(&request_mapping_file("macos-2563-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_2566_10_2: Encoding = parse_ucm(&request_mapping_file("macos-2566-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_2817_10_2: Encoding = parse_ucm(&request_mapping_file("macos-2817-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_29_10_2: Encoding = parse_ucm(&request_mapping_file("macos-29-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_3074_10_2: Encoding = parse_ucm(&request_mapping_file("macos-3074-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_33_10_5: Encoding = parse_ucm(&request_mapping_file("macos-33-10.5").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_34_10_2: Encoding = parse_ucm(&request_mapping_file("macos-34-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_35_10_2: Encoding = parse_ucm(&request_mapping_file("macos-35-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_36_1_10_2: Encoding = parse_ucm(&request_mapping_file("macos-36_1-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_36_2_10_2: Encoding = parse_ucm(&request_mapping_file("macos-36_2-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_37_2_10_2: Encoding = parse_ucm(&request_mapping_file("macos-37_2-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_37_3_10_2: Encoding = parse_ucm(&request_mapping_file("macos-37_3-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_37_4_10_2: Encoding = parse_ucm(&request_mapping_file("macos-37_4-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_37_5_10_2: Encoding = parse_ucm(&request_mapping_file("macos-37_5-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_38_1_10_2: Encoding = parse_ucm(&request_mapping_file("macos-38_1-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_38_2_10_2: Encoding = parse_ucm(&request_mapping_file("macos-38_2-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_513_10_2: Encoding = parse_ucm(&request_mapping_file("macos-513-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_514_10_2: Encoding = parse_ucm(&request_mapping_file("macos-514-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_515_10_2: Encoding = parse_ucm(&request_mapping_file("macos-515-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_516_10_2: Encoding = parse_ucm(&request_mapping_file("macos-516-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_517_10_2: Encoding = parse_ucm(&request_mapping_file("macos-517-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_518_10_2: Encoding = parse_ucm(&request_mapping_file("macos-518-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_519_10_2: Encoding = parse_ucm(&request_mapping_file("macos-519-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_520_10_2: Encoding = parse_ucm(&request_mapping_file("macos-520-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_521_10_2: Encoding = parse_ucm(&request_mapping_file("macos-521-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_527_10_2: Encoding = parse_ucm(&request_mapping_file("macos-527-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_6_10_2: Encoding = parse_ucm(&request_mapping_file("macos-6-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_6_2_10_4: Encoding = parse_ucm(&request_mapping_file("macos-6_2-10.4").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_7_1_10_2: Encoding = parse_ucm(&request_mapping_file("macos-7_1-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_7_2_10_2: Encoding = parse_ucm(&request_mapping_file("macos-7_2-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref MACOS_7_3_10_2: Encoding = parse_ucm(&request_mapping_file("macos-7_3-10.2").unwrap()).unwrap();
}
lazy_static! {
    pub static ref OSD_EBCDIC_DF03_IRV: Encoding = parse_ucm(&request_mapping_file("osd-EBCDIC-DF03-IRV").unwrap()).unwrap();
}
lazy_static! {
    pub static ref OSD_EBCDIC_DF04_15: Encoding = parse_ucm(&request_mapping_file("osd-EBCDIC-DF04-15").unwrap()).unwrap();
}
lazy_static! {
    pub static ref OSD_EBCDIC_DF04_1: Encoding = parse_ucm(&request_mapping_file("osd-EBCDIC-DF04-1").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_5601_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-5601-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_646_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-646-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_10_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_10-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_1_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_1-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_15_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_15-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_2_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_2-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_3_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_3-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_4_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_4-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_5_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_5-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_6_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_6-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_7_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_7-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_8_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_8-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_8859_9_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-8859_9-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_EUCJP_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-eucJP-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_EUC_KR_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-EUC_KR-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_EUCTH_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-eucTH-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_KOI8_R_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-KOI8_R-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_PCK_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-PCK-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_ZH_CN_CP935_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-zh_CN_cp935-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_ZH_CN_EUC_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-zh_CN.euc-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_ZH_CN_GBK_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-zh_CN.gbk-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_ZH_HK_HKSCS_5_9: Encoding = parse_ucm(&request_mapping_file("solaris-zh_HK.hkscs-5.9").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_ZH_TW_BIG5_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-zh_TW_big5-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_ZH_TW_CP937_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-zh_TW_cp937-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref SOLARIS_ZH_TW_EUC_2_7: Encoding = parse_ucm(&request_mapping_file("solaris-zh_TW_euc-2.7").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10000_2000: Encoding = parse_ucm(&request_mapping_file("windows-10000-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10001_2000: Encoding = parse_ucm(&request_mapping_file("windows-10001-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10002_2000: Encoding = parse_ucm(&request_mapping_file("windows-10002-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10003_2000: Encoding = parse_ucm(&request_mapping_file("windows-10003-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10004_2000: Encoding = parse_ucm(&request_mapping_file("windows-10004-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10005_2000: Encoding = parse_ucm(&request_mapping_file("windows-10005-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10006_2000: Encoding = parse_ucm(&request_mapping_file("windows-10006-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10007_2000: Encoding = parse_ucm(&request_mapping_file("windows-10007-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10008_2000: Encoding = parse_ucm(&request_mapping_file("windows-10008-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10010_2000: Encoding = parse_ucm(&request_mapping_file("windows-10010-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10017_2000: Encoding = parse_ucm(&request_mapping_file("windows-10017-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10021_2000: Encoding = parse_ucm(&request_mapping_file("windows-10021-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10029_2000: Encoding = parse_ucm(&request_mapping_file("windows-10029-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10079_2000: Encoding = parse_ucm(&request_mapping_file("windows-10079-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10081_2000: Encoding = parse_ucm(&request_mapping_file("windows-10081-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_10082_2000: Encoding = parse_ucm(&request_mapping_file("windows-10082-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1026_2000: Encoding = parse_ucm(&request_mapping_file("windows-1026-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1047_2000: Encoding = parse_ucm(&request_mapping_file("windows-1047-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1140_2000: Encoding = parse_ucm(&request_mapping_file("windows-1140-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1141_2000: Encoding = parse_ucm(&request_mapping_file("windows-1141-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1142_2000: Encoding = parse_ucm(&request_mapping_file("windows-1142-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1143_2000: Encoding = parse_ucm(&request_mapping_file("windows-1143-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1144_2000: Encoding = parse_ucm(&request_mapping_file("windows-1144-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1145_2000: Encoding = parse_ucm(&request_mapping_file("windows-1145-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1146_2000: Encoding = parse_ucm(&request_mapping_file("windows-1146-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1147_2000: Encoding = parse_ucm(&request_mapping_file("windows-1147-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1148_2000: Encoding = parse_ucm(&request_mapping_file("windows-1148-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1149_2000: Encoding = parse_ucm(&request_mapping_file("windows-1149-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1250_2000: Encoding = parse_ucm(&request_mapping_file("windows-1250-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1251_2000: Encoding = parse_ucm(&request_mapping_file("windows-1251-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1252_2000: Encoding = parse_ucm(&request_mapping_file("windows-1252-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1253_2000: Encoding = parse_ucm(&request_mapping_file("windows-1253-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1254_2000: Encoding = parse_ucm(&request_mapping_file("windows-1254-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1255_2000: Encoding = parse_ucm(&request_mapping_file("windows-1255-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1256_2000: Encoding = parse_ucm(&request_mapping_file("windows-1256-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1257_2000: Encoding = parse_ucm(&request_mapping_file("windows-1257-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1258_2000: Encoding = parse_ucm(&request_mapping_file("windows-1258-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1258_DB_2013: Encoding = parse_ucm(&request_mapping_file("windows-1258_db-2013").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_1361_2000: Encoding = parse_ucm(&request_mapping_file("windows-1361-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20000_2000: Encoding = parse_ucm(&request_mapping_file("windows-20000-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20001_2000: Encoding = parse_ucm(&request_mapping_file("windows-20001-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20002_2000: Encoding = parse_ucm(&request_mapping_file("windows-20002-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20003_2000: Encoding = parse_ucm(&request_mapping_file("windows-20003-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20004_2000: Encoding = parse_ucm(&request_mapping_file("windows-20004-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20005_2000: Encoding = parse_ucm(&request_mapping_file("windows-20005-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20105_2000: Encoding = parse_ucm(&request_mapping_file("windows-20105-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20106_2000: Encoding = parse_ucm(&request_mapping_file("windows-20106-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20107_2000: Encoding = parse_ucm(&request_mapping_file("windows-20107-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20108_2000: Encoding = parse_ucm(&request_mapping_file("windows-20108-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20127_2000: Encoding = parse_ucm(&request_mapping_file("windows-20127-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20261_2000: Encoding = parse_ucm(&request_mapping_file("windows-20261-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20269_2000: Encoding = parse_ucm(&request_mapping_file("windows-20269-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20273_2000: Encoding = parse_ucm(&request_mapping_file("windows-20273-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20277_2000: Encoding = parse_ucm(&request_mapping_file("windows-20277-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20278_2000: Encoding = parse_ucm(&request_mapping_file("windows-20278-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20280_2000: Encoding = parse_ucm(&request_mapping_file("windows-20280-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20284_2000: Encoding = parse_ucm(&request_mapping_file("windows-20284-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20285_2000: Encoding = parse_ucm(&request_mapping_file("windows-20285-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20290_2000: Encoding = parse_ucm(&request_mapping_file("windows-20290-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20297_2000: Encoding = parse_ucm(&request_mapping_file("windows-20297-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20420_2000: Encoding = parse_ucm(&request_mapping_file("windows-20420-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20423_2000: Encoding = parse_ucm(&request_mapping_file("windows-20423-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20424_2000: Encoding = parse_ucm(&request_mapping_file("windows-20424-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20833_2000: Encoding = parse_ucm(&request_mapping_file("windows-20833-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20838_2000: Encoding = parse_ucm(&request_mapping_file("windows-20838-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20866_2000: Encoding = parse_ucm(&request_mapping_file("windows-20866-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20871_2000: Encoding = parse_ucm(&request_mapping_file("windows-20871-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20880_2000: Encoding = parse_ucm(&request_mapping_file("windows-20880-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20905_2000: Encoding = parse_ucm(&request_mapping_file("windows-20905-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20924_2000: Encoding = parse_ucm(&request_mapping_file("windows-20924-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20932_2000: Encoding = parse_ucm(&request_mapping_file("windows-20932-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20936_2000: Encoding = parse_ucm(&request_mapping_file("windows-20936-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_20949_2000: Encoding = parse_ucm(&request_mapping_file("windows-20949-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_21025_2000: Encoding = parse_ucm(&request_mapping_file("windows-21025-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_21027_2000: Encoding = parse_ucm(&request_mapping_file("windows-21027-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_21866_2000: Encoding = parse_ucm(&request_mapping_file("windows-21866-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28591_2000: Encoding = parse_ucm(&request_mapping_file("windows-28591-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28592_2000: Encoding = parse_ucm(&request_mapping_file("windows-28592-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28593_2000: Encoding = parse_ucm(&request_mapping_file("windows-28593-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28594_2000: Encoding = parse_ucm(&request_mapping_file("windows-28594-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28595_2000: Encoding = parse_ucm(&request_mapping_file("windows-28595-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28596_2000: Encoding = parse_ucm(&request_mapping_file("windows-28596-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28597_2000: Encoding = parse_ucm(&request_mapping_file("windows-28597-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28598_2000: Encoding = parse_ucm(&request_mapping_file("windows-28598-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28599_2000: Encoding = parse_ucm(&request_mapping_file("windows-28599-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28603_VISTA: Encoding = parse_ucm(&request_mapping_file("windows-28603-vista").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_28605_2000: Encoding = parse_ucm(&request_mapping_file("windows-28605-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_37_2000: Encoding = parse_ucm(&request_mapping_file("windows-37-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_38598_2000: Encoding = parse_ucm(&request_mapping_file("windows-38598-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_437_2000: Encoding = parse_ucm(&request_mapping_file("windows-437-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_500_2000: Encoding = parse_ucm(&request_mapping_file("windows-500-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_51932_2006: Encoding = parse_ucm(&request_mapping_file("windows-51932-2006").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_51936_2000: Encoding = parse_ucm(&request_mapping_file("windows-51936-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_51949_2000: Encoding = parse_ucm(&request_mapping_file("windows-51949-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_708_2000: Encoding = parse_ucm(&request_mapping_file("windows-708-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_720_2000: Encoding = parse_ucm(&request_mapping_file("windows-720-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_737_2000: Encoding = parse_ucm(&request_mapping_file("windows-737-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_775_2000: Encoding = parse_ucm(&request_mapping_file("windows-775-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_850_2000: Encoding = parse_ucm(&request_mapping_file("windows-850-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_852_2000: Encoding = parse_ucm(&request_mapping_file("windows-852-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_855_2000: Encoding = parse_ucm(&request_mapping_file("windows-855-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_857_2000: Encoding = parse_ucm(&request_mapping_file("windows-857-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_858_2000: Encoding = parse_ucm(&request_mapping_file("windows-858-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_860_2000: Encoding = parse_ucm(&request_mapping_file("windows-860-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_861_2000: Encoding = parse_ucm(&request_mapping_file("windows-861-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_862_2000: Encoding = parse_ucm(&request_mapping_file("windows-862-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_863_2000: Encoding = parse_ucm(&request_mapping_file("windows-863-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_864_2000: Encoding = parse_ucm(&request_mapping_file("windows-864-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_865_2000: Encoding = parse_ucm(&request_mapping_file("windows-865-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_866_2000: Encoding = parse_ucm(&request_mapping_file("windows-866-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_869_2000: Encoding = parse_ucm(&request_mapping_file("windows-869-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_870_2000: Encoding = parse_ucm(&request_mapping_file("windows-870-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_874_2000: Encoding = parse_ucm(&request_mapping_file("windows-874-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_875_2000: Encoding = parse_ucm(&request_mapping_file("windows-875-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_932_2000: Encoding = parse_ucm(&request_mapping_file("windows-932-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_936_2000: Encoding = parse_ucm(&request_mapping_file("windows-936-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_949_2000: Encoding = parse_ucm(&request_mapping_file("windows-949-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_950_2000: Encoding = parse_ucm(&request_mapping_file("windows-950-2000").unwrap()).unwrap();
}
lazy_static! {
    pub static ref WINDOWS_950_HKSCS_2001: Encoding = parse_ucm(&request_mapping_file("windows-950_hkscs-2001").unwrap()).unwrap();
}
