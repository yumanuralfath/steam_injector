use crate::api::fetch_steam::fetch_steam_apps;

pub fn search_steam_apps(search_text: &str, page: usize) -> (Vec<serde_json::Value>, usize) {
    let steam_apps = fetch_steam_apps().unwrap_or_default();
    let search_query = search_text.trim().to_lowercase();
    let results_per_page = 5; // Jumlah hasil per halaman

    // Filter dan cari berdasarkan input
    let matches: Vec<_> = steam_apps
        .iter()
        .filter(|(_, name)| name.to_lowercase().contains(&search_query))
        .collect();

    let total_pages = (matches.len() + results_per_page - 1) / results_per_page; // Hitung total halaman

    // Ambil hasil sesuai halaman
    let start = (page - 1) * results_per_page;
    let end = start + results_per_page;
    let paged_matches = &matches[start..matches.len().min(end)];

    // Format hasil sebagai JSON
    let result: Vec<serde_json::Value> = paged_matches
        .iter()
        .map(|(id, name)| {
            serde_json::json!({
                "title": name,
                "image": format!("https://shared.cloudflare.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg", id),
                "appid": id
            })
        })
        .collect();

    (result, total_pages)
}
