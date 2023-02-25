use crate::chunk::Chunk;

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

#[cfg(test)]
mod pngtests {
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
}
