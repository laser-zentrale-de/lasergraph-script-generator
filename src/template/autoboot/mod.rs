use crate::template::write_template;
use std::error::Error;
use std::path::PathBuf;

use askama::Template;
use log::debug;

const AUTOBOOT_NAME: &str = "Autoboot.DSCR";

#[derive(Template)]
#[template(path = "autoboot.txt")]
struct Autoboot {}

/// Write the autoboot script
pub fn write_autoboot_script(dest_path: PathBuf) -> Result<(), Box<dyn Error>> {
    debug!("Write the autoboot script");
    let autoboot = Autoboot {};

    // Render the template
    let rendered = autoboot.render()?;
    debug!("template rendered:\n{}", rendered);

    // Create the file path
    let file_path = dest_path.join(AUTOBOOT_NAME);
    debug!("destination file will be: {:?}", file_path);

    // Write the rendered template to a file
    write_template(file_path, rendered)?;

    Ok(())
}
