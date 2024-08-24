use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use askama::Template;
use log::trace;

const SHARE_TIMESCRIPT_NAME: &str = "ShareTimescript.DSCR";

#[derive(Template)]
#[template(path = "share_timescript_master.txt")]
pub struct ShareTimescriptMaster {
    pub script_name: String,
    pub share_path: String,
    pub load_path: String,
    pub port: i32,
    pub nodes: Vec<String>,
}

/// Writes the templated ShareTimescript.DSCR file
pub fn write_share_time_script(
    script_name: String,
    share_path: String,
    load_path: String,
    port: i32,
    nodes: Vec<String>,
    dest_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let share_timescript_master = ShareTimescriptMaster {
        script_name,
        share_path,
        load_path,
        port,
        nodes,
    };

    // Render the template
    let rendered = share_timescript_master.render()?;
    trace!("template rendered: {}", rendered);

    // Create the file path
    let file_path = dest_path.join(SHARE_TIMESCRIPT_NAME);
    trace!("destination file will be: {:?}", file_path);

    // Write the rendered template to a file
    write_template(file_path, rendered)?;

    Ok(())
}

/// Writes a templated string to a file
fn write_template(file_path: PathBuf, content: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
