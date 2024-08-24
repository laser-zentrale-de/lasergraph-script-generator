use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use askama::Template;
use log::debug;

/// Write all programming scripts
pub fn write_programming_scripts(
    script_name: &str,
    share_path: &str,
    load_path: &str,
    port: i32,
    nodes: &Vec<String>,
    dest_path: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    debug!("Start programming scripts");

    // share timescript
    debug!("Write share timescript {}", SHARE_TIMESCRIPT_NAME);
    write_share_time_script(
        script_name.to_string(),
        share_path.to_string(),
        load_path.to_string(),
        port,
        nodes.to_vec(),
        dest_path.to_path_buf(),
    )?;

    // load timescript
    debug!("Write load timescript {}", LOAD_TIMESCRIPT_NAME);
    write_load_time_script(
        script_name.to_string(),
        share_path.to_string(),
        dest_path.to_path_buf(),
    )?;

    // share film
    debug!("Write share film {}", SHARE_FILM_NAME);
    write_share_film(
        script_name.to_string(),
        share_path.to_string(),
        load_path.to_string(),
        port,
        nodes.to_vec(),
        dest_path.to_path_buf(),
    )?;

    // load film
    debug!("Write load film {}", LOAD_FILM_NAME);
    write_load_film(
        script_name.to_string(),
        share_path.to_string(),
        dest_path.to_path_buf(),
    )?;

    Ok(())
}

// load timescript
const LOAD_TIMESCRIPT_NAME: &str = "LoadScript.DSCR";

#[derive(Template)]
#[template(path = "load_timescript.txt")]
struct LoadTimescript {
    script_name: String,
    share_path: String,
}

fn write_load_time_script(
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

// share timescript
const SHARE_TIMESCRIPT_NAME: &str = "ShareScript.DSCR";

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
fn write_share_time_script(
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

// share film
const SHARE_FILM_NAME: &str = "ShareFilm.DSCR";

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
fn write_share_film(
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

// load film
const LOAD_FILM_NAME: &str = "LoadFilm.DSCR";

#[derive(Template)]
#[template(path = "load_film.txt")]
struct LoadFilm {
    film_name: String,
    share_path: String,
}

fn write_load_film(
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

/// Writes a templated string to a file
fn write_template(file_path: PathBuf, content: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
