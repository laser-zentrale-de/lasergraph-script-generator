use crate::template::programming::write_template;
use std::error::Error;
use std::path::PathBuf;

use askama::Template;
use log::debug;

// load timescript
pub const LOAD_TIMESCRIPT_NAME: &str = "LoadScript.DSCR";

#[derive(Template)]
#[template(path = "load_timescript.txt")]
struct LoadTimescript {
    script_name: String,
    share_path: String,
}

pub fn write_load_time_script(
    script_name: String,
    share_path: String,
    dest_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let load_timescript = LoadTimescript {
        script_name,
        share_path,
    };

    // Render the template
    let rendered = load_timescript.render()?;
    debug!("template rendered:\n{}", rendered);

    // Create the file path
    let file_path = dest_path.join(LOAD_TIMESCRIPT_NAME);
    debug!("destination file will be: {:?}", file_path);

    // Write the rendered template to a file
    write_template(file_path, rendered)?;

    Ok(())
}
