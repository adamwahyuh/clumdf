use std::collections::HashSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::document;
use crate::error::PdfError;
use crate::page_tree;

/// Satu halaman di dokumen hasil: nomor halaman asli + rotasi tambahan.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageEdit {
    /// Nomor halaman asli, mulai dari 1
    pub page: u32,
    /// Derajat searah jarum jam, kelipatan 90 (boleh negatif)
    #[serde(default)]
    pub rotate: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PagesReport {
    pub page_count: usize,
    pub output_size: u64,
}

/// Simpan PDF baru berisi halaman sesuai `edits`: urutan baru, rotasi, dan
/// halaman yang tidak disebut dibuang. Dipakai untuk susun ulang, putar,
/// hapus, maupun ekstrak halaman.
pub fn edit_pages(input: &Path, output: &Path, edits: &[PageEdit]) -> Result<PagesReport, PdfError> {
    if edits.is_empty() {
        return Err(PdfError::NoInput);
    }
    let doc = document::open(input)?;
    let pages = doc.get_pages();

    let mut used = HashSet::new();
    let mut selection = Vec::with_capacity(edits.len());
    for edit in edits {
        let id = *pages
            .get(&edit.page)
            .ok_or_else(|| PdfError::invalid_range(format!("halaman {} tidak ada", edit.page)))?;
        if edit.rotate % 90 != 0 {
            return Err(PdfError::invalid_range(format!("rotasi {} bukan kelipatan 90", edit.rotate)));
        }
        // Halaman yang sama dua kali akan berbagi satu objek dengan dua Parent
        if !used.insert(edit.page) {
            return Err(PdfError::invalid_range(format!("halaman {} disebut dua kali", edit.page)));
        }
        selection.push((id, edit.rotate));
    }

    let mut out = page_tree::extract_pages(&doc, &selection);
    document::save(&mut out, output)?;
    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(PagesReport { page_count: selection.len(), output_size })
}
