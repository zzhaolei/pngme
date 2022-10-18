use std::{
    io::{Read, Write},
    path::PathBuf,
};

use crate::{args, chunk::Chunk, chunk_type::ChunkType, png::Png, Result};

pub struct Commands;

impl Commands {
    pub fn process(&self, args: args::Args) -> Result<()> {
        if let Some(command) = &args.command {
            match command {
                args::Commands::Encode {
                    path,
                    chunk_type,
                    message,
                    output,
                } => self.encode(path, chunk_type, message, output)?,
                args::Commands::Decode { path, chunk_type } => self.decode(path, chunk_type)?,
                args::Commands::Remove { path, chunk_type } => self.remove(path, chunk_type)?,
                args::Commands::Print { path } => self.print(path)?,
                args::Commands::Check { path } => self.check(path)?,
            };
        }
        Ok(())
    }

    fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>> {
        let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf)?;
        Ok(buf)
    }

    fn write_file(&self, path: &PathBuf, content: &[u8]) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(content)?;
        file.flush()?;
        Ok(())
    }

    fn png_from_file(&self, path: &PathBuf) -> Result<Png> {
        let png_data = self.read_file(path)?;
        Png::try_from(png_data.as_slice())
    }

    fn encode<'a, 'b>(
        &self,
        mut path: &'a PathBuf,
        chunk_type: &String,
        message: &String,
        output: &'b Option<PathBuf>,
    ) -> Result<()>
    where
        'b: 'a,
    {
        let mut png = self.png_from_file(path)?;
        if png.chunk_by_type(chunk_type).is_some() {
            let _ = png.remove_chunk(chunk_type);
        }

        let bytes: [u8; 4] = chunk_type.as_bytes().try_into()?;
        let chunk_type = ChunkType::try_from(bytes)?;
        let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

        png.append_chunk(chunk);

        if let Some(p) = output {
            path = p;
        }
        self.write_file(path, &png.as_bytes())?;
        Ok(())
    }

    fn decode(&self, path: &PathBuf, chunk_type: &String) -> Result<()> {
        let png = self.png_from_file(path)?;
        if let Some(chunk) = png.chunk_by_type(chunk_type) {
            println!("{}", chunk.data_as_string()?);
        } else {
            println!("`{chunk_type}` message not exists")
        }
        Ok(())
    }

    fn remove(&self, path: &PathBuf, chunk_type: &String) -> Result<()> {
        let mut png = self.png_from_file(path)?;
        if png.remove_chunk(chunk_type).is_some() {
            self.write_file(path, &png.as_bytes())?;
            println!("`{chunk_type}` message removed");
        }
        Ok(())
    }

    fn print(&self, path: &PathBuf) -> Result<()> {
        let png = self.png_from_file(path)?;
        for chunk in png.chunks() {
            if let Ok(data) = chunk.data_as_string() {
                if !data.is_empty() {
                    println!("{}", chunk.chunk_type());
                }
            }
        }
        Ok(())
    }

    fn check(&self, path: &PathBuf) -> Result<()> {
        let png = self.png_from_file(path)?;
        for chunk in png.chunks() {
            if let Ok(data) = chunk.data_as_string() {
                if !data.is_empty() {
                    println!("include secret message");
                    return Ok(());
                }
            }
        }
        println!("exculde secret message");
        Ok(())
    }
}
