use crate::enums::Mode;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "echo-server", about = "A TCP async echo server")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run {
        #[arg(value_enum)]
        mode: Mode,
    },
}
