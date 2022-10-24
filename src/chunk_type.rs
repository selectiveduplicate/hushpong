#[derive(PartialEq, Eq)]
pub struct ChunkType(pub [u8; 4]);

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.0
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error =  &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if !value.is_ascii() {
            Err("chunk type code must be uppercase and lowercase ASCII letters")
        } else {
            Ok(Self(value))
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
}
