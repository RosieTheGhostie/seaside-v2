use clap::{arg, command, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CmdArgs {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long)]
    /// An explicit path to a `Seaside.toml` file
    pub config: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Runs `text` as machine code. The other segments are optional.
    Run {
        text: PathBuf,
        data: Option<PathBuf>,
        ktext: Option<PathBuf>,
        kdata: Option<PathBuf>,
    },
    /// Runs experimental code.
    Experiment,
}