use crate::{chunk_type::{ChunkType, self}, chunk};
use crc::CRC_32_ISO_HDLC;

use crate::errors::ChunkTypeErrors;

/// A chunk of a PNG file.
/// Each chunk consits of four parts: length, chunk type, chunk data, and CRC
#[derive(Debug)]
pub struct Chunk {
    /// A 4-byte unsigned integer giving the number
    /// of bytes in the chunk's data field.
    ///
    /// The length counts only the data field,
    /// not itself, the chunk type code, or the CRC.
    /// Its value must not exceed 2^31 bytes.
    length: u32,
    /// A 4-byte chunk type code.
    chunk_type: ChunkType,
    /// The data bytes appropriate to the chunk type, if any.
    /// This field can be of zero length.
    chunk_data: Vec<u8>,
    /// A 4-byte CRC (Cyclic Redundancy Check) calculated
    /// on the preceding bytes in the chunk,
    /// including the chunk type code and chunk data fields, but
    /// not including the length field.
    /// The CRC is always present.
    crc: u32,
}

impl Chunk {
    /// Creates a new Chunk with the specified chunk type 
    /// and chunk data.
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC)
            .checksum(&[chunk_type.bytes().as_slice(), data.as_slice()].concat());

        let data_lenth = data.len();
        println!("Data length is: {data_lenth}");

        Self {
            length: data.len() as u32,
            chunk_type,
            chunk_data: data,
            crc,
        }
    }

    /// Returns the 4-byte CRC value of the chunk.
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Returns the data length of the chunk.
    pub fn length(&self) -> u32 {
        self.length
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkTypeErrors;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut chunked_chunk = value.chunks(4);
        
        let data_length = chunked_chunk.next().unwrap().len();

        let chunk_type_byte_slice: [u8; 4] = chunked_chunk.next().unwrap().try_into()?;
        let chunk_type = TryFrom::try_from(chunk_type_byte_slice)?;

        let mut right_chunked_chunk = value.rchunks(4);

        let _crc = right_chunked_chunk.next();
        
        let chunk_data: Vec<u8> = value[data_length+4..value.len()-4].iter().copied().collect();

        let new_chunk = Self::new(chunk_type, chunk_data);

        Ok(new_chunk)
    }
}

#[cfg(test)]
mod chunk_tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

}
