use clap::Args;
use read::{launch_reading, SaveServerError};

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct ShowArgs {}

pub fn main(path: std::path::PathBuf, args: &ShowArgs) -> anyhow::Result<()> {
    let client = launch_reading(1, path);

    match client.get_reading_from_file() {
        Ok(reading) => println!("{}", reading),
        Err(SaveServerError::FailedToGetSave) => {
            println!("There was a problem in getting save. Have you save your reading ?")
        }
        Err(_) => println!("AAAAHHHHH!"),
    }
    Ok(())
}
