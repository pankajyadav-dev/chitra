use clap_lib::{Cli, config::Command};
use clap::Parser;
use anyhow::{Result,Error};
use tracing::info;
use treesitter_lib::get_ctx_parser_dir;

#[tokio::main]
async fn main() -> Result<(),Error> {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    info!("chitra, here how can i help you today");
    info!("{:?}", args);
    match args.cmd {
        Command::Init { path } => {
            info!("Init command trigger {:?}", path);
                let ctx_parser_dir = get_ctx_parser_dir().await?;
                info!("Ctx parser dir: {:?}", ctx_parser_dir);
            if let Some(path) = path {
                info!("Ctx path: {:?}", path);
            }
        },
        Command::Index { path } => {
            info!("Index command trigger {:?}", path);
        }
    }
    Ok(())
}
