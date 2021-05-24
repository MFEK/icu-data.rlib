/// Error type. You should only ever expect to see `UnknownMappingRequested` unless you're doing
/// development on the library.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IcuDataError {
    UnknownMappingRequested,
    BrotliDecompressionFailure,
    TarArchiveEntriesReadError,
    TarArchiveEntryParseError,
    TarArchivePathParseError,
    MappingFileNotUtf8,
}

use std::fmt;

/// Our version of this just translates the error into a message describing what it does.
impl fmt::Display for IcuDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use IcuDataError::*;
        match self {
            UnknownMappingRequested => write!(f, "Unknown mapping requested. Not in Unicode ICU database?"),
            BrotliDecompressionFailure => write!(f, "Failed to Brotli decompress bytes for mapping. This is a bug, report it."),
            TarArchiveEntriesReadError | TarArchiveEntryParseError | TarArchivePathParseError => write!(f, "Tar archive parse error. This is a bug, please report it."),
            MappingFileNotUtf8 => write!(f, "Mapping file not UTF-8! This is a bug, report it.")
        }
    }
}

use std::error::Error;
impl Error for IcuDataError {}
