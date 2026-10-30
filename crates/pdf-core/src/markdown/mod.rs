//! Markdown → PDF dengan font standar PDF (tanpa font tertanam).

pub mod blocks;
pub mod layout;
mod writer;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use lopdf::Stream;
use serde::{Deserialize, Serialize};

use crate::document;
use crate::error::PdfError;
use crate::image_to_pdf::PageSize;
use crate::imaging::{encode, load, transform};
use layout::{ImageSource, PageSetup};

/// Sisi terpanjang gambar di dokumen; cukup tajam untuk dicetak tanpa membuat file besar.
const MAX_IMAGE_PX: u32 = 2000;
const IMAGE_QUALITY: u8 = 85;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FontFamily {
    Sans,
    Serif,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownOptions {
    /// Mode `fit` tidak bermakna untuk dokumen teks dan diperlakukan sebagai A4
    pub page_size: PageSize,
    pub font: FontFamily,
    pub font_size: f32,
    pub margin_mm: f32,
    pub page_numbers: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownReport {
    pub page_count: usize,
    pub output_size: u64,
}

/// Gambar dari file lokal; path relatif dibaca terhadap folder file Markdown.
struct FileImages<'a> {
    base_dir: Option<&'a Path>,
    streams: Vec<Stream>,
    loaded: HashMap<String, Option<(usize, u32, u32)>>,
}

impl FileImages<'_> {
    fn resolve(&self, src: &str) -> Option<PathBuf> {
        // Aplikasi offline: gambar dari internet tidak diunduh
        if src.starts_with("http://") || src.starts_with("https://") {
            return None;
        }
        let path = PathBuf::from(src.strip_prefix("file://").unwrap_or(src));
        if path.is_absolute() { Some(path) } else { self.base_dir.map(|dir| dir.join(path)) }
    }
}

impl ImageSource for FileImages<'_> {
    fn load(&mut self, src: &str) -> Option<(usize, u32, u32)> {
        if let Some(cached) = self.loaded.get(src) {
            return *cached;
        }
        let result = self.resolve(src).and_then(|path| load::load(&path).ok()).and_then(|loaded| {
            let img = transform::fit_within(transform::flatten_on_white(loaded.image), MAX_IMAGE_PX);
            let (w, h) = (img.width(), img.height());
            let stream = encode::encode_jpeg(&img, IMAGE_QUALITY).ok()?.into_stream();
            self.streams.push(stream);
            Some((self.streams.len() - 1, w, h))
        });
        self.loaded.insert(src.to_string(), result);
        result
    }
}

pub fn markdown_to_pdf(
    markdown: &str,
    base_dir: Option<&Path>,
    output: &Path,
    options: &MarkdownOptions,
) -> Result<MarkdownReport, PdfError> {
    let (short, long) = options.page_size.portrait_mm().unwrap_or((210.0, 297.0));
    let mm = 72.0 / 25.4;
    let setup = PageSetup {
        width: short * mm,
        height: long * mm,
        margin: options.margin_mm.clamp(5.0, 50.0) * mm,
        font_size: options.font_size.clamp(6.0, 24.0),
        serif: options.font == FontFamily::Serif,
        page_numbers: options.page_numbers,
    };

    let blocks = blocks::parse(markdown);
    let mut images = FileImages { base_dir, streams: Vec::new(), loaded: HashMap::new() };
    let pages = layout::layout(&blocks, &setup, &mut images);
    let mut doc = writer::write(&pages, images.streams, &setup);
    doc.compress();

    document::save(&mut doc, output)?;
    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(MarkdownReport { page_count: pages.len(), output_size })
}
