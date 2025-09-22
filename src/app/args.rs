use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub config_file: PathBuf,
}

pub fn parse() -> Args {
    Args::parse()
}
