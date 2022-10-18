use std::path::PathBuf;

use clap::{command, Parser, Subcommand};

const CT_HELP: &str = "块类型，由4位a-z、A-Z的字符组成，第三位需大写。例如: ruSt";

#[derive(Parser, Debug)]
#[command(name = "pngme")]
#[command(author = "zhaolei <im.zhaolei@foxmail.com>")]
#[command(version = "v1.0.0")]
#[command(about = "shadow message in png file", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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

impl Args {}
