mod cli;
mod template;

use self::template::{autoboot, programming};
use clap::{CommandFactory, Parser};
use log::{debug, error, info};

fn main() {
    // Initialize the logger
    env_logger::init();

    info!("lasergraph-script-generator started");

    // Parse arguments from CLI
    let args = cli::Args::parse();

    match args.cmd {
        // Handle shell completions
        Some(cli::Commands::Completions { shell }) => {
            info!("Subcommand: completions");
            let mut cmd = cli::Args::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
        }

        // Create the Autoboot script
        Some(cli::Commands::Autoboot { dest_path }) => {
            info!("Subcommand: autoboot");

            // Call the template function to write the autoboot script
            match autoboot::write_autoboot_script(dest_path) {
                Ok(()) => info!("Successfully wrote the autoboot script"),
                Err(e) => error!("Failed to write the autoboot script:\n{}", e),
            }
        }

        // Create the programming scripts
        Some(cli::Commands::Programming {
            share_path,
            load_path,
            dest_path,
            port,
            nodes,
            master,
        }) => {
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

            // Call the template function to write the programming scripts
            match programming::write_programming_scripts(
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

        // Show help if no subcommand is provided
        None => {
            info!("Subcommand not provided -> Fallback to help");
            cli::Args::command().print_help().unwrap();
            std::process::exit(0);
        }
    }
}
