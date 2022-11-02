use crate::chunk_type::ChunkType;
use crc::CRC_32_ISO_HDLC;

/// A chunk of a PNG file.
/// Each chunk consits of four parts: length, chunk type, chunk data, and CRC
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
}
