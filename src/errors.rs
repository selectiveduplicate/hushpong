use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("chunk type code must be uppercase and lowercase ASCII letters")]
    NonAsciiLetterInChunkType,
}
