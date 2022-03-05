use serde::Deserialize;

use crate::constants;

#[derive(Debug, Deserialize)]
pub struct AragConf{
    pub ipfs_gateway: String,
    // pub static_dir: String,
    // pub html_dir: String,
    // pub js_dir: String,
    // pub css_dir: String,
    // pub entry_html: String,
    // pub entry_js: String,
    // pub entry_css: String
}

impl AragConf{
    pub fn defaults()->Self{
        Self{
            ipfs_gateway: constants::IPFS_GATEWAY.to_owned()
        }
    }
}