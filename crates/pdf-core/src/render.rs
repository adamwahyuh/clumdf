//! PDF → gambar lewat PDFium (library terpisah, dimuat saat dipakai).

use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, ImageFormat};
use pdfium_render::prelude::*;
use serde::{Deserialize, Serialize};

use crate::document;
use crate::error::PdfError;
use crate::imaging::transform;
use crate::progress::ProgressReporter;

/// Batas sisi terpanjang gambar; melindungi memori dari halaman raksasa (mis. poster) × DPI tinggi.
const MAX_SIDE_PX: i32 = 10_000;
const JPEG_QUALITY: u8 = 90;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RenderFormat {
    Jpg,
    Png,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderOptions {
    pub format: RenderFormat,
    /// Resolusi (titik per inci), 72–600
    pub dpi: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RenderReport {
    pub files: Vec<PathBuf>,
}

// Binding PDFium global dan hanya boleh dibuat sekali per proses.
static PDFIUM: OnceLock<Pdfium> = OnceLock::new();

pub(crate) fn pdfium(library_dir: &Path) -> Result<&'static Pdfium, PdfError> {
    if let Some(pdfium) = PDFIUM.get() {
        return Ok(pdfium);
    }
    let bindings = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(library_dir))
        .map_err(|e| PdfError::RendererMissing { reason: e.to_string() })?;
    Ok(PDFIUM.get_or_init(|| Pdfium::new(bindings)))
}

/// Simpan tiap halaman `input` sebagai gambar di `output_dir`, mis. "Surat-01.jpg".
/// `library_dir` = folder berisi libpdfium (lihat scripts/fetch-pdfium.sh).
pub fn pdf_to_images(
    library_dir: &Path,
    input: &Path,
    output_dir: &Path,
    options: &RenderOptions,
    reporter: &impl ProgressReporter,
) -> Result<RenderReport, PdfError> {
    let pdfium = pdfium(library_dir)?;
    let doc = pdfium.load_pdf_from_file(input, None).map_err(|e| match e {
        PdfiumError::PdfiumLibraryInternalError(PdfiumInternalError::PasswordError) => {
            PdfError::Encrypted { path: input.to_path_buf() }
        }
        other => PdfError::invalid(input, other),
    })?;

    let pages = doc.pages();
    let total = pages.len() as usize;
    let digits = total.to_string().len();
    let stem = input.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    let (ext, format) = match options.format {
        RenderFormat::Jpg => ("jpg", ImageFormat::Jpeg),
        RenderFormat::Png => ("png", ImageFormat::Png),
    };
    let config = PdfRenderConfig::new()
        .scale_page_by_factor(options.dpi.clamp(72, 600) as f32 / 72.0)
        .set_maximum_width(MAX_SIDE_PX)
        .set_maximum_height(MAX_SIDE_PX)
        .render_form_data(true);

    let mut files = Vec::with_capacity(total);
    for (i, page) in pages.iter().enumerate() {
        let image = page
            .render_with_config(&config)
            .and_then(|bitmap| bitmap.as_image())
            .map_err(|e| PdfError::invalid(input, e))?;
        let bytes = encode(image, format).map_err(|e| PdfError::invalid(input, e))?;
        // Nomor diberi nol di depan supaya urutan file benar di Finder/Explorer
        let name = format!("{stem}-{:0digits$}.{ext}", i + 1);
        let path = document::unique_path(&output_dir.join(name));
        document::write_atomic(&path, &bytes)?;
        files.push(path);
        reporter.report(i + 1, total);
    }
    Ok(RenderReport { files })
}

fn encode(image: DynamicImage, format: ImageFormat) -> image::ImageResult<Vec<u8>> {
    let mut bytes = Vec::new();
    if format == ImageFormat::Jpeg {
        // JPEG tidak punya transparansi; latar halaman tanpa warna jadi putih
        transform::flatten_on_white(image)
            .to_rgb8()
            .write_with_encoder(JpegEncoder::new_with_quality(&mut bytes, JPEG_QUALITY))?;
    } else {
        image.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)?;
    }
    Ok(bytes)
}
