use clap::{Parser, Subcommand};
use std::path::PathBuf;
/// A fictional versioning CLI
#[derive(Debug, Parser)]
#[command(name = "merger")]
#[command(about = "A simple CLI to process images", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Connects two images
    #[command(arg_required_else_help = true)]
    Merge {
        /// First image path
        #[arg(required = true)]
        img1: PathBuf,

        /// Second image path
        #[arg(required = true)]
        img2: PathBuf,

        /// Output file
        #[arg(required = true)]
        output: PathBuf,
    },

    /// Merge everything in folder
    #[command(arg_required_else_help = true)]
    MergeFolder {
        /// Folder path
        #[arg(required = true)]
        path: PathBuf,

        /// Output folder
        #[arg(required = false, long, short)]
        output: PathBuf,
    },
}
