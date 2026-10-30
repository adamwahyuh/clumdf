use std::path::PathBuf;

use pdf_core::{RenderOptions, RenderReport};
use tauri::{AppHandle, Manager};

use super::run_blocking;
use crate::error::{CommandError, CommandResult};
use crate::progress::EventReporter;

#[tauri::command]
pub async fn pdf_to_images(
    app: AppHandle,
    input: PathBuf,
    output_dir: PathBuf,
    options: RenderOptions,
    job_id: String,
) -> CommandResult<RenderReport> {
    // libpdfium dibundel sebagai resource (lihat scripts/fetch-pdfium.sh & tauri.conf.json)
    let library_dir = app.path().resource_dir().map_err(CommandError::from)?.join("pdfium");
    let reporter = EventReporter { app, job_id };
    run_blocking(move || pdf_core::pdf_to_images(&library_dir, &input, &output_dir, &options, &reporter)).await
}
