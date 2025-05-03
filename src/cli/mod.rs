use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::PathBuf;

/// Generate scripts for the Laseranimation Sollinger Lasergraph DSP
///
/// Use the subcommands to generate the desired scripts
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

// Define all subcommands below
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate shell completions
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Generate the Autoboot script
    ///
    /// This subcommand generates the Autoboot script that can be placed into every Lasergraph DSP.
    Autoboot {
        /// Set the local destination path of the script
        #[clap(short, long)]
        dest_path: PathBuf,
    },

    /// Generate the programming scripts
    ///
    /// This subcommand generates the programming scripts for the Lasergraph DSP.
    Programming {
        /// Set the path to the share folder
        #[clap(short, long)]
        share_path: String,

        /// Set the path to the load folder
        #[clap(short, long)]
        load_path: String,

        /// Set the local destination path of the scripts
        #[clap(short, long)]
        dest_path: PathBuf,

        /// TCP port of the Lasergraph DSP nodes
        #[arg(short, long, default_value_t = 8210)]
        port: i32,

        /// IP addresses of all Lasergraph DSP nodes
        #[arg(short, long)]
        nodes: Vec<String>,

        /// IP address of the Lasergraph DSP master
        #[arg(short, long)]
        master: String,
    },
}
