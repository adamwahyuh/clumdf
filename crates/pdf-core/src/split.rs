use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::document;
use crate::error::PdfError;
use crate::page_tree;
use crate::progress::ProgressReporter;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "mode", rename_all = "camelCase")]
pub enum SplitMode {
    /// Satu file per halaman
    EachPage,
    /// Satu file per rentang, mis. "1-3, 5, 8-10"
    Ranges { ranges: String },
    /// Satu file tiap `n` halaman
    EveryN { n: u32 },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitReport {
    pub files: Vec<PathBuf>,
}

/// Rentang halaman (inklusif, mulai dari 1) yang masing-masing jadi satu file.
pub fn plan(mode: &SplitMode, page_count: u32) -> Result<Vec<(u32, u32)>, PdfError> {
    match mode {
        SplitMode::EachPage => Ok((1..=page_count).map(|p| (p, p)).collect()),
        SplitMode::EveryN { n: 0 } => Err(PdfError::invalid_range("jumlah halaman per file harus lebih dari 0")),
        SplitMode::EveryN { n } => Ok((1..=page_count)
            .step_by(*n as usize)
            .map(|start| (start, (start + n - 1).min(page_count)))
            .collect()),
        SplitMode::Ranges { ranges } => parse_ranges(ranges, page_count),
    }
}

/// "1-3, 5, 8-" → [(1,3), (5,5), (8,page_count)]. Bagian kosong dilewati.
pub fn parse_ranges(text: &str, page_count: u32) -> Result<Vec<(u32, u32)>, PdfError> {
    let number = |s: &str| -> Result<u32, PdfError> {
        let n: u32 = s.trim().parse().map_err(|_| PdfError::invalid_range(format!("'{}' bukan angka", s.trim())))?;
        if n == 0 || n > page_count {
            return Err(PdfError::invalid_range(format!("halaman {n} tidak ada (1–{page_count})")));
        }
        Ok(n)
    };

    let mut ranges = Vec::new();
    for part in text.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        // Terima juga en dash yang sering muncul dari copy-paste
        let range = match part.split_once(['-', '–']) {
            None => (number(part)?, number(part)?),
            Some((start, end)) if end.trim().is_empty() => (number(start)?, page_count),
            Some((start, end)) => (number(start)?, number(end)?),
        };
        if range.0 > range.1 {
            return Err(PdfError::invalid_range(format!("{part}: awal lebih besar dari akhir")));
        }
        ranges.push(range);
    }
    if ranges.is_empty() {
        return Err(PdfError::invalid_range("belum ada rentang"));
    }
    Ok(ranges)
}

/// Pecah `input` menjadi beberapa PDF di `output_dir`, mis. "Surat-1-3.pdf".
/// Nama yang sudah dipakai diberi akhiran " (2)" supaya tidak menimpa file.
pub fn split_pdf(
    input: &Path,
    output_dir: &Path,
    mode: &SplitMode,
    reporter: &impl ProgressReporter,
) -> Result<SplitReport, PdfError> {
    let doc = document::open(input)?;
    let pages = doc.get_pages();
    let groups = plan(mode, pages.len() as u32)?;
    let stem = input.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();

    let mut files = Vec::with_capacity(groups.len());
    for (i, &(start, end)) in groups.iter().enumerate() {
        let selection: Vec<_> = (start..=end).map(|p| (pages[&p], 0)).collect();
        let mut out = page_tree::extract_pages(&doc, &selection);
        let name = if start == end { format!("{stem}-{start}.pdf") } else { format!("{stem}-{start}-{end}.pdf") };
        let path = document::unique_path(&output_dir.join(name));
        document::save(&mut out, &path)?;
        files.push(path);
        reporter.report(i + 1, groups.len());
    }
    Ok(SplitReport { files })
}
