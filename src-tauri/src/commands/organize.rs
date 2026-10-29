use std::path::PathBuf;

use pdf_core::{PageEdit, PagesReport};

use super::run_blocking;
use crate::error::CommandResult;

#[tauri::command]
pub async fn edit_pages(input: PathBuf, output: PathBuf, pages: Vec<PageEdit>) -> CommandResult<PagesReport> {
    run_blocking(move || pdf_core::edit_pages(&input, &output, &pages)).await
}
