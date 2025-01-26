use crate::api::fetch_steam::fetch_steam_apps;
use std::cmp::Ordering;

pub fn search_steam_apps(search_text: &str, page: usize) -> (Vec<serde_json::Value>, usize) {
    let steam_apps = fetch_steam_apps().unwrap_or_default();
    let search_query = search_text.trim().to_lowercase();
    let results_per_page = 6;

    // Deteksi apakah input adalah angka (pencarian berdasarkan AppID)
    if let Ok(appid) = search_query.parse::<u32>() {
        if let Some((id, name)) = steam_apps.iter().find(|(id, _)| **id == appid) {
            return (
                vec![serde_json::json!({
                    "title": name,
                    "image": format!("https://shared.cloudflare.steamstatic.com/store_item_assets/steam/apps/{}/header.jpg", id),
                    "appid": id
                })],
                1, // Hanya satu halaman untuk pencarian berdasarkan AppID
            );
        }
    }

    // Filter berdasarkan nama aplikasi
    let mut matches: Vec<_> = steam_apps
        .iter()
        .filter(|(_, name)| name.to_lowercase().contains(&search_query))
        .collect();

    // Urutkan berdasarkan relevansi: prioritas ke hasil yang lebih akurat
    matches.sort_by(|(_, name_a), (_, name_b)| {
        let a_contains = name_a.to_lowercase().starts_with(&search_query);
        let b_contains = name_b.to_lowercase().starts_with(&search_query);

        match (a_contains, b_contains) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => name_a.len().cmp(&name_b.len()), // Prioritaskan nama yang lebih pendek
        }
    });

    let total_pages = (matches.len() + results_per_page - 1) / results_per_page;

    // Ambil hasil sesuai halaman (tanpa mengubah urutan saat pagination)
    let start = (page - 1) * results_per_page;
    let end = start + results_per_page;
    let paged_matches = &matches[start..matches.len().min(end)];

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
