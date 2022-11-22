use std::array::TryFromSliceError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChunkTypeErrors {
    #[error(
        "error: chunk type code must be valid uppercase or lowercase ASCII letters ('A'-'Z' and 'a'-'z')"
    )]
    InvalidByteError,
    #[error("error: failed to construct chunk type from string slice")]
    TryFromStrError(#[from] TryFromSliceError),
}