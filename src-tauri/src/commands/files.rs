use std::path::PathBuf;

use pdf_core::files::ExpandedPaths;

use super::run_blocking;
use crate::error::CommandResult;

/// Buka folder & saring ekstensi; di thread terpisah karena folder jaringan bisa lambat.
#[tauri::command]
pub async fn expand_paths(paths: Vec<PathBuf>, extensions: Vec<String>) -> CommandResult<ExpandedPaths> {
    run_blocking(move || Ok(pdf_core::files::expand_paths(&paths, &extensions))).await
}
