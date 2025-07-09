// src/main.rs
/*
 * Main executable for NFTAiSuiteCoreUltra
 */

use clap::Parser;
use nftaisuitecoreultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "NFTAiSuiteCoreUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
