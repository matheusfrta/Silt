use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "silt", about = "The framework of frameworks")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    New { name: String },
    Build,
}

fn main() {
    let cli = Cli::parse();
    match &cli.cmd {
        Cmd::New { name } => {
            fs::create_dir_all(name).unwrap();
            fs::write(format!("{}/package.json", name), r#"{"name":"app","dependencies":{"silt":"*"}}"#).unwrap();
            fs::write(format!("{}/index.ts", name), "import { Store } from 'silt';\n").unwrap();
            println!("created {}. run silt build.", name);
        }
        Cmd::Build => {
            println!("building silt graph targets...");
        }
    }
}