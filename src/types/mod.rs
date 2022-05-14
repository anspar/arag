use std::sync::{Arc, Mutex};

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
pub mod cli;

#[derive(Debug, Deserialize, Clone)]
pub struct AragConf {
    pub ipfs_gateway: Option<String>,
    pub dependencies: Vec<String>,
}

impl AragConf {
    pub fn default() -> Self {
        Self {
            ipfs_gateway: None,
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

#[derive(Debug, Clone, Serialize)]
pub struct Context {
    pub ipfs_gateway: String,
    pub release: bool,
}
