use std::sync::{Arc, Mutex};

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::constants::{IPFS_GATEWAY, SERVER_PORT};
pub mod cli;

#[derive(Debug, Deserialize, Clone)]
pub struct AragConf {
    pub ipfs_gateway: Option<String>,
    pub dependencies: Vec<String>,
    pub port: Option<u64>,
}

impl AragConf {
    pub fn default() -> Self {
        Self {
            ipfs_gateway: Some(IPFS_GATEWAY.to_owned()),
            dependencies: vec![],
            port: Some(SERVER_PORT),
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
