use crate::chunk::Chunk;

pub(crate) struct Png {
    signature: [u8; 8],
    chunks: Vec<Chunk>
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

mod pngtests {
    use std::str::FromStr;

    use super::*;
    use crate::{chunk::Chunk, chunk_type::{self, ChunkType}};
    use crate::errors::PngError;

    fn chunks() -> Vec<Chunk> {
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
        let chunks = chunks();
        Png::from_chunks(chunks)
    }

    #[test]
    fn test_chunks_of_png() {
        let png = get_png_from_chunks();
        assert_eq!(png.chunks().len(), 3);
    }
}
