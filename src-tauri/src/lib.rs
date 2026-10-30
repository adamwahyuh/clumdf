mod commands;
mod error;
mod opened_files;
mod progress;

use opened_files::OpenedFiles;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        // Harus plugin pertama: instance kedua (Windows/Linux) meneruskan file ke yang sudah jalan
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            opened_files::push(app, opened_files::paths_from_args(argv, cwd.as_ref()));
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(OpenedFiles::default())
        .setup(|app| {
            let cwd = std::env::current_dir().unwrap_or_default();
            opened_files::push(app.handle(), opened_files::paths_from_args(std::env::args(), &cwd));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::compress::compress_pdf,
            commands::files::expand_paths,
            commands::image_to_pdf::images_to_pdf,
            commands::inspect::inspect_pdf,
            commands::merge::merge_pdfs,
            commands::metadata::strip_metadata,
            commands::organize::edit_pages,
            commands::password::protect_pdf,
            commands::password::unlock_pdf,
            commands::render::pdf_to_images,
            commands::signature::import_signature,
            commands::signature::place_signature,
            commands::split::split_pdf,
            commands::stamp::stamp_text,
            opened_files::take_opened_files,
        ])
        .build(tauri::generate_context!())
        .expect("gagal menjalankan aplikasi Tauri");

    app.run(|_app, _event| {
        // macOS: file di-drop ke ikon Dock atau dibuka lewat "Buka dengan"
        #[cfg(target_os = "macos")]
        if let tauri::RunEvent::Opened { urls } = _event {
            let paths = urls.into_iter().filter_map(|url| url.to_file_path().ok()).collect();
            opened_files::push(_app, paths);
        }
    });
}
