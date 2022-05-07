use std::{env, error::Error, fs};

use console::style;

pub mod cache;

fn get_file_content_text(file_path: &str) -> Result<String, std::io::Error> {
    let r_path = env::current_dir()?;
    fs::read_to_string(format!("{}/{}", r_path.display(), file_path))
}

fn get_file_content_bytes(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let r_path = env::current_dir()?;
    fs::read(format!("{}/{}", r_path.display(), file_path))
}

fn get_file_content_base64(file_path: &str) -> Result<String, std::io::Error> {
    let path = format!("{}/{}", env::current_dir()?.display(), file_path);
    // println!("{} : {}", &path, &file_path);
    let img = fs::read(path)?;
    let b64 = base64::encode(img);
    Ok(b64)
}

// async fn get_web_content_text(url: &str) -> Result<String, Box<dyn Error>> {
//     let name = *(url
//         .split("/")
//         .collect::<Vec<&str>>()
//         .last()
//         .unwrap_or(&"Unknown"));
//     println!("Downloading {}", style(name).bold());
//     let c = reqwest::get(url).await?.text().await?;
//     Ok(c)
// }

pub async fn get_web_content_bytes(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let name = url
        .split("/")
        .collect::<Vec<&str>>()
        .last()
        .unwrap_or(&"Unknown")
        .to_owned();
    println!("Downloading {}", style(name).bold());
    // let mut c = vec![];
    let c = reqwest::get(url).await?.bytes().await?.to_vec();
    Ok(c)
}

fn get_cached_content_text(name: &str) -> Result<String, std::io::Error> {
    get_file_content_text(&format!(
        "{}/{}",
        crate::constants::CACHE_DIR,
        cache::get_hash(name)
    ))
}

fn get_cached_content_bytes(name: &str) -> Result<Vec<u8>, std::io::Error> {
    get_file_content_bytes(&format!(
        "{}/{}",
        crate::constants::CACHE_DIR,
        cache::get_hash(name)
    ))
}

pub mod cli;
pub mod helpers;
pub mod yaml_parser;
