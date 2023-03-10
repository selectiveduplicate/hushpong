use std::io::{BufReader, Read};

use crate::{chunk::Chunk, errors::PngError};

/// A PNG file.
///
/// A PNG file consists of a PNG signature followed by a series of chunks.
/// The PNG signature always consists of the following eight bytes (in decimal):
/// `[137, 80, 78, 71, 13, 10, 26, 10]`
///
///
pub(crate) struct Png {
    signature: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    const PNG_FILE_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];
    const MIN_CHUNK_LENGTH: usize = 12;

    /// Creates a new PNG from some chunks.
    pub(crate) fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self {
            signature: Self::PNG_FILE_SIGNATURE,
            chunks,
        }
    }

    /// Returns a slice of chunks from the PNG.
    pub(crate) fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = PngError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // First take the signature.
        let signature: [u8; 8]= value[..8].try_into()?;
        // The chunks start from the 8th byte of `value` after
        // taking into account the signature.
        let mut starting_cursor = 8;
        let mut chunks = Vec::<Chunk>::new();
        let mut end_cursor: usize;

        // While the end cursor doesn't reach the length of the slice.
        while starting_cursor < value.len() {
            // Get the `length` field of the `Chunk`.
            let chunk_length_bytes: [u8; 4] =
                value[starting_cursor..starting_cursor + 4].try_into()?;
            let chunk_length = u32::from_be_bytes(chunk_length_bytes) as usize;

            // We can now get upto which byte the current chunk exists.
            end_cursor = Self::MIN_CHUNK_LENGTH + chunk_length + starting_cursor;

            // The chunk exists from `starting_cursor` upto `end_cursor-1`
            let chunk = Chunk::try_from(&value[starting_cursor..end_cursor])?;
            chunks.push(chunk);
            starting_cursor = end_cursor;
        }

        Ok(Self {
            signature,
            chunks,
        })
    }
}

#[cfg(test)]
mod pngtests {
    use std::io::Read;
    use std::str::FromStr;

    use super::*;
    use crate::errors::PngError;
    use crate::{chunk::Chunk, chunk_type::ChunkType};

    fn get_testing_chunks() -> Vec<Chunk> {
        let mut chunks = Vec::new();

        chunks.push(get_chunk_from_strings("RuSt", "I don't know what I'm doing").unwrap());
        chunks.push(get_chunk_from_strings("TeAr", "Yes I'm crying").unwrap());
        chunks.push(get_chunk_from_strings("RaGe", "Nooooooo").unwrap());
        chunks
    }

    fn get_chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk, PngError> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        let chunk_data = data.as_bytes().iter().copied().collect::<Vec<u8>>();
        let chunk = Chunk::new(chunk_type, chunk_data);

        Ok(chunk)
    }

    fn get_png_from_chunks() -> Png {
        let chunks = get_testing_chunks();
        Png::from_chunks(chunks)
    }

    #[test]
    fn test_chunks_of_png() {
        let png = get_png_from_chunks();
        assert_eq!(png.chunks().len(), 3);
    }

    #[test]
    fn test_from_chunks() {
        let chunks = get_testing_chunks();
        let png = Png::from_chunks(chunks);

        assert_eq!(png.chunks().len(), 3);
    }

    #[test]
    fn test_valid_png_from_bytes() {
        let chunk_bytes: Vec<u8> = get_testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::PNG_FILE_SIGNATURE
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());
        assert!(png.is_ok());
    }
}
