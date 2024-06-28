use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version = "1.0", about = "Switch easily between different kubernetes clusters", long_about = None)]
pub struct Cli {
    /// Kubeconfig name you want to switch to
    pub(crate) kubeconfig: Option<String>,

    #[command(subcommand)]
    pub(crate) command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add new kubeconfig
    Add {
        kubeconfig_file: PathBuf,

        /// Name for a new kubeconfig
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Delete kubeconfig
    Delete {
        /// Name of the kubeconfig to be deleted
        kubeconfig: String
    }
}