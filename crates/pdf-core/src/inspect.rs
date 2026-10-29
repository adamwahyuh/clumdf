use std::path::Path;

use serde::Serialize;

use crate::document;
use crate::error::PdfError;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfInfo {
    pub page_count: usize,
    pub file_size: u64,
}

/// Info ringkas untuk kartu file. Error-nya (terkunci/rusak) dipakai UI
/// untuk menandai file sebelum diproses.
pub fn inspect(path: &Path) -> Result<PdfInfo, PdfError> {
    let file_size = std::fs::metadata(path).map_err(|e| PdfError::io(path, e))?.len();
    let doc = document::open(path)?;
    Ok(PdfInfo { page_count: doc.get_pages().len(), file_size })
}
