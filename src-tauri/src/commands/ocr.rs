use std::path::PathBuf;

use pdf_core::{OcrReport, OcrTools};
use tauri::{AppHandle, Manager};

use super::run_blocking;
use crate::error::{CommandError, CommandResult};
use crate::progress::EventReporter;

/// Lokasi tesseract di sistem, atau null bila belum terpasang.
#[tauri::command]
pub fn ocr_status() -> Option<String> {
    pdf_core::ocr::tesseract::find().map(|p| p.to_string_lossy().into_owned())
}

#[tauri::command]
pub async fn ocr_pdf(app: AppHandle, input: PathBuf, output: PathBuf, job_id: String) -> CommandResult<OcrReport> {
    let resources = app.path().resource_dir().map_err(CommandError::from)?;
    let tesseract = pdf_core::ocr::tesseract::find()
        .ok_or_else(|| pdf_core::PdfError::OcrMissing { reason: "tesseract belum terpasang".into() })?;
    let reporter = EventReporter { app, job_id };
    run_blocking(move || {
        let tools = OcrTools {
            pdfium_dir: &resources.join("pdfium"),
            tesseract: &tesseract,
            // Data bahasa ind+eng dibundel (scripts/fetch-tessdata.sh)
            tessdata_dir: &resources.join("tessdata"),
        };
        pdf_core::ocr_pdf(&tools, &input, &output, &reporter)
    })
    .await
}
