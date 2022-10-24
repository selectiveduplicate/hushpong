use std::error;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod errors;
mod png;

pub type Result<T> = std::result::Result<T, errors::Errors>;

fn main() -> Result<()> {
    todo!()
}
