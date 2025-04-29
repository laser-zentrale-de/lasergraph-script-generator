use crate::template::programming::write_template;
use std::error::Error;
use std::path::PathBuf;

use askama::Template;
use log::debug;

// load film
pub const LOAD_FILM_NAME: &str = "LoadFilm.DSCR";

#[derive(Template)]
#[template(path = "load_film.txt")]
struct LoadFilm {
    film_name: String,
    share_path: String,
}

pub fn write_load_film(
    film_name: String,
    share_path: String,
    dest_path: PathBuf,
) -> Result<(), Box<dyn Error>> {
    let load_film = LoadFilm {
        film_name,
        share_path,
    };

    // Render the template
    let rendered = load_film.render()?;
    debug!("template rendered:\n{}", rendered);

    // Create the file path
    let file_path = dest_path.join(LOAD_FILM_NAME);
    debug!("destination file will be: {:?}", file_path);

    // Write the rendered template to a file
    write_template(file_path, rendered)?;

    Ok(())
}
