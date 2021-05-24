//! This module contains a UniCode Mapping (`.ucm`) file format parser and all of the data files
//! available in the Unicode Consortium's `icu-data` repository. For a list, see [`KNOWN_CHARSETS`].
//!
//! Most uses of this library should look like this:
//!
//! ```
//! use icu_data::ucm::{request_mapping_file, parser::parse as parse_ucm};
//!
//! let f = request_mapping_file("java-EUC_JP-1.3_P").unwrap(); // holds the .ucm file as a String
//! let enc = parse_ucm(&f).unwrap(); // holds an `Encoding`
//! /* ... */
//! ```
//!
//! If you only want a single encoding, they're all in the module named [`mappings`]. They are all
//! [`lazy_static`] types, so are only evaluated when used. The evaluation of them can panic,
//! because it is just the code above, but they all work on my machine, and will only ever panic if
//! Brotli decompression or [`tar`] metadata parsing fails.
//!
//! Example:
//! ```
//! use icu_data::ucm::mappings;
//! assert_eq!(mappings::JAVA_EUC_JP_1_3_P.codepoints.len(), 13139);
//! ```

mod charsets;
use charsets::{BROTLI_UCM_ARRAYS, CHARSET_LOOKUP};
pub use charsets::KNOWN_CHARSETS;
mod errors;
pub use errors::IcuDataError;
pub mod mappings;
pub mod parser;
pub use parser::Parser as PestParser;
mod types;
pub use types::{Codepoint, Encoding, EquivalenceType};
mod util;

use brotli;
use tar;

use std::path::PathBuf;

/// Given the name of an encoding known to ICU, return its raw UCM data as a String.
///
/// You should not request encoding filenames (with `.ucm`), but it'll still be understood.
///
/// Internally, this function considers 105 byte arrays stored as static strings. It takes the name
/// of the mapping and looks it up in `CHARSET_LOOKUP`, which tells it what byte array contains the
/// mapping. For example, `CHARSET_LOOKUP["ibm-737_P100-1997.ucm"]` is `58`. So, we know that
/// `BYTES_58` contains the file. `BYTES_58` is defined as:
///
/// ```no_run
/// include_bytes!("../../resources/brotli/ibm-737_P100-1997ibm-775_P100-1996ibm-803_P100-1999ibm-806_P100-1998ibm-808_P100-1999ibm-813_P100-1995ibm-819_P100-1999ibm-833_P100-1995ibm-834_P100-1995ibm-834_X100-1995.ucm.tar.b");
/// ```
///
/// So, we un-Brotli compress the data in `BYTES_58` and then send it through a `.tar` file parser.
/// We iterate through the metadata entries until we find one equal to `ibm-737_P100-1997.ucm`. We
/// then clone that data to a `String` type because only the compressed versions are owned in
/// memory and return.
///
/// This function returns a `Result<_, _>` type because users may provide unknown mappings. The
/// only error you should ever receive is [`IcuDataError::UnknownMappingRequested`], all the valid
/// mappings have been tested and decompress on my machine.
pub fn request_mapping_file(mapping: &str) -> Result<String, IcuDataError> {
    // We do this in case we're given a mapping file with a .ucm extension
	let mapping = util::remove_suffix(mapping, ".ucm");
	let request = format!("{}.ucm", mapping);
	let index = CHARSET_LOOKUP.get(request.as_str()).ok_or(IcuDataError::UnknownMappingRequested)?;
	let mut bytes = BROTLI_UCM_ARRAYS[*index].clone();

	let mut brotli_decompressed = vec![];
	brotli::BrotliDecompress(&mut bytes, &mut brotli_decompressed).or(Err(IcuDataError::BrotliDecompressionFailure))?;
	let mut tar_archive = tar::Archive::new(&*brotli_decompressed);
	// e.g. aix-IBM_858-4.3.6; windows-862-2000
	let mut found = None;
	for entry in tar_archive.entries().or(Err(IcuDataError::TarArchiveEntriesReadError))?.into_iter() {
		let e = entry.or(Err(IcuDataError::TarArchiveEntryParseError))?;
		if *e.path().or(Err(IcuDataError::TarArchivePathParseError))? == PathBuf::from(request.as_str()) {
			found = Some(e);
			break
		}
	}
	let found = found.ok_or_else(||IcuDataError::UnknownMappingRequested)?;
	let begin = found.raw_file_position() as usize;
	let end = begin + found.size() as usize;
	Ok( String::from_utf8(brotli_decompressed[begin..end].to_vec()).or(Err(IcuDataError::MappingFileNotUtf8))? )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fetching_works() {
        // Windows Code Page 862 (Hebrew)
		let _f = request_mapping_file("windows-862-2000").unwrap();
		//eprintln!("{}", &f);
    }

    #[test]
    fn parsing_works() {
        let f = request_mapping_file("java-EUC_JP-1.3_P").unwrap();
        //eprintln!("{}", &f);
        let enc = parser::parse(&f).unwrap();
        assert_eq!(enc.codepoints.len(), 13139);
        assert_eq!(enc.codepoints[0].uni, '\u{0}');
        assert_eq!(enc.states.len(), 5);
        assert_eq!(enc.metadata.len(), 5);
        assert_eq!(enc.metadata["code_set_name"], "java-EUC_JP-1.3_P");
        assert_eq!(enc.metadata["uconv_class"], "MBCS");
    }

    #[test]
    fn known_charsets_works() {
        assert!(KNOWN_CHARSETS.contains(&"windows-862-2000"));
        assert_eq!(KNOWN_CHARSETS.len(), 1049);
    }
}
