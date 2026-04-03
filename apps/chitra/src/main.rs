use anyhow::{Error, Result};
use clap::Parser;
use clap_lib::{Cli, config::Command};
use ctx_lib::chitra::{check_chitra_dir, init_chitra};
use ctx_lib::index::filter_index_files;
use ctx_lib::index_tree::create_index_tree;
use ctx_lib::validate_path;
use std::env;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    info!("chitra, here how can i help you today");
    info!("{:?}", args);
    let root_dir_path = env::current_dir()?;
    match args.cmd {
        Command::Init { path } => {
            let dir_path = validate_path(path);
            info!("Dir path {:?}", dir_path);
            let ctx_dir_path = root_dir_path.join(dir_path);
            init_chitra(ctx_dir_path).await?;
            info!(".chitra is init completed successfully");
        }
        Command::Index { path } => {
            let dir_path = root_dir_path.join(validate_path(path));
            // info!("the directory path {:?}", &dir_path);
            let chitra_path = check_chitra_dir(&dir_path).await;
            // info!("the chitra config fiel path {:?}", &chitra_path);
            if let Some(c) = chitra_path {
                // info!("The chitra dir {:?}", c);
                // let relative_path = index_relative_path(&c, &dir_path).await?;
                // info!(
                //     "The parsered relative path for indexing {:?}",
                //     relative_path
                // );
                // info!("start the indexing of the {:?}", relative_path);
                let filter_paths = filter_index_files(&c, &dir_path).await?;
                // info!("the filter paths to index {:?}", filter_paths);
                create_index_tree(&c, filter_paths).await?;
                // info!("Process completed");
            } else {
                warn!("The .chitra not found please create it using ctx init");
                return Ok(());
            }
        }
    }
    Ok(())
}
