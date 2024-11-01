use clap::CommandFactory;
use clap::{Parser, Subcommand};

use crate::cmd;

/// Show and manage Bible reading
#[derive(Debug, Parser)]
#[command(name = "Bible Read", author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    /// The path for the save file
    #[arg(short, long, default_value = "reading.ron")]
    pub path: std::path::PathBuf,
    /// Command
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Show the current status of reading
    Show(cmd::show::ShowArgs),
    /// Set a status of reading
    Set(cmd::set::SetArgs),
    /// Go to the next resource
    Next(cmd::next::NextArgs),
    /// Go to the previous resource
    Previous(cmd::previous::PreviousArgs),
}

pub fn create_doc_man() -> std::io::Result<()> {
    let out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);
    let cmd = Cli::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(out_dir.join("bible_read.1"), buffer)?;

    Ok(())
}
