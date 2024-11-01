use std::sync::Arc;

use bible::Bible;
use clap::Args;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct NextArgs {}

pub fn main(bible: Arc<Bible>, path: std::path::PathBuf, args: &NextArgs) -> anyhow::Result<()> {
    Ok(())
}
