use std::path::PathBuf;

use pdf_core::PdfInfo;

use super::run_blocking;
use crate::error::CommandResult;

#[tauri::command]
pub async fn inspect_pdf(path: PathBuf) -> CommandResult<PdfInfo> {
    run_blocking(move || pdf_core::inspect(&path)).await
}
