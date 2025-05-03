mod load_film;
mod load_time_script;
mod share_film;
mod share_time_script;

use crate::programming;
use std::error::Error;
use std::path::Path;

use log::debug;

/// Write all programming scripts
pub fn write_programming_scripts(
    script_name: &str,
    share_path: &str,
    load_path: &str,
    port: i32,
    nodes: &[String],
    dest_path: &Path,
) -> Result<(), Box<dyn Error>> {
    debug!("Start programming scripts");

    // share timescript
    debug!(
        "Write share timescript {}",
        programming::share_time_script::SHARE_TIMESCRIPT_NAME
    );
    programming::share_time_script::write_share_time_script(
        script_name.to_string(),
        share_path.to_string(),
        load_path.to_string(),
        port,
        nodes.to_vec(),
        dest_path.to_path_buf(),
    )?;

    // load timescript
    debug!(
        "Write load timescript {}",
        programming::load_time_script::LOAD_TIMESCRIPT_NAME
    );
    programming::load_time_script::write_load_time_script(
        script_name.to_string(),
        share_path.to_string(),
        dest_path.to_path_buf(),
    )?;

    // share film
    debug!(
        "Write share film {}",
        programming::share_film::SHARE_FILM_NAME
    );
    programming::share_film::write_share_film(
        script_name.to_string(),
        share_path.to_string(),
        load_path.to_string(),
        port,
        nodes.to_vec(),
        dest_path.to_path_buf(),
    )?;

    // load film
    debug!("Write load film {}", programming::load_film::LOAD_FILM_NAME);
    programming::load_film::write_load_film(
        script_name.to_string(),
        share_path.to_string(),
        dest_path.to_path_buf(),
    )?;

    Ok(())
}
