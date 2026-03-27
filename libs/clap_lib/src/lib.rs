pub mod config;
use clap::Parser;
use config::Command;

#[derive(Parser,Debug)]
#[command(name = "chitra")] 
#[command(bin_name = "ctx")] 
#[command(about = "Context Optimization CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}