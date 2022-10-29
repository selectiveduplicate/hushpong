use crate::errors::ChunkTypeErrors;
use std::str::FromStr;

use crate::errors;

#[derive(Debug, PartialEq, Eq)]
/// The 4-byte chunk type code of a PNG file.
pub struct ChunkType([u8; 4]);

impl ChunkType {
    /// Yields a slice of bytes from a chunk type code.
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    /// Checks if the chunk is critical by examining bit 5 of the first byte.
    pub fn is_critical(&self) -> bool {
        // Get the first byte
        let bits = self.0[0];
        // Check if bit 5 is 0 or 1 
        // 0 (uppercase) = critical, 1 (lowercase) = ancillary
        bits & (1 << 5) == 0u8
    }

    /// Checks if chunk is a public chunk by examining bit 5 of the second byte.
    pub fn is_public(&self) -> bool {
        let bits = self.0[1];
        bits & (1 << 5) == 0u8
    }

    /// Checks for reserved bit in chunk by examining bit 5 of the third byte.
    pub fn is_reserved_bit_valid(&self) -> bool {
        let bits = self.0[2];
        bits & (1 << 5) == 0u8
    }

    /// Checks the chunk's safe-to-copy bit by examining 
    /// bit 5 of the fourth byte.
    pub fn is_safe_to_copy(&self) -> bool {
        let bits = self.0[3];
        (bits & (1 << 5)) > 0u8
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeErrors;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().any(|byte| !byte.is_ascii_alphabetic()) {
            Err(Self::Error::NonAlphabeticAscii)
        } else {
            Ok(Self(value))
        }
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().any(|ch| !ch.is_ascii_alphabetic()) {
            Err(Self::Err::NonAlphabeticAscii)
        } else {
            let byte: [u8; 4] = s.as_bytes().try_into()?;
            Self::try_from(byte)
        }
    }
}

#[cfg(test)]
mod chunktype_tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }
    
    #[test]
    pub fn test_chunk_type_from_bytes_containing_asciialphabetic() {
        let actual = ChunkType::try_from([82, 117, 36, 116]);
        assert!(actual.is_err());
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }
}
