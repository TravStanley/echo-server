mod cli;
mod enums;

use clap::Parser;
use cli::{Cli, Commands};
use enums::Mode;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { mode } => match mode {
            Mode::Server => {
                println!("Starting Server...")
            }
            Mode::Client => {
                println!("Starting Client...")
            }
        },
    }
}
