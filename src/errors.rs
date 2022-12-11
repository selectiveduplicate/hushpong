use std::{array::TryFromSliceError, string::FromUtf8Error, num::TryFromIntError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PngError {
    #[error(
        "error: chunk type code must be valid uppercase or lowercase ASCII letters ('A'-'Z' and 'a'-'z')"
    )]
    InvalidByte,
    #[error("error: failed to construct chunk type from string slice: {0}")]
    TryFromStrError(#[from] TryFromSliceError),
    #[error("error occurred while interpreting chunk data as string: {}", .0.utf8_error())]
    TryStringFromChunkData(#[from] FromUtf8Error),
    #[error("error occurred while reading bytes from byte slice: {0}")]
    ReadFromByteSlice(#[from] std::io::Error),
    #[error("error while converting between integer types: {0}")]
    TryUsizeFromU32 (#[from] TryFromIntError),
    #[error("invalid CRC {0}, expected {1}")]
    InvalidCrc(u32, u32)
}