use std::sync::{Arc, Mutex};

use handlebars::Handlebars;
use serde::Deserialize;

use crate::constants;

#[derive(Debug, Deserialize, Clone)]
pub struct AragConf {
    pub ipfs_gateway: String,
    pub dependencies: Vec<String>,
    // pub static_dir: String,
    // pub html_dir: String,
    // pub js_dir: String,
    // pub css_dir: String,
    // pub entry_html: String,
    // pub entry_js: String,
    // pub entry_css: String
}

impl AragConf {
    pub fn defaults() -> Self {
        Self {
            ipfs_gateway: constants::IPFS_GATEWAY.to_owned(),
            dependencies: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct AragState<'a> {
    pub root_dir_path: String,
    pub conf: AragConf,
    pub entry_dir: String,
    pub hb: Arc<Mutex<Handlebars<'a>>>,
    pub files_updated: Arc<Mutex<bool>>,
}
