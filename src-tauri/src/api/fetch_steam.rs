use crate::models::app::CachedData;
use crate::models::app::SteamAppList;
use reqwest::blocking::get;
use std::{
    collections::HashMap,
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

const CACHE_FILE: &str = "steam_cache.json";
const CACHE_DURATION: u64 = 24 * 60 * 60;

pub fn fetch_steam_apps() -> Result<HashMap<u32, String>, Box<dyn std::error::Error>> {
    // Cek apakah file cache sudah ada
    if let Ok(cached_apps) = read_cache() {
        return Ok(cached_apps);
    }

    // Jika cache tidak ada atau kadaluarsa, ambil dari API
    let url = "https://api.steampowered.com/ISteamApps/GetAppList/v0002/";
    let response: SteamAppList = get(url)?.json()?;

    let apps_map: HashMap<u32, String> = response
        .applist
        .apps
        .into_iter()
        .map(|app| (app.appid, app.name.trim().to_string()))
        .collect();

    // Simpan hasil ke cache
    save_cache(&apps_map)?;
    Ok(apps_map)
}

// Fungsi untuk membaca cache dari file
fn read_cache() -> Result<HashMap<u32, String>, Box<dyn std::error::Error>> {
    let path = Path::new(CACHE_FILE);

    // Jika file tidak ada, return error
    if !path.exists() {
        return Err("Cache not found".into());
    }

    let cache_content = fs::read_to_string(CACHE_FILE)?;
    let cached_data: CachedData = serde_json::from_str(&cache_content)?;

    // Cek apakah cache masih valid (kurang dari 24 jam)
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    if now - cached_data.timestamp > CACHE_DURATION {
        return Err("Cache expired".into()); // Cache kadaluarsa
    }

    Ok(cached_data.data)
}

// Fungsi untuk menyimpan cache ke file
fn save_cache(data: &HashMap<u32, String>) -> Result<(), Box<dyn std::error::Error>> {
    let cached_data = CachedData {
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        data: data.clone(),
    };

    let json_data = serde_json::to_string(&cached_data)?;
    println!("Menyimpan cache di steam_cache.json...");

    if let Err(e) = fs::write("steam_cache.json", json_data) {
        println!("Gagal menulis file cache: {}", e);
    } else {
        println!("Cache berhasil disimpan!");
    }
    Ok(())
}
