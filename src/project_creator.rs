use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    process,
};

use console::style;

use crate::{constants, utils::get_web_content_bytes};

fn create_file_from_web(url: &str, path: &str, name: &str) -> Result<(), Box<dyn Error>> {
    let file_bytes = get_web_content_bytes(url)?;
    let mut fio = File::create(format!("{}/{}", path, name))?;
    fio.write_all(&file_bytes)?;
    Ok(())
}

fn create_folders(r_path: &str, name: &str) -> Result<(), std::io::Error> {
    match fs::create_dir(format!("{}/{}", r_path, &name)) {
        Err(_) => {
            println!("Project {} exists", style(name).red().bold());
            process::exit(1)
        }
        _ => {}
    };
    fs::create_dir(format!("{}/{}/{}", r_path, &name, constants::TEMPLATE_DIR))?;
    fs::create_dir(format!("{}/{}/{}", r_path, &name, constants::STATIC_DIR))?;
    fs::create_dir(format!(
        "{}/{}/{}/{}",
        r_path,
        &name,
        constants::STATIC_DIR,
        constants::JAVASCRIPT_DIR
    ))?;
    fs::create_dir(format!(
        "{}/{}/{}/{}",
        r_path,
        &name,
        constants::STATIC_DIR,
        constants::CSS_DIR
    ))
}

pub fn create_new_project(r_path: &str, name: &str) -> Result<(), Box<dyn Error>> {
    create_folders(r_path, name)?;
    create_file_from_web(
        &format!("{}/{}/{}", constants::PROJECT_URL, constants::TEMPLATE_DIR, constants::ENTRY_TEMPLATE),
        &format!("{}/{}/{}", r_path, &name, constants::TEMPLATE_DIR),
        constants::ENTRY_TEMPLATE,
    )?;

    create_file_from_web(
        &format!("{}/{}/{}/{}", constants::PROJECT_URL, constants::STATIC_DIR, constants::JAVASCRIPT_DIR, constants::JAVASCRIPT_FILE),
        &format!(
            "{}/{}/{}/{}",
            r_path,
            &name,
            constants::STATIC_DIR,
            constants::JAVASCRIPT_DIR
        ),
        constants::JAVASCRIPT_FILE,
    )?;

    create_file_from_web(
        &format!("{}/{}/{}/{}", constants::PROJECT_URL, constants::STATIC_DIR, constants::CSS_DIR, constants::CSS_FILE),
        &format!(
            "{}/{}/{}/{}",
            r_path,
            &name,
            constants::STATIC_DIR,
            constants::CSS_DIR
        ),
        constants::CSS_FILE,
    )?;

    create_file_from_web(
        &format!("{}/{}/{}", constants::PROJECT_URL, constants::STATIC_DIR, constants::IMAGE_FILE),
        &format!("{}/{}/{}", r_path, &name, constants::STATIC_DIR),
        constants::IMAGE_FILE,
    )?;

    println!("Created project {}", style(name).green().bold());
    Ok(())
}
