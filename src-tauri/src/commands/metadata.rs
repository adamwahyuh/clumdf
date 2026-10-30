use std::path::PathBuf;

use pdf_core::{EditReport, PdfMetadata, StripReport};

use super::run_blocking;
use crate::error::CommandResult;

#[tauri::command]
pub async fn strip_metadata(input: PathBuf, output: PathBuf) -> CommandResult<StripReport> {
    run_blocking(move || pdf_core::strip_metadata(&input, &output)).await
}

#[tauri::command]
pub async fn read_pdf_metadata(path: PathBuf) -> CommandResult<PdfMetadata> {
    run_blocking(move || pdf_core::read_metadata(&path)).await
}

#[tauri::command]
pub async fn write_pdf_metadata(input: PathBuf, output: PathBuf, metadata: PdfMetadata) -> CommandResult<EditReport> {
    run_blocking(move || pdf_core::write_metadata(&input, &output, &metadata)).await
}
