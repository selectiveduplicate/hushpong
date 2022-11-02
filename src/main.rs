use std::str::FromStr;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod errors;
mod png;

pub type Result<T> = std::result::Result<T, errors::ChunkTypeErrors>;

fn main() -> Result<()> {
    let ct = chunk_type::ChunkType::from_str("Ru$t");
    if let Err(e) = ct {
        eprintln!("{e}");
    }
    Ok(())
}
