use std::path::PathBuf;

use pdf_core::{Placement, SignReport, SignatureInfo};
use tauri::{AppHandle, Manager};

use super::run_blocking;
use crate::error::{CommandError, CommandResult};

/// Rapikan gambar tanda tangan dan simpan salinannya di folder data app,
/// supaya tetap ada walau file aslinya dipindah atau dihapus.
#[tauri::command]
pub async fn import_signature(app: AppHandle, source: PathBuf) -> CommandResult<SignatureInfo> {
    let dir = app.path().app_data_dir().map_err(CommandError::from)?;
    std::fs::create_dir_all(&dir).map_err(|e| pdf_core::PdfError::Io { path: dir.clone(), source: e })?;
    let dest = dir.join("signature.png");
    run_blocking(move || pdf_core::prepare_signature(&source, &dest)).await
}

#[tauri::command]
pub async fn place_signature(
    input: PathBuf,
    output: PathBuf,
    signature: PathBuf,
    placements: Vec<Placement>,
) -> CommandResult<SignReport> {
    run_blocking(move || pdf_core::place_signature(&input, &output, &signature, &placements)).await
}
