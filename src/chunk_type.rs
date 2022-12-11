use crate::errors::PngError;
use std::fmt::Display;
use std::str::FromStr;

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
        self.0[0].is_ascii_uppercase()
    }

    /// Checks if chunk is a public chunk by examining bit 5 of the second byte.
    pub fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
    }

    /// Checks for reserved bit in chunk by examining bit 5 of the third byte.
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase()
    }

    /// Checks the chunk's safe-to-copy bit by examining
    /// bit 5 of the fourth byte.
    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
    }

    /// Returns `true` if the reserved bit is valid and all four bytes are
    /// uppercase or lowercase ASCII letters
    pub fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() {
            return false;
        }
        !self.0.into_iter().any(|byte| !byte.is_ascii_alphabetic())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = PngError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().any(|byte| !byte.is_ascii_alphabetic()) {
            Err(Self::Error::InvalidByte)
        } else {
            Ok(Self(value))
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .into_iter()
            .try_for_each(|byte| write!(f, "{}", byte as char))
    }
}

impl FromStr for ChunkType {
    type Err = PngError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars()
            .any(|ch| !ch.is_ascii() || !ch.is_ascii_alphabetic())
        {
            Err(Self::Err::InvalidByte)
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

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("bLOb").unwrap();
        assert_eq!(chunk.to_string(), "bLOb");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let chunk_to_string = format!("{}", chunk_type_1);
        let are_chunks_equal = chunk_type_1 == chunk_type_2;

        assert_eq!(chunk_to_string, String::from("RuSt"));
        assert_eq!(are_chunks_equal, true);
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }
}
