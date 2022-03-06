use std::{env, error::Error, fs, io::Read};

use console::style;

fn get_file_content_text(file_path: &str) -> Result<String, std::io::Error> {
    let r_path = env::current_dir()?;
    fs::read_to_string(format!("{}/{}", r_path.display(), file_path))
}

fn get_file_content_bytes(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let r_path = env::current_dir()?;
    fs::read(format!("{}/{}", r_path.display(), file_path))
}

fn get_web_content_text(url: &str) -> Result<String, Box<dyn Error>> {
    let name = *(url
        .split("/")
        .collect::<Vec<&str>>()
        .last()
        .unwrap_or(&"Unknown"));
    println!("Downloading content for {}", style(name).bold());
    let mut c: String = "".to_owned();
    let _ = reqwest::blocking::get(url)?.read_to_string(&mut c)?;
    Ok(c)
}

pub fn get_web_content_bytes(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let name = url
        .split("/")
        .collect::<Vec<&str>>()
        .last()
        .unwrap_or(&"Unknown")
        .to_owned();
    println!("Downloading Bytes for {}", style(name).bold());
    let mut c = vec![];
    let _ = reqwest::blocking::get(url)?.read_to_end(&mut c)?;
    Ok(c)
}

fn get_file_content_base64(file_path: &str) -> Result<String, std::io::Error> {
    let path = format!("{}/{}", env::current_dir()?.display(), file_path);
    // println!("{} : {}", &path, &file_path);
    let img = fs::read(path)?;
    let b64 = base64::encode(img);
    Ok(b64)
}

pub mod cli;
pub mod helpers;
pub mod yaml_parser;
