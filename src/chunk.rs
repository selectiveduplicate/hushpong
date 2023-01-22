use std::io::{BufReader, Read};

use crate::{chunk_type::ChunkType, errors::Expectations};
use crc::CRC_32_ISO_HDLC;

use crate::errors::PngError;

/// A chunk of a PNG file.
/// Each chunk consits of four parts: length, chunk type, chunk data, and CRC
#[derive(Debug)]
pub(crate) struct Chunk {
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
    pub(crate) fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
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
    pub(crate) fn crc(&self) -> u32 {
        self.crc
    }

    /// Returns the data length of the chunk.
    pub(crate) fn length(&self) -> u32 {
        self.length
    }

    /// Returns a reference to the `ChunkType` of the `Chunk`.
    pub(crate) fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Returns the `Chunk`s data as a string
    pub(crate) fn data_as_string(&self) -> Result<String, PngError> {
        let stringified_data = String::from_utf8(self.chunk_data.clone())?;
        Ok(stringified_data)
    }

    /// Returns the `Chunk` as a vector of bytes.
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let chunk_bytes: Vec<u8> = self
            .length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type().bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect();
        chunk_bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = PngError;

    /// Tries to produces a valid `Chunk` from a slice of bytes.
    // The `value` contains the 4-byte data length type,
    // the 4-byte chunk type,
    // the unspecified amount of chunk data, and
    // the 4-byte CRC that's calculated based on the chunk
    // type and chunk data.
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // Create a reader to read from the byte slice
        let mut reader = BufReader::new(value);
        // A 4-byte buffer to write to from the reader
        let mut buffer: [u8; 4] = [0; 4];
        // Get the chunk's length field
        reader.read_exact(&mut buffer)?;
        let chunk_length = u32::from_be_bytes(buffer);

        // Get the chunk type
        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;

        // Get the chunk data
        let mut chunk_data = vec![0; usize::try_from(chunk_length)?];
        reader.read_exact(&mut chunk_data)?;

        // Get CRC
        reader.read_exact(&mut buffer)?;
        let received_crc = u32::from_be_bytes(buffer);

        let actual_crc = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC)
            .checksum(&[chunk_type.bytes().as_slice(), chunk_data.as_slice()].concat());

        (received_crc == actual_crc)
            .then_some(Self::new(chunk_type, chunk_data))
            .ok_or(Self::Error::InvalidCrc(Expectations {
                got: received_crc,
                expected: actual_crc,
            }))
    }
}

#[cfg(test)]
mod chunk_tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn chunk_test_input() -> Chunk {
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
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "My life is like an eternal night...".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 35);
        assert_eq!(chunk.crc(), 2591807180);
    }

    #[test]
    pub(crate) fn test_chunk_with_invalid_chunk_type() {
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
        assert!(matches!(chunk, Err(PngError::InvalidByte)));
    }

    #[test]
    fn test_chunk_type_of_chunk() {
        let chunk = chunk_test_input();
        let chunk_type = chunk.chunk_type().to_string();
        assert_eq!(chunk_type, String::from("RuSt"));
    }

    #[test]
    fn test_chunk_length() {
        let chunk = chunk_test_input();
        assert_eq!(chunk.length(), 35);
    }

    #[test]
    fn test_chunk_string() {
        let chunk = chunk_test_input();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("My life is like an eternal night...");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = chunk_test_input();
        assert_eq!(chunk.crc(), 2591807180);
    }

    #[test]
    pub(crate) fn test_chunk_with_invalid_crc() {
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

        let chunk = Chunk::try_from(chunk_data.as_ref());
        assert!(chunk.is_err());
        assert!(matches!(chunk, Err(PngError::InvalidCrc(_))));
    }

    #[test]
    pub(crate) fn test_chunk_as_bytes() {
        let chunk = chunk_test_input();
        let chunk_bytes_received = chunk.as_bytes();

        let data_length: u32 = 35;
        let crc: u32 = 2591807180;
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data: Vec<u8> = "My life is like an eternal night..."
            .as_bytes()
            .into_iter()
            .copied()
            .collect();

        let chunk_bytes_expected: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.bytes().iter())
            .chain(data.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        assert_eq!(chunk_bytes_expected, chunk_bytes_received);
    }
}
