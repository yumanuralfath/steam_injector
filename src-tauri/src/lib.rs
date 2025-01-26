mod api;
mod models;
mod services;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn search_steam_apps(search_text: String, current_page: u32) -> Result<(String, u32), String> {
    match current_page.try_into() {
        Ok(page) => {
            let (results, total_pages) =
                services::steam_service::search_steam_apps(&search_text, page);

            // Serialize the Vec<serde_json::Value> into a JSON String
            match serde_json::to_string(&results) {
                Ok(json_results) => Ok((json_results, total_pages.try_into().unwrap_or(0))),
                Err(_) => Err("Failed to serialize results".to_string()),
            }
        }
        Err(_) => Err("Invalid page number".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search_steam_apps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
