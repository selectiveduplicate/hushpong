use std::str::FromStr;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod errors;
mod png;

pub type Result<T> = std::result::Result<T, errors::ChunkTypeErrors>;

fn main() -> Result<()> {
    let ct = chunk_type::ChunkType::from_str("rus$");
    if let Err(e) = ct {
        match e {
            errors::ChunkTypeErrors::TryFourByteSliceFromStr(_) => {
                eprintln!("error: failed to construct valid chunk type bytes from string slice")
            }
            errors::ChunkTypeErrors::NonAlphabeticAscii => {
                eprintln!("error: all characters should be valid ASCII alphabetic")
            }
            _ => {
                eprintln!("failed due to unknown error you error-handling-ignorant fucking dumbass")
            }
        }
    }
    Ok(())
}
