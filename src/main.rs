use std::path::PathBuf;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use log::{ info, warn};

fn main() -> Result<()> {
    env_logger::init();
    #[derive(Parser)]
    struct Cli {
        /// The pattern to look for
        pattern: String,
        /// The path to the file to read
        path: PathBuf,
    }

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file: {}", &args.path.display()))?;
    // error!("Error");
    warn!("来测试了");

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    info!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    Ok(())
}


