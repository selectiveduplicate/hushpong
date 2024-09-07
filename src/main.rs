mod args;
mod chunk;
mod chunk_type;
mod commands;
mod crc32;
mod errors;
mod png;

pub(crate) type Result<T> = std::result::Result<T, errors::PngError>;

fn main() -> Result<()> {
    //let ct = chunk_type::ChunkType::from_str("Ru$t");
    //if let Err(e) = ct {
    //    eprintln!("{e}");
    //}
    //let chunk_type = chunk_type::ChunkType::from_str("RuSt").unwrap();
    //let data = "My life is like an eternal night...".as_bytes().to_vec();

    //let crc = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC)
    //        .checksum(&[chunk_type.bytes().as_slice(), data.as_slice()].concat());
    //println!("{crc}");
    //Ok(())
    let data_length: u32 = 35;
    let chunk_type = "RuSt".as_bytes();
    let message_bytes = "My life is like an eternal night...".as_bytes();
    let crc: u32 = 2591807189;

    let chunk_data: Vec<u8> = data_length
        .to_be_bytes()
        .iter()
        .chain(chunk_type.iter())
        .chain(message_bytes.iter())
        .chain(crc.to_be_bytes().iter())
        .copied()
        .collect();
    if let Err(e) = chunk::Chunk::try_from(chunk_data.as_ref()) {
        println!("{e}");
    }
    Ok(())
}
