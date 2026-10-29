use std::path::PathBuf;

use pdf_core::{StampOptions, StampReport};

use super::run_blocking;
use crate::error::CommandResult;

#[tauri::command]
pub async fn stamp_text(input: PathBuf, output: PathBuf, options: StampOptions) -> CommandResult<StampReport> {
    run_blocking(move || pdf_core::stamp_text(&input, &output, &options)).await
}
