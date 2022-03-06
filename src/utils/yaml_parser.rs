use crate::types::AragConf;
use std::{error, io::Read};

fn get_file_content(path: &String) -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open(path)?;
    let mut content: String = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

pub fn get_conf(path: &String) -> Result<AragConf, Box<dyn error::Error>> {
    let content = get_file_content(path)?;
    let conf = serde_yaml::from_str::<AragConf>(&content)?;
    Ok(conf)
    // println!("{:?}", deserialized_point)
}
