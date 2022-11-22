use crate::{
    chunk,
    chunk_type::{self, ChunkType},
};
use crc::CRC_32_ISO_HDLC;

use crate::errors::ChunkErrors;

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
    type Error = ChunkErrors;

    /// Tries to produces a valid `Chunk` from a slice of bytes.
    // The `value` contains the 4-byte data length type, 
    // the 4-byte chunk type,
    // the unspecified amount of chunk data, and 
    // the 4-byte CRC that's calculated based on the chunk 
    // type and chunk data.
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // The first 4 bytes is the chunk length field.
        let mut chunked_chunk = value.chunks(4);
        let data_length = chunked_chunk.next().unwrap().len();

        // The next 4 byte is the chunk type
        let chunk_type_as_byte_slice: [u8; 4] = chunked_chunk.next().unwrap().try_into()?;
        let chunk_type = TryFrom::try_from(chunk_type_as_byte_slice)?;

        // The chunk data should be what's left in `value` after we disregard 
        // the 4-byte data length, the 4-byte chunk type, and the 4-byte CRC.
        let chunk_data: Vec<u8> = value[data_length + 4..value.len() - 4].to_vec();

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
        let data = "My life is like an eternal night..."
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 35);
        assert_eq!(chunk.crc(), 2591807180);
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 35;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "My life is like an eternal night...".as_bytes();
        let crc: u32 = 2591807180;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        assert_eq!(chunk.length(), 35);
        assert_eq!(chunk.crc(), 2591807180);
    }

    #[test]
    pub fn test_chunk_with_invalid_chunk_type() {
        let data_length: u32 = 35;
        let chunk_type = "Ru$t".as_bytes();
        let message_bytes = "My life is like an eternal night...".as_bytes();
        let crc: u32 = 2591807180;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());
        assert!(chunk.is_err());
        assert!(matches!(chunk, Err(ChunkErrors::InvalidByteError)));
    }
}
