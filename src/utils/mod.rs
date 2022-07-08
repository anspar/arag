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
    println!("Downloading {}", style(url).bold());
    let c = reqwest::get(url).await?.bytes().await?.to_vec();
    Ok(c)
}

struct HtmlHandler(Vec<u8>);

impl curl::easy::Handler for HtmlHandler {
    fn write(&mut self, data: &[u8]) -> Result<usize, curl::easy::WriteError> {
        self.0.extend_from_slice(data);

        Ok(data.len())
    }
}

pub fn get_web_content_bytes_blocking(url: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut dst = Vec::new();
    let mut easy = curl::easy::Easy::new();
    easy.url(url).unwrap();
    easy.get(true)?;
    easy.follow_location(true)?;
    easy.progress(true)?;
    let term = console::Term::stdout();
    term.write_line(&format!("Downloading {}", style(url).bold()))?;
    term.write_line(" ")?;
    easy.progress_function(move |td, d, _, _| {
        term.clear_last_lines(1).unwrap();
        term.write_line(&format!("{}/{} Bytes", style(d).green(), style(td).green()))
            .unwrap();
        true
    })?;

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|d| {
            dst.extend_from_slice(d);
            Ok(d.len())
        })?;
        transfer.perform()?;
    }
    Ok(dst)
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

pub mod helpers;
pub mod yaml_parser;
