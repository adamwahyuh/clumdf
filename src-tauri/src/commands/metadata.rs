use std::path::PathBuf;

use pdf_core::StripReport;

use super::run_blocking;
use crate::error::CommandResult;

#[tauri::command]
pub async fn strip_metadata(input: PathBuf, output: PathBuf) -> CommandResult<StripReport> {
    run_blocking(move || pdf_core::strip_metadata(&input, &output)).await
}
