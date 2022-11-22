use std::str::FromStr;
use crc::CRC_32_ISO_HDLC;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod errors;
mod png;

pub type Result<T> = std::result::Result<T, errors::ChunkErrors>;

fn main() -> Result<()> {
    //let ct = chunk_type::ChunkType::from_str("Ru$t");
    //if let Err(e) = ct {
    //    eprintln!("{e}");
    //}
    let chunk_type = chunk_type::ChunkType::from_str("RuSt").unwrap();
    let data = "My life is like an eternal night...".as_bytes().to_vec();

    let crc = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC)
            .checksum(&[chunk_type.bytes().as_slice(), data.as_slice()].concat());
    println!("{crc}");
    Ok(())
}
