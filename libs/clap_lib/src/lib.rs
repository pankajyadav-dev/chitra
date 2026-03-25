use clap::Parser;



#[derive(Parser)]
#[command(name = "chitra")] 
#[command(bin_name = "ctx")] 
#[command(about = "Context Optimization CLI")]
pub struct Cli {
    pub cmd: String,
    pub path: std::path::PathBuf,
}