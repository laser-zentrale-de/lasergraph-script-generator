use crate::template::write_template;
use std::error::Error;
use std::path::PathBuf;

use askama::Template;
use log::debug;

// share film
pub const SHARE_FILM_NAME: &str = "ShareFilm.DSCR";

#[derive(Template)]
#[template(path = "share_film.txt")]
struct ShareFilm {
    film_name: String,
    share_path: String,
    load_path: String,
    port: i32,
    nodes: Vec<String>,
}

/// Writes the templated file
pub fn write_share_film(
    film_name: String,
    share_path: String,
    load_path: String,
    port: i32,
    nodes: Vec<String>,
    dest_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let share_film = ShareFilm {
        film_name,
        share_path,
        load_path,
        port,
        nodes,
    };

    // Render the template
    let rendered = share_film.render()?;
    debug!("template rendered:\n{}", rendered);

    // Create the file path
    let file_path = dest_path.join(SHARE_FILM_NAME);
    debug!("destination file will be: {:?}", file_path);

    // Write the rendered template to a file
    write_template(file_path, rendered)?;

    Ok(())
}
