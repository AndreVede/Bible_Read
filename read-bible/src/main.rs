use clap::Parser;
use cli::{Cli, Commands};
use cmd::next_previous::Direction;

mod cli;
mod cmd;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("The path selected is {}", cli.path.to_str().unwrap());

    match &cli.command {
        Commands::Show(args) => cmd::show::main(cli.path, args)?,
        Commands::Set(args) => cmd::set::main(cli.path, args)?,
        Commands::Next(args) => cmd::next_previous::main(cli.path, args, Direction::Next)?,
        Commands::Previous(args) => cmd::next_previous::main(cli.path, args, Direction::Previous)?,
    };

    Ok(())
}
