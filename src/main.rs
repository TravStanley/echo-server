mod cli;
mod client;
mod enums;
mod server;

use std::error::Error;

use clap::Parser;
use cli::{Cli, Commands};
use enums::Mode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { mode } => match mode {
            Mode::Server => {
                println!("Starting Server...");
                server::run_server().await?;
            }
            Mode::Client => {
                println!("Starting Client...");
                client::run_client().await?;
            }
        },
    }

    Ok(())
}
