use clap_lib::Cli;
use clap::Parser;
use anyhow::{Result,Error};
use tracing::info;

fn main() -> Result<(),Error> {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    info!("chitra, here how can i help you today");
    info!("{:?}  {:?}", args.cmd, args.path);
    Ok(())
}
