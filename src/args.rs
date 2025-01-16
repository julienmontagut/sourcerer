use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "srcman",
    version = "0.1.0",
    about = "A CLI tool to manage several repositories at once"
)]
pub struct Args {
    // #[arg(short, long)]
    // pub version: bool,
    #[arg(short, long, value_name = "CONFIG_PATH")]
    pub config: Option<PathBuf>,

    #[arg(short, long)]
    pub debug: bool,

    #[arg(short, long, value_name = "SAVE_DIR")]
    pub save: Option<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser)]
pub enum Command {
    Init,
    // Clone,
    // Pull,
    // Push,
    Status,
}
