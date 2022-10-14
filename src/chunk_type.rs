#![allow(unused)]
use std::{fmt::Display, io::Read, str::FromStr};

use crate::{Error, Result};

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    chunk: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk
    }

    fn check_valid(chunks: &[u8]) -> bool {
        chunks
            .iter()
            .filter(|i| (65..=90u8).contains(i) || (97..=122u8).contains(i))
            .count()
            == 4
    }

    /// Reference:
    ///     http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
    ///     3.3. Chunk naming conventions
    ///         Ancillary bit: bit 5 of first byte
    fn is_critical(&self) -> bool {
        // 0 (uppercase) = critical, 1 (lowercase) = ancillary.
        self.chunk[0] >> 5 & 1 == 0
    }

    /// Reference:
    ///     http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
    ///     3.3. Chunk naming conventions
    ///         Private bit: bit 5 of second byte
    fn is_public(&self) -> bool {
        // 0 (uppercase) = public, 1 (lowercase) = private.
        self.chunk[1] >> 5 & 1 == 0
    }

    /// Reference:
    ///     http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
    ///     3.3. Chunk naming conventions
    ///         Reserved bit: bit 5 of third byte
    fn is_reserved_bit_valid(&self) -> bool {
        // Must be 0 (uppercase) in files conforming to this version of PNG.
        self.chunk[2] >> 5 & 1 == 0
    }

    /// Reference:
    ///     http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
    ///     3.3. Chunk naming conventions
    ///         Safe-to-copy bit: bit 5 of fourth byte
    fn is_safe_to_copy(&self) -> bool {
        // 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy.
        self.chunk[3] >> 5 & 1 == 1
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && Self::check_valid(&self.chunk)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self> {
        if !Self::check_valid(&value) {
            return Err(Error::from("incorrect chunk type"));
        }
        Ok(ChunkType { chunk: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.as_bytes();
        let mut chunk = [0; 4];
        let size = s.read(&mut chunk)?;
        if size != 4 {
            return Err(Error::from("len not equal 4"));
        }
        ChunkType::try_from(chunk)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.chunk))
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
