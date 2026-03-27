use std::path::PathBuf;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Command {
    Init {
        path: Option<PathBuf>,
    },
    Index {
        path: Option<PathBuf>,
    }
}
