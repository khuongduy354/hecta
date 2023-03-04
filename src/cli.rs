use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // ///file name
    // #[arg(short, long)]
    // pub file: Option<String>,
    ///path to file
    #[arg(short, long)]
    pub path_to_file: Option<PathBuf>,
}

pub struct CliApp {
    pub file_path: Option<PathBuf>,
}

impl CliApp {
    pub fn new() -> Self {
        let args = Args::parse();
        CliApp {
            file_path: args.path_to_file,
        }
    }
}
