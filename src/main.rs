mod cli;
mod template;

use clap::Parser;
use cli::Commands;
use log::{debug, error, info};

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
                error!("At least one Lasergraph DSP node must be provided.");
                std::process::exit(1);
            }

            // Trace print the parsed arguments
            debug!(
                "\nshare_path: {}\nload_path: {}\ndest_path: {:?}\nmaster: {}\nnodes: {:?}\nport: {}",
                share_path,
                load_path,
                dest_path,
                master,
                nodes,
                port
            );

            // Call the template function to write the ShareTimescript.DSCR file
            match template::write_programming_scripts(
                "show",
                &share_path,
                &load_path,
                port,
                &nodes,
                &dest_path,
            ) {
                Ok(()) => info!("Successfully wrote the programming scripts"),
                Err(e) => error!("Failed to write the programming scripts:\n{}", e),
            }
        }
    }
}
