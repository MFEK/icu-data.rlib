//! A `.ucm` file format (UniCode Mapping) Pest parser
use std::collections::HashMap;

use super::{Codepoint, Encoding, EquivalenceType};

use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;
pub use pest::Parser;
use pest::Position;
use pest_derive::Parser;

/// This [`pest`] parser implements the grammar in `../resources/ucm.pest`.
#[derive(Parser)]
#[grammar = "../resources/ucm.pest"]
pub struct UcmParser;

/// Dumps a tree to `stderr` of the `.ucm` data in the argument as seen by the Pest parser. The
/// tree comes from the [`pest_ascii_tree`] crate.
pub fn parse_debug_dump(ucms: &str) {
    let ast = UcmParser::parse(Rule::ucm, ucms).unwrap_or_else(|e|{panic!("{:?}", e);});
    use pest_ascii_tree::into_ascii_tree;
    eprintln!("{}", into_ascii_tree(ast).unwrap());
}

/// Parse a UCM document into an [`Encoding`].
pub fn parse(ucms: &str) -> Result<Encoding, Error<Rule>> {
    fn parse_bytestring(bs: &str) -> Vec<u8> {
        bs.split("\\x")
            .filter(|s| s.trim().len() != 0)
            .map(|s| u8::from_str_radix(s.trim(), 16).unwrap())
            .collect()
    }

    let ucm = match UcmParser::parse(Rule::ucm, ucms)?.next() {
        Some(parsed) => parsed,
        None => {
            Err(Error::new_from_pos(
                ErrorVariant::CustomError {
                    message: "No rules in parsed file?".to_string(),
                },
                Position::new(ucms, 0).unwrap(),
            ))?
        }
    };

    let mut codepoints = vec![];
    let mut metadata = HashMap::new();
    let mut states = vec![];
    for i in ucm.into_inner() {
        let rules: Vec<Pair<_>> = i.clone().into_inner().into_iter().collect();
        match i.as_rule() {
            Rule::unicode_record => {
                let (uni, bytestring, utype) = (&rules[0], &rules[1], &rules[2]);
                debug_assert_eq!(uni.as_rule(), Rule::unicode_inner);
                debug_assert!([Rule::type0, Rule::type1, Rule::type2, Rule::type3].iter().any(|r|utype.as_rule() == *r));
                let uni = char::from_u32(u32::from_str_radix(uni.as_span().as_str(), 16).unwrap())
                    .unwrap();
                let eq_type = match utype.as_rule() {
                    Rule::type0 => EquivalenceType::Type0,
                    Rule::type1 => EquivalenceType::Type1,
                    Rule::type2 => EquivalenceType::Type2,
                    Rule::type3 => EquivalenceType::Type3,
                    _ => unreachable!(),
                };
                let bytestring: Vec<u8> = parse_bytestring(bytestring.as_str());
                codepoints.push(Codepoint {
                    uni,
                    eq_type,
                    bytestring,
                });
            }
            Rule::metadata_record => {
                let (key, value) = (&rules[0], &rules[1]);
                debug_assert_eq!(key.as_rule(), Rule::metadata_key);
                metadata
                    .insert(key.as_str().to_owned(), value.as_str().to_owned());
            }
            Rule::state_record => {
                let state_row = &rules[0];
                debug_assert_eq!(state_row.as_rule(), Rule::state_row);
                states.push(state_row.as_str().to_owned());
            }
            _ => {}
        }
    }
    Ok(Encoding { codepoints, metadata, states })
}
