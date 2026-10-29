mod commands;
mod error;
mod progress;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::inspect::inspect_pdf,
            commands::merge::merge_pdfs,
        ])
        .run(tauri::generate_context!())
        .expect("gagal menjalankan aplikasi Tauri");
}
