pub mod programming;

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Writes a templated string to a file
pub fn write_template(file_path: PathBuf, content: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
