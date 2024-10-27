use clap::Parser;

/// Show and manage Bible reading
#[derive(Debug, Parser)]
pub struct Cli {
    /// The path for the save file
    pub path: std::path::PathBuf,
}
