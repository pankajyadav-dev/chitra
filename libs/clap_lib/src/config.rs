use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Command {
    Init { path: Option<PathBuf> },
    Index { path: Option<PathBuf> },
}
