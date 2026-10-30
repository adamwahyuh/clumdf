//! OCR: halaman hasil scan diberi lapisan teks tak terlihat supaya bisa dicari & di-copy.

pub mod tesseract;
pub mod text_layer;
pub mod tsv;

use std::io::Cursor;
use std::path::Path;

use image::ImageFormat;
use lopdf::{Dictionary, Object};
use pdfium_render::prelude::*;
use serde::Serialize;

use crate::error::PdfError;
use crate::progress::ProgressReporter;
use crate::{document, overlay, render};

/// Resolusi render untuk OCR; 300 DPI adalah titik manis akurasi tesseract.
const OCR_DPI: f32 = 300.0;
/// Halaman dengan teks lebih dari ini dianggap sudah punya teks (bukan hasil scan).
const EXISTING_TEXT_CHARS: usize = 20;
const LANGUAGES: &str = "ind+eng";
const FONT_NAME: &str = "BerkasOcr";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrReport {
    pub pages_recognized: usize,
    /// Halaman yang dilewati karena sudah punya teks
    pub pages_skipped: usize,
    pub words: usize,
    pub output_size: u64,
}

/// Lokasi program & data yang dibutuhkan OCR.
pub struct OcrTools<'a> {
    pub pdfium_dir: &'a Path,
    pub tesseract: &'a Path,
    pub tessdata_dir: &'a Path,
}

pub fn ocr_pdf(tools: &OcrTools, input: &Path, output: &Path, reporter: &impl ProgressReporter) -> Result<OcrReport, PdfError> {
    // 1. Kenali kata di tiap halaman (PDFium untuk render, tesseract untuk OCR)
    let pdfium = render::pdfium(tools.pdfium_dir)?;
    let source = pdfium.load_pdf_from_file(input, None).map_err(|e| match e {
        PdfiumError::PdfiumLibraryInternalError(PdfiumInternalError::PasswordError) => {
            PdfError::Encrypted { path: input.to_path_buf() }
        }
        other => PdfError::invalid(input, other),
    })?;
    let config = PdfRenderConfig::new().scale_page_by_factor(OCR_DPI / 72.0).render_form_data(true);
    let total = source.pages().len() as usize;
    let mut recognized: Vec<Option<(Vec<tsv::Word>, (u32, u32))>> = Vec::with_capacity(total);

    for (i, page) in source.pages().iter().enumerate() {
        let has_text = page.text().map(|t| t.all().trim().chars().count() > EXISTING_TEXT_CHARS).unwrap_or(false);
        if has_text {
            recognized.push(None);
        } else {
            let image = page
                .render_with_config(&config)
                .and_then(|bitmap| bitmap.as_image())
                .map_err(|e| PdfError::invalid(input, e))?
                .to_luma8();
            let mut png = Vec::new();
            image
                .write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
                .map_err(|e| PdfError::invalid(input, e))?;
            let tsv = tesseract::recognize(tools.tesseract, tools.tessdata_dir, LANGUAGES, &png)?;
            recognized.push(Some((tsv::parse_words(&tsv), image.dimensions())));
        }
        reporter.report(i + 1, total + 1);
    }
    drop(source);

    // 2. Tulis lapisan teks tak terlihat di atas halaman asli (tampilan tidak berubah)
    let mut doc = document::open(input)?;
    let page_ids: Vec<_> = doc.get_pages().into_values().collect();
    let font = doc.add_object(Dictionary::from_iter([
        ("Type", Object::Name(b"Font".to_vec())),
        ("Subtype", Object::Name(b"Type1".to_vec())),
        ("BaseFont", Object::Name(b"Helvetica".to_vec())),
        ("Encoding", Object::Name(b"WinAnsiEncoding".to_vec())),
    ]));
    let save_state = overlay::save_state_stream(&mut doc);
    let (mut pages_recognized, mut words) = (0, 0);

    for (page_id, result) in page_ids.iter().zip(&recognized) {
        let Some((page_words, image_px)) = result else { continue };
        pages_recognized += 1;
        if page_words.is_empty() {
            continue;
        }
        let page = overlay::prepare_page(&mut doc, *page_id, b"Font", FONT_NAME, font)
            .map_err(|e| PdfError::invalid(input, e))?;
        let placed: Vec<_> = page_words
            .iter()
            .map(|w| text_layer::place_word(w, *image_px, (page.width, page.height)))
            .collect();
        words += placed.len();
        let [a, b, c, d, e, f] = page.matrix;
        let layer = format!("q {a} {b} {c} {d} {e} {f} cm\n{}Q\n", text_layer::content(&placed, FONT_NAME));
        overlay::wrap_contents(&mut doc, *page_id, save_state, layer).map_err(|e| PdfError::invalid(input, e))?;
    }

    doc.compress();
    document::save(&mut doc, output)?;
    reporter.report(total + 1, total + 1);
    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(OcrReport { pages_recognized, pages_skipped: total - pages_recognized, words, output_size })
}
