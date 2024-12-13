use std::sync::Arc;

use bible::Bible;
use clap::Parser;
use cli::{Cli, Commands};

mod cli;
mod cmd;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("The path selected is {}", cli.path.to_str().unwrap());

    match &cli.command {
        Commands::Show(args) => cmd::show::main(cli.path, args)?,
        Commands::Set(args) => cmd::set::main(cli.path, args)?,
        Commands::Next(args) => cmd::next::main(cli.path, args)?,
        Commands::Previous(args) => cmd::previous::main(cli.path, args)?,
    };

    Ok(())
}
