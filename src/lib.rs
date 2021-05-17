pub enum Codepoint {
    U0(char),
    U1(char),
    U2(char),
    U3(char),
}

pub mod charsets;
pub use charsets::*;
pub use charsets::{BROTLI_UCM_ARRAYS, CHARSET_LOOKUP};
pub mod errors;
use errors::IcuDataError;
mod util;

use brotli;
use pest::Parser;
use pest_derive::Parser;
use tar;

use std::path::PathBuf;

#[derive(Parser)]
#[grammar = "../resources/ucm.pest"]
struct UcmParser;

pub fn request_mapping_file(mapping: &str) -> Result<String, IcuDataError> {
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
    use crate::*;
    #[test]
    fn it_works() {
		let f = request_mapping_file("windows-862-2000").unwrap();
		//eprintln!("{}", &f);
    }

    #[test]
    fn parsing_works() {
        let f = request_mapping_file("windows-862-2000").unwrap();
        let ast = UcmParser::parse(Rule::ucm, &f);
        use pest_ascii_tree::{self, into_ascii_tree};
        eprintln!("{}", into_ascii_tree(ast.unwrap()).unwrap());
    }
}
