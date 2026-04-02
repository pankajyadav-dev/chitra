use anyhow::{Error, Result};
use clap::Parser;
use clap_lib::{Cli, config::Command};
use ctx_lib::chitra::{check_chitra_dir, init_chitra};
use ctx_lib::index::{
    filter_index_files, index_files, index_relative_path, read_chitra_ignore_files,
};
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
                let files_to_index = index_files(&dir_path).unwrap_or_default();
                let files_to_ignore_from_indexing = read_chitra_ignore_files(&c).await?;
                info!("start the indexing of the {:?}", relative_path);
                info!(
                    "the file to be ignored from indexing {:?}",
                    files_to_ignore_from_indexing
                );
                let filter_paths = filter_index_files(c, files_to_index).await?;
                info!("the filter paths to index {:?}", filter_paths);
            } else {
                warn!("The .chitra not found please create it using ctx init");
                return Ok(());
            }
            //info!("THe chitra path check completed");
        }
    }
    Ok(())
}
