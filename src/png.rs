use crate::{
    chunk::Chunk,
    errors::{self, Expectations, PngError},
};

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

    /// Returns the PNG file signature as a slice of eight bytes.
    pub(crate) fn signature(&self) -> &[u8; 8] {
        &self.signature
    }

    /// Searches for a chunk inside the PNG.
    ///
    /// It looks for the string-type representation of the chunk's `ChunkType`.
    /// Returns a reference to the first `Chunk` it finds inside the PNG.
    pub(crate) fn search_chunk(&self, chunk_type: &str) -> Option<(usize, &Chunk)> {
        self.chunks()
            .iter()
            .enumerate()
            .find(|(_, chunk)| chunk.chunk_type().to_string().eq(chunk_type))
    }

    /// Appends a new chunk to the PNG.
    pub(crate) fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    /// Removes a chunk from the PNG.
    pub(crate) fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk, PngError> {
        let query = self.search_chunk(chunk_type);
        if query.is_none() {
            return Err(errors::PngError::ChunkNotFound);
        }
        let (index, _) = query.unwrap();
        let removed_chunk = self.chunks.remove(index);
        Ok(removed_chunk)
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = PngError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // First take the signature.
        let signature: [u8; 8] = value[..8].try_into()?;
        if !signature.eq(&Self::PNG_FILE_SIGNATURE) {
            return Err(Self::Error::InvalidPngSignature);
        }
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

        Ok(Self { signature, chunks })
    }
}

#[cfg(test)]
mod pngtests {
    use std::str::FromStr;

    use super::*;
    use crate::errors::PngError;
    use crate::{chunk::Chunk, chunk_type::ChunkType};

    fn get_testing_chunks() -> Vec<Chunk> {
        let chunks = vec![
            get_chunk_from_strings("RuSt", "I don't know what I'm doing").unwrap(),
            get_chunk_from_strings("TeAr", "Yes I'm crying").unwrap(),
            get_chunk_from_strings("RaGe", "Nooooooo").unwrap(),
        ];

        chunks
    }

    fn get_chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk, PngError> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        let chunk_data = data.as_bytes().to_vec();
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

    #[test]
    fn test_invalid_signature() {
        let chunk_bytes: Vec<u8> = get_testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = [130, 90, 78, 71, 13, 10, 26, 10]
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());
        assert!(png.is_err());
        assert!(matches!(png, Err(PngError::InvalidPngSignature)));
    }

    #[test]
    fn test_get_header_from_png() {
        let png = Png::from_chunks(get_testing_chunks());
        assert!(png.signature().eq(&Png::PNG_FILE_SIGNATURE));
    }

    #[test]
    fn test_search_chunk_some() {
        let png = Png::from_chunks(get_testing_chunks());
        let (_, chunk) = png.search_chunk("TeAr").unwrap();
        assert_eq!(chunk.chunk_type().to_string(), "TeAr");
        assert_eq!(chunk.data_as_string().unwrap(), "Yes I'm crying");
    }

    #[test]
    fn test_search_chunk_none() {
        let png = Png::from_chunks(get_testing_chunks());
        let chunk = png.search_chunk("CuTe");
        assert!(chunk.is_none());
    }

    #[test]
    fn test_append_chunk_some() {
        let mut png = Png::from_chunks(get_testing_chunks());
        png.append_chunk(get_chunk_from_strings("CuTe", "You are cute!").unwrap());
        let query = png.search_chunk("CuTe");
        assert!(query.is_some());
        let (_, chunk) = query.unwrap();
        assert_eq!(chunk.chunk_type().to_string(), "CuTe");
        assert_eq!(chunk.data_as_string().unwrap(), "You are cute!");
    }

    #[test]
    fn test_remove_chunk_ok() {
        let mut png = Png::from_chunks(get_testing_chunks());
        png.append_chunk(get_chunk_from_strings("CuTe", "You are cute!").unwrap());
        let query = png.search_chunk("RuSt");
        assert!(query.is_some());

        // Remove
        let removed = png.remove_chunk("RuSt");
        assert!(removed.is_ok());
        assert_eq!(removed.as_ref().unwrap().chunk_type().to_string(), "RuSt");
        assert!(png
            .search_chunk(&removed.unwrap().chunk_type().to_string())
            .is_none());
    }

    #[test]
    fn test_remove_chunk_err() {
        let mut png = Png::from_chunks(get_testing_chunks());
        // Remove
        let removed = png.remove_chunk("CuTe");
        assert!(removed.is_err());
        assert!(matches!(removed, Err(PngError::ChunkNotFound)));
    }
}
