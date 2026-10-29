use std::path::PathBuf;

use pdf_core::PasswordReport;

use super::run_blocking;
use crate::error::CommandResult;

#[tauri::command]
pub async fn protect_pdf(input: PathBuf, output: PathBuf, password: String) -> CommandResult<PasswordReport> {
    run_blocking(move || pdf_core::protect_pdf(&input, &output, &password)).await
}

#[tauri::command]
pub async fn unlock_pdf(input: PathBuf, output: PathBuf, password: String) -> CommandResult<PasswordReport> {
    run_blocking(move || pdf_core::unlock_pdf(&input, &output, &password)).await
}
