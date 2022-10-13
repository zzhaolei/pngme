#![allow(unused)]

use std::{fmt::Display, io::Read};

use crc::{Crc, CRC_32_ISO_HDLC};

use crate::{chunk_type::ChunkType, Error};

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Self::checksum(
            &chunk_type
                .bytes()
                .iter()
                .chain(data.iter())
                .copied()
                .collect::<Vec<u8>>(),
        );
        Chunk {
            chunk_type,
            data,
            crc,
        }
    }

    fn length(&self) -> usize {
        self.data.len()
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data_as_string(&self) -> anyhow::Result<String, Error> {
        Ok(String::from_utf8(self.data.as_slice().to_vec())?)
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn checksum(bytes: &[u8]) -> u32 {
        let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        crc.checksum(bytes)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    /// &[u8] 包含数据 [长度、chunk_type、数据、crc]
    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            return Err(Error::from("incorrect chunk data"));
        }

        // 将 length 从 value 中分割出来
        let mut length = [0; 4];
        let _ = value.read(&mut length)?;
        let length = u32::from_be_bytes(length) as usize;
        let crc = Chunk::checksum(&value[..value.len() - 4]);

        // 判断 value 的数据长度是否符合规范
        // chunk_type + data + crc
        if value.len() < length + 4 + 4 {
            return Err(Error::from("incorrect chunk data"));
        }

        // 将 chuank_type 从 value 中分割出来
        let mut chunk = [0; 4];
        let _ = value.read(&mut chunk)?;
        let chunk_type = ChunkType::try_from(chunk)?;

        // 将 data 从 value 中分割出来
        let data = value[..length].to_vec();
        value = &value[length..];

        // 将 crc 从 value 中分割出来
        let mut raw_crc = [0; 4];
        let _ = value.read(&mut raw_crc)?;
        let raw_crc = u32::from_be_bytes(raw_crc);
        if crc != raw_crc {
            return Err(Error::from("error"));
        }

        Ok(Chunk {
            chunk_type,
            data,
            crc: raw_crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = match self.data_as_string() {
            Ok(s) => s,
            Err(err) => "".to_owned(),
        };

        write!(f, "{}", data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

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
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

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
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
