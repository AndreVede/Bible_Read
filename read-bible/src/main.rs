use std::sync::Arc;

use bible::Bible;
use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod cmd;

fn main() -> anyhow::Result<()> {
    let bible: Arc<Bible> = Arc::new(Bible::new());
    let cli = Cli::parse();

    println!("The path selected is {}", cli.path.to_str().unwrap());

    match &cli.command {
        Commands::Show(args) => cmd::show::main(cli.path, args)?,
        Commands::Set(args) => cmd::set::main(bible.clone(), cli.path, args)?,
        Commands::Next(args) => cmd::next::main(bible.clone(), cli.path, args)?,
        Commands::Previous(args) => cmd::previous::main(bible.clone(), cli.path, args)?,
    };

    Ok(())
}
