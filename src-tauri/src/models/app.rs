use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SteamAppList {
    pub applist: AppList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppList {
    pub apps: Vec<App>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub appid: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CachedData {
    pub timestamp: u64,
    pub data: HashMap<u32, String>,
}
    