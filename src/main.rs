mod cli;

use clap::Parser;
use cli::Commands;
use log::{error, info, trace};

fn main() {
    // Initialize the logger
    env_logger::init();

    info!("lasergraph-script-generator started");

    // Parse arguments from CLI
    let args = cli::Args::parse();

    match args.cmd {
        // Subcommand: Programming
        Commands::Programming {
            share_path,
            load_path,
            dest_path,
            port,
            nodes,
            master,
        } => {
            info!("Subcommand: programming");

            // Check if nodes has at least 1 entry
            if nodes.is_empty() {
                error!("Error: At least one Lasergraph DSP node must be provided.");
                std::process::exit(1);
            }

            // Trace print the parsed arguments
            trace!("share_path: {}", share_path);
            trace!("load_path: {}", load_path);
            trace!("dest_path: {:?}", dest_path);
            trace!("master: {}", master);
            trace!("nodes: {:?}", nodes);
            trace!("port: {}", port);
        }
    }
}
