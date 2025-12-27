use clap::Parser;

mod cli;
use cli::{Cli, Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init { minecraft } => {
            todo!("init command not implemented yet (minecraft = {})", minecraft);
        }

        Command::Search { query } => {
            todo!("search command not implemented yet (query = {})", query);
        }

        Command::Add { mod_name, version } => {
            todo!(
                "add command not implemented yet (mod = {}, version = {:?})",
                mod_name,
                version
            );
        }

        Command::Remove { mod_name } => {
            todo!("remove command not implemented yet (mod = {})", mod_name);
        }

        Command::List => {
            todo!("list command not implemented yet");
        }

        Command::Resolve => {
            todo!("resolve command not implemented yet");
        }
    }

}
