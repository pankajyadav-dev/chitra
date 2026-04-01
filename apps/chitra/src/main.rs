use anyhow::{Error, Result};
use clap::Parser;
use clap_lib::{Cli, config::Command};
use ctx_lib::chitra::{check_chitra_dir, init_chitra};
use ctx_lib::index::{index_files, index_relative_path};
use ctx_lib::validate_path;
use std::env;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    //   info!("chitra, here how can i help you today");
    // info!("{:?}", args);
    let root_dir_path = env::current_dir()?;
    match args.cmd {
        Command::Init { path } => {
            let dir_path = validate_path(path);
            info!("Dir path {:?}", dir_path);
            let ctx_dir_path = root_dir_path.join(dir_path);
            //      info!(
            //        "The current dir in which the programm is running {:?}",
            //      ctx_dir_path
            // );
            init_chitra(ctx_dir_path).await?;
            info!(".chitra is init completed successfully");
        }
        Command::Index { path } => {
            //info!("Index command trigger {:?}", path);
            let dir_path = root_dir_path.join(validate_path(path));
            //info!("The dir passed by the user {:?}", dir_path);
            let chitra_path = check_chitra_dir(&dir_path).await;
            if let Some(c) = chitra_path {
                info!("The chitra dir {:?}", c);
                let relative_path = index_relative_path(&c, &dir_path).await?;
                info!(
                    "The parsered relative path for indexing {:?}",
                    relative_path
                );
                let _ = index_files(&dir_path);
                info!("start the indexing of the {:?}", relative_path);
            } else {
                warn!("The .chitra not found please create it using ctx init");
                return Ok(());
            }
            //info!("THe chitra path check completed");
        }
    }
    Ok(())
}
