use anyhow::{Error, Result};
use clap::Parser;
use clap_lib::{Cli, config::Command};
use ctx_lib::chitra::init_chitra;
use std::env;
use std::path::PathBuf;
use tracing::info;

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
            let root_dir_path = env::current_dir()?;
            let ctx_dir_path = root_dir_path.join(dir_path);
            info!(
                "The current dir in which the programm is running {:?}",
                ctx_dir_path
            );
            init_chitra(ctx_dir_path).await?;
            info!(".chitra is init completed successfully");
        }
        Command::Index { path } => {
            info!("Index command trigger {:?}", path);
        }
    }
    Ok(())
}
