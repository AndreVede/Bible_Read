use clap::Args;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct PreviousArgs {}

pub fn main(path: std::path::PathBuf, args: &PreviousArgs) -> anyhow::Result<()> {
    Ok(())
}
