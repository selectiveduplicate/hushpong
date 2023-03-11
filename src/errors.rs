use std::{array::TryFromSliceError, num::TryFromIntError, string::FromUtf8Error};

use thiserror::Error;

#[derive(Debug)]
pub(crate) struct Expectations {
    pub(crate) got: u32,
    pub(crate) expected: u32,
}

#[derive(Error, Debug)]
pub(crate) enum PngError {
    #[error(
        "error: chunk type code must be valid uppercase or lowercase ASCII letters ('A'-'Z' and 'a'-'z')"
    )]
    InvalidByte,
    #[error("error {0}: failed to construct chunk type from string slice")]
    TryFromStrError(#[from] TryFromSliceError),
    #[error("error occurred while interpreting chunk data as string: {}", .0.utf8_error())]
    TryStringFromChunkData(#[from] FromUtf8Error),
    #[error("error occurred while reading bytes from byte slice: {0}")]
    ReadFromByteSlice(#[from] std::io::Error),
    #[error("error {0}: could not convert integer from u32 to usize")]
    TryUsizeFromU32(#[from] TryFromIntError),
    #[error("error: invalid CRC {}, expected {}", .0.got, .0.expected)]
    InvalidCrc(Expectations),
    #[error("error: invalid PNG file signature")]
    InvalidPngSignature,
}
