use std::{array::TryFromSliceError, string::FromUtf8Error};

use thiserror::Error;

use crate::chunk::Chunk;

#[derive(Error, Debug)]
pub enum ChunkErrors {
    #[error(
        "error: chunk type code must be valid uppercase or lowercase ASCII letters ('A'-'Z' and 'a'-'z')"
    )]
    InvalidByteError,
    #[error("error: failed to construct chunk type from string slice")]
    TryFromStrError(#[from] TryFromSliceError),
    #[error("error occurred while interpreting string from chunk data: {}", .0.utf8_error())]
    StringFromUtf8Error(#[from] FromUtf8Error)
}

//impl From<FromUtf8Error> for ChunkErrors {
//    fn from(err: FromUtf8Error) -> Self {
//        ChunkErrors::StringFromUtf8Error(err)
//    }
//}