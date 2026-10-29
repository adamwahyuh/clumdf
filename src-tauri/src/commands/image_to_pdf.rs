use std::path::PathBuf;

use pdf_core::{ImageToPdfOptions, ImageToPdfReport};
use tauri::AppHandle;

use super::run_blocking;
use crate::error::CommandResult;
use crate::progress::EventReporter;

#[tauri::command]
pub async fn images_to_pdf(
    app: AppHandle,
    inputs: Vec<PathBuf>,
    output: PathBuf,
    options: ImageToPdfOptions,
    job_id: String,
) -> CommandResult<ImageToPdfReport> {
    let reporter = EventReporter { app, job_id };
    run_blocking(move || pdf_core::images_to_pdf(&inputs, &output, &options, &reporter)).await
}
