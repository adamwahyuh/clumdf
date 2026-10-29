use std::path::PathBuf;

use pdf_core::MergeReport;
use tauri::AppHandle;

use super::run_blocking;
use crate::error::CommandResult;
use crate::progress::EventReporter;

#[tauri::command]
pub async fn merge_pdfs(
    app: AppHandle,
    inputs: Vec<PathBuf>,
    output: PathBuf,
    job_id: String,
) -> CommandResult<MergeReport> {
    let reporter = EventReporter { app, job_id };
    run_blocking(move || pdf_core::merge_pdfs(&inputs, &output, &reporter)).await
}
