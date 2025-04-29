use crate::template::programming::write_template;
use std::error::Error;
use std::path::PathBuf;

use askama::Template;
use log::debug;

// share timescript
pub const SHARE_TIMESCRIPT_NAME: &str = "ShareScript.DSCR";

#[derive(Template)]
#[template(path = "share_timescript.txt")]
struct ShareTimescript {
    script_name: String,
    share_path: String,
    load_path: String,
    port: i32,
    nodes: Vec<String>,
}

/// Writes the templated file
pub fn write_share_time_script(
    script_name: String,
    share_path: String,
    load_path: String,
    port: i32,
    nodes: Vec<String>,
    dest_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let share_timescript_master = ShareTimescript {
        script_name,
        share_path,
        load_path,
        port,
        nodes,
    };

    // Render the template
    let rendered = share_timescript_master.render()?;
    debug!("template rendered:\n{}", rendered);

    // Create the file path
    let file_path = dest_path.join(SHARE_TIMESCRIPT_NAME);
    debug!("destination file will be: {:?}", file_path);

    // Write the rendered template to a file
    write_template(file_path, rendered)?;

    Ok(())
}
