//! File yang dibuka lewat OS: drop ke ikon app (Dock), "Buka dengan", atau
//! argumen baris perintah (Windows/Linux). Ditampung dulu karena bisa datang
//! sebelum UI siap; UI mengambilnya lewat `take_opened_files`.

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tauri::{AppHandle, Emitter, Manager, State};

#[derive(Default)]
pub struct OpenedFiles(Mutex<Vec<String>>);

/// Tampung file baru, beri tahu UI, dan munculkan jendela.
pub fn push(app: &AppHandle, paths: Vec<PathBuf>) {
    let paths: Vec<String> = paths.into_iter().filter(|p| p.exists()).map(|p| p.to_string_lossy().into_owned()).collect();
    if paths.is_empty() {
        return;
    }
    if let Ok(mut pending) = app.state::<OpenedFiles>().0.lock() {
        pending.extend(paths);
    }
    let _ = app.emit("files-opened", ());
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// Path dari argumen program (lewati nama program & flag), relatif terhadap `cwd`.
pub fn paths_from_args(args: impl IntoIterator<Item = String>, cwd: &Path) -> Vec<PathBuf> {
    args.into_iter().skip(1).filter(|a| !a.starts_with('-')).map(|a| cwd.join(a)).collect()
}

#[tauri::command]
pub fn take_opened_files(state: State<'_, OpenedFiles>) -> Vec<String> {
    state.0.lock().map(|mut pending| std::mem::take(&mut *pending)).unwrap_or_default()
}
