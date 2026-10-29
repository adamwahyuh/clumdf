use std::path::PathBuf;

use pdf_core::{CompressOptions, CompressReport};
use tauri::AppHandle;

use super::run_blocking;
use crate::error::CommandResult;
use crate::progress::EventReporter;

#[tauri::command]
pub async fn compress_pdf(
    app: AppHandle,
    input: PathBuf,
    output: PathBuf,
    options: CompressOptions,
    job_id: String,
) -> CommandResult<CompressReport> {
    let reporter = EventReporter { app, job_id };
    run_blocking(move || pdf_core::compress_pdf(&input, &output, &options, &reporter)).await
}
