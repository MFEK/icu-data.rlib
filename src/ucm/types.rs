/// The "equivalence type" of the Unicode codepoint to the bytestring in the [`Encoding`]. The
/// equivalence types are defined by the Unicode consortium as such:
///
/// ```text
/// # The 1st column is the Unicode scalar value.
/// # The 2nd column is the codepage byte sequence.
/// # The 3rd column is the fallback indicator.
/// # The fallback indicator can have one of the following values:
/// #   |0 for exact 1-1 roundtrip mapping
/// #   |1 for the best fallback codepage byte sequence.
/// #   |2 for the substitution character
/// #   |3 for the best reverse fallback Unicode scaler value
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum EquivalenceType {
    Type0,
    Type1,
    Type2,
    Type3,
}

/// This represents a `CHARMAP` row in a `.ucm` (UniCode Mapping) file.
#[derive(Debug, Clone, PartialEq)]
pub struct Codepoint {
    pub uni: char,
    pub eq_type: EquivalenceType,
    pub bytestring: Vec<u8>,
}

impl Into<char> for Codepoint {
    fn into(self) -> char {
        self.uni
    }
}

/// This represents a single `.ucm` (UniCode Mapping) file.
#[derive(Debug, Clone, PartialEq)]
pub struct Encoding {
    /// Note: does not include `<icu:state>`. Unordered.
    pub metadata: std::collections::HashMap<String, String>,
    /// This list is guaranteed to be in the order it is in the file.
    pub codepoints: Vec<Codepoint>,
    /// Parsing of `states` is left to those who wish to implement them. We provide a more complete
    /// parser of them by using [`UcmParser`](super::parser::UcmParser) with
    /// [`Rule::state_row`](super::parser::Rule::state_row), but matching the rules is up to you.
    /// For more information on their format, see [this page from the ICU User
    /// Guide](https://unicode-org.github.io/icu/userguide/conversion/data.html#state-table-syntax-in-ucm-files).
    pub states: Vec<String>,
}
