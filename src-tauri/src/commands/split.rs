use std::path::PathBuf;

use pdf_core::{SplitMode, SplitReport};
use tauri::AppHandle;

use super::run_blocking;
use crate::error::CommandResult;
use crate::progress::EventReporter;

#[tauri::command]
pub async fn split_pdf(
    app: AppHandle,
    input: PathBuf,
    output_dir: PathBuf,
    mode: SplitMode,
    job_id: String,
) -> CommandResult<SplitReport> {
    let reporter = EventReporter { app, job_id };
    run_blocking(move || pdf_core::split_pdf(&input, &output_dir, &mode, &reporter)).await
}
