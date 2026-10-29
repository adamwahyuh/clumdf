use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use image::{DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};

use crate::document;
use crate::error::PdfError;
use crate::imaging::encode::Encoded;
use crate::imaging::{load, transform};
use crate::overlay;

/// Sisi terpanjang tanda tangan tersimpan; cukup tajam untuk dicetak.
const MAX_SIGNATURE_PX: u32 = 1200;
const XOBJECT_NAME: &str = "BerkasSig";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureInfo {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
}

/// Posisi tanda tangan sebagai pecahan (0–1) halaman yang terlihat, asal di kiri-atas,
/// sama seperti koordinat pratinjau di UI.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Placement {
    /// Nomor halaman, mulai dari 1
    pub page: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignReport {
    pub signed_pages: usize,
    pub output_size: u64,
}

/// Rapikan gambar tanda tangan lalu simpan sebagai PNG transparan di `dest`.
/// Gambar tanpa transparansi (foto di kertas) latar putihnya dibuang.
pub fn prepare_signature(source: &Path, dest: &Path) -> Result<SignatureInfo, PdfError> {
    let img = load::load(source)?.image;
    let has_transparency = img.color().has_alpha() && img.to_rgba8().pixels().any(|p| p.0[3] < 250);
    let img = if has_transparency { img } else { transform::white_to_transparent(img) };
    let img = transform::trim_transparent(img).ok_or_else(|| PdfError::invalid_image(source, "gambar kosong"))?;
    let img = transform::fit_within(img, MAX_SIGNATURE_PX);

    let mut bytes = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .map_err(|e| PdfError::invalid_image(source, e))?;
    document::write_atomic(dest, &bytes)?;
    Ok(SignatureInfo { path: dest.to_path_buf(), width: img.width(), height: img.height() })
}

/// Tempel tanda tangan di posisi-posisi `placements`.
pub fn place_signature(
    input: &Path,
    output: &Path,
    signature: &Path,
    placements: &[Placement],
) -> Result<SignReport, PdfError> {
    if placements.is_empty() {
        return Err(PdfError::NoInput);
    }
    let mut doc = document::open(input)?;
    let pages = doc.get_pages();
    let mut by_page: BTreeMap<u32, Vec<&Placement>> = BTreeMap::new();
    for p in placements {
        let inside = [p.x, p.y, p.width, p.height].iter().all(|v| v.is_finite() && *v >= 0.0)
            && p.width > 0.0
            && p.height > 0.0
            && p.x + p.width <= 1.001
            && p.y + p.height <= 1.001;
        if !inside || !pages.contains_key(&p.page) {
            return Err(PdfError::invalid_range(format!("posisi tanda tangan tidak valid: {p:?}")));
        }
        by_page.entry(p.page).or_default().push(p);
    }

    let image = load::load(signature)?.image;
    let image_id = add_transparent_image(&mut doc, &image);
    let save_state = overlay::save_state_stream(&mut doc);

    for (page_number, items) in &by_page {
        let page_id = pages[page_number];
        let page = overlay::prepare_page(&mut doc, page_id, b"XObject", XOBJECT_NAME, image_id)
            .map_err(|e| PdfError::invalid(input, e))?;
        let [a, b, c, d, e, f] = page.matrix;
        let mut content = String::new();
        for p in items {
            let (w, h) = (p.width * page.width, p.height * page.height);
            let (x, y) = (p.x * page.width, page.height - (p.y + p.height) * page.height);
            let _ = writeln!(content, "q {a} {b} {c} {d} {e} {f} cm {w:.2} 0 0 {h:.2} {x:.2} {y:.2} cm /{XOBJECT_NAME} Do Q");
        }
        overlay::wrap_contents(&mut doc, page_id, save_state, content).map_err(|e| PdfError::invalid(input, e))?;
    }

    doc.compress();
    document::save(&mut doc, output)?;
    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(SignReport { signed_pages: by_page.len(), output_size })
}

/// Gambar RGB lossless + SMask dari kanal alpha, supaya latar tetap tembus pandang.
fn add_transparent_image(doc: &mut lopdf::Document, image: &DynamicImage) -> lopdf::ObjectId {
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    let alpha: Vec<u8> = rgba.pixels().map(|p| p.0[3]).collect();
    let rgb = DynamicImage::ImageRgba8(rgba).to_rgb8().into_raw();

    let mask = Encoded { bytes: alpha, width, height, gray: true, jpeg: false }.into_stream();
    let mask_id = doc.add_object(mask);
    let mut stream = Encoded { bytes: rgb, width, height, gray: false, jpeg: false }.into_stream();
    stream.dict.set("SMask", mask_id);
    doc.add_object(stream)
}
