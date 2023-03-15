use anyhow::bail;
use clap::Parser;
use stash_rs::list::list_all;
use thiserror::Error;

#[derive(Parser)]
#[command(name = "stash")]
#[command(about = "moves files or directories from/to trashcan")]
struct Cli {
    /// Files to act on
    #[arg(required_unless_present = "list", index = 1)]
    files: Vec<String>,

    /// List files currently in the trashcan(s)
    #[arg(short = 'l', long, group = "input")]
    list: bool,

    /// Restore/undelete files from the trashcan back to their original location
    #[arg(short = 'r', long, group = "input", requires = "files")]
    restore: bool,

    /// Copy files to the trashcan instead of moving them
    #[arg(short = 'c', long, requires = "files")]
    copy: bool,

    /// Output superfluous information about the operations being done
    #[arg(short = 'v', long)]
    verbose: bool,
}

#[derive(Debug, Error)]
pub enum CustomError {
    #[error("operating system not supported")]
    OperatingSystemNotSupported,
}

fn main() -> anyhow::Result<()> {
    if !cfg!(unix) {
        bail!(CustomError::OperatingSystemNotSupported);
    }

    let cli = Cli::parse();

    if cli.list {
        list_all()?
            .into_iter()
            .filter_map(|entry| entry.ok())
            .for_each(|entry| {
                match entry.file() {
                    Some(path) => if entry.missing_file() {
                        println!("missing: {}", path.to_str().unwrap_or("<error>"))
                    } else {
                        println!("found  : {}", path.to_str().unwrap_or("<error>"))
                    },
                    None => println!("none"),
                }
            })
    }

    Ok(())
}
