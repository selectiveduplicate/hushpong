use std::array::TryFromSliceError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChunkTypeErrors {
    #[error(
        "chunk type code must be valid uppercase and lowercase ASCII letters ('A'-'Z' and 'a'-'z')"
    )]
    NonAlphabeticAscii,
    #[error("chunk type code must 4-bytes in length")]
    ByteParseError,
    #[error("chunk type construction from slice failed")]
    TryFourByteSliceFromStr(#[from] TryFromSliceError),
}
