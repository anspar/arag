use sha2::{Digest, Sha256};
use std::{env, fs};

fn cache_exists() -> Result<bool, std::io::Error> {
    let r_path = env::current_dir()?.display().to_string();
    Ok(fs::metadata(format!("{r_path}/{}", crate::constants::CACHE_DIR))?.is_dir())
}

fn create_cache_dir() -> Result<bool, std::io::Error> {
    match cache_exists() {
        Ok(v) => Ok(v),
        Err(e) => {
            println!(
                "'{}' {e} -> Creating directory for cache",
                crate::constants::CACHE_DIR
            );
            let r_path = env::current_dir()?.display().to_string();
            fs::create_dir(format!("{r_path}/{}", crate::constants::CACHE_DIR))?;
            Ok(true)
        }
    }
}

pub fn get_hash(text: &str) -> String {
    let mut h = Sha256::new();
    h.update(text.as_bytes());
    format!("{:#02x}", h.finalize())
}

pub fn cache_bytes(name: &str, bytes: Vec<u8>) -> Result<String, std::io::Error> {
    create_cache_dir()?;
    let hash = get_hash(name);
    let r_path = env::current_dir()?.display().to_string();
    fs::write(
        format!("{r_path}/{}/{hash}", crate::constants::CACHE_DIR),
        bytes,
    )?;
    Ok(hash)
}

// pub async fn cache_from_web(url: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let r_path = env::current_dir()?.display().to_string();
//     let hash = get_hash(url);
//     let store = match fs::metadata(format!("{r_path}/{}/{hash}", crate::constants::CACHE_DIR,)) {
//         Ok(v) => !v.is_file(),
//         Err(_) => true,
//     };
//     if store {
//         let bytes = super::get_web_content_bytes(url).await?;
//         return Ok(cache_bytes(url, bytes)?);
//     }
//     println!("Skipping {} Cache Exists", style(url).blue().bold());
//     Ok(hash)
// }

pub fn cache_from_web_blocking(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let r_path = env::current_dir()?.display().to_string();
    let hash = get_hash(url);
    let store = match fs::metadata(format!("{r_path}/{}/{hash}", crate::constants::CACHE_DIR,)) {
        Ok(v) => !v.is_file(),
        Err(_) => true,
    };
    if store {
        let bytes = super::get_web_content_bytes_blocking(url)?;
        if bytes.len() == 0 {
            return Err(Box::new(crate::types::CustomError::Any(
                "Empty Content".to_owned(),
            )));
        }
        return Ok(cache_bytes(url, bytes)?);
    }
    // println!("Skipping {} Cache Exists", style(url).blue().bold());
    Ok(hash)
}
