use std::sync::Arc;

use bible::Bible;
use clap::Parser;
use cli::Cli;
use read::launch_reading;

mod cli;

fn main() {
    let args = Cli::parse();

    let bible: Arc<Bible> = Arc::new(Bible::new());

    let client = launch_reading(2, args.path);
}
