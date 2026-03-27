use anyhow::{Error, Result};
use clap::Parser;
use clap_lib::{Cli, config::Command};
use std::path::PathBuf;
//use tokio::fs;
use tracing::info;
use treesitter_lib::get_ctx_parser_dir;
use std::env;
use ctx_lib::chitra::check_chitra_dir;
//use anyhow::Context; 

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    info!("chitra, here how can i help you today");
    info!("{:?}", args);
    match args.cmd {
        Command::Init { path } => {
            let dir_path = match path {
                Some(p) => p,
                None => PathBuf::from("."),
            };
            info!("Dir path {:?}", dir_path);
            let root_ctx = check_chitra_dir(&dir_path).await;
            info!("the root_ctx path {:?}",Some(root_ctx));
            let ctx_dir_path = env::current_dir()?;
            info!("The current dir in which the programm is running {:?}",ctx_dir_path);
            let ctx_parser_dir = get_ctx_parser_dir().await?;
            info!("Ctx parser dir: {:?}", ctx_parser_dir);
        }
        Command::Index { path } => {
            info!("Index command trigger {:?}", path);
        }
    }
    Ok(())
}
