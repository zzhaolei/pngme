use std::{
    io::{Read, Write},
    path::PathBuf,
};

use clap::{command, Parser, Subcommand};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png, Result};

const CT_HELP: &str = "块类型，由4位a-z、A-Z的字符组成，第三位需大写。例如: ruSt";

#[derive(Parser, Debug)]
#[command(name = "pngme")]
#[command(author = "zhaolei <im.zhaolei@foxmail.com>")]
#[command(version = "v1.0.0")]
#[command(about = "shadow message in png file", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Encode {
        path: PathBuf,
        #[arg(
            short,
            long,
            help = CT_HELP
        )]
        chunk_type: String,
        #[arg(short, long, help = "需要隐藏的信息")]
        message: String,
        #[arg(short = 'o', long, help = "输出文件")]
        output: Option<PathBuf>,
    },
    Decode {
        path: PathBuf,
        #[arg(
            short,
            long,
            help = CT_HELP
        )]
        chunk_type: String,
    },
    Remove {
        path: PathBuf,
        #[arg(
            short,
            long,
            help = CT_HELP
        )]
        chunk_type: String,
    },
    Print {
        path: PathBuf,
    },
}

impl Args {
    pub fn process(&self) -> Result<()> {
        if let Some(command) = &self.command {
            match command {
                Commands::Encode {
                    path,
                    chunk_type,
                    message,
                    output,
                } => self.encode(path, chunk_type, message, output)?,
                Commands::Decode { path, chunk_type } => self.decode(path, chunk_type)?,
                Commands::Remove { path, chunk_type } => self.remove(path, chunk_type)?,
                Commands::Print { path } => self.print(path)?,
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
        let png_data = self.png_from_file(path)?.as_bytes();
        println!("{:?}", png_data);
        Ok(())
    }
}
