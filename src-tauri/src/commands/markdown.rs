use std::path::PathBuf;

use pdf_core::{MarkdownOptions, MarkdownReport};
use tauri::{AppHandle, Manager};

use super::run_blocking;
use crate::error::{CommandError, CommandResult};

/// Batas file teks yang dibuka di editor; lebih dari ini bukan dokumen ketikan biasa.
const MAX_TEXT_BYTES: u64 = 10 * 1024 * 1024;

#[tauri::command]
pub async fn markdown_to_pdf(
    markdown: String,
    base_dir: Option<PathBuf>,
    output: PathBuf,
    options: MarkdownOptions,
) -> CommandResult<MarkdownReport> {
    run_blocking(move || pdf_core::markdown_to_pdf(&markdown, base_dir.as_deref(), &output, &options)).await
}

/// PDF pratinjau di folder cache app; mengembalikan path-nya.
#[tauri::command]
pub async fn markdown_preview(
    app: AppHandle,
    markdown: String,
    base_dir: Option<PathBuf>,
    options: MarkdownOptions,
) -> CommandResult<String> {
    let dir = app.path().app_cache_dir().map_err(CommandError::from)?;
    let output = dir.join("markdown-preview.pdf");
    run_blocking(move || {
        std::fs::create_dir_all(&dir).map_err(|e| pdf_core::PdfError::Io { path: dir.clone(), source: e })?;
        pdf_core::markdown_to_pdf(&markdown, base_dir.as_deref(), &output, &options)?;
        Ok(output.to_string_lossy().into_owned())
    })
    .await
}

#[tauri::command]
pub async fn read_text_file(path: PathBuf) -> CommandResult<String> {
    run_blocking(move || {
        let io = |e| pdf_core::PdfError::Io { path: path.clone(), source: e };
        let size = std::fs::metadata(&path).map_err(io)?.len();
        if size > MAX_TEXT_BYTES {
            return Err(pdf_core::PdfError::Io { path: path.clone(), source: std::io::Error::other("file terlalu besar") });
        }
        std::fs::read_to_string(&path).map_err(io)
    })
    .await
}

#[tauri::command]
pub async fn write_text_file(path: PathBuf, content: String) -> CommandResult<()> {
    run_blocking(move || pdf_core::document::write_atomic(&path, content.as_bytes())).await
}
