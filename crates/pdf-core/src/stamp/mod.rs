pub mod helvetica;
pub mod layout;

pub use layout::Position;

use std::fmt::Write as _;
use std::path::Path;

use lopdf::{Dictionary, Object, ObjectId};
use serde::{Deserialize, Serialize};

use crate::document;
use crate::error::PdfError;
use crate::overlay;

/// Nama font di Resources halaman; dibuat unik supaya tidak bentrok dengan font asli.
const FONT_NAME: &str = "BerkasHelv";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStamp {
    /// Teks dengan penanda `{page}` dan `{total}`
    pub text: String,
    pub position: Position,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StampOptions {
    pub stamps: Vec<TextStamp>,
    pub font_size: f32,
    /// Jarak dari tepi halaman (mm)
    pub margin_mm: f32,
    /// Nomor untuk halaman pertama yang diberi teks
    pub first_number: u32,
    /// Lewati halaman pertama (mis. sampul); penomoran dimulai di halaman 2
    pub skip_first: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StampReport {
    pub page_count: usize,
    pub output_size: u64,
}

/// Tambahkan teks (nomor halaman, header, footer) ke setiap halaman.
pub fn stamp_text(input: &Path, output: &Path, options: &StampOptions) -> Result<StampReport, PdfError> {
    if options.stamps.iter().all(|s| s.text.trim().is_empty()) {
        return Err(PdfError::NoInput);
    }
    let mut doc = document::open(input)?;
    let pages: Vec<ObjectId> = doc.get_pages().into_values().collect();
    let skip = usize::from(options.skip_first);
    let stamped = pages.len().saturating_sub(skip);
    let total = options.first_number as usize + stamped.saturating_sub(1);
    let font_size = options.font_size.clamp(4.0, 72.0);
    let margin = options.margin_mm.clamp(0.0, 100.0) * 72.0 / 25.4;

    let font = doc.add_object(Dictionary::from_iter([
        ("Type", Object::Name(b"Font".to_vec())),
        ("Subtype", Object::Name(b"Type1".to_vec())),
        ("BaseFont", Object::Name(b"Helvetica".to_vec())),
        ("Encoding", Object::Name(b"WinAnsiEncoding".to_vec())),
    ]));
    let save_state = overlay::save_state_stream(&mut doc);

    for (i, &page_id) in pages.iter().enumerate().skip(skip) {
        let number = options.first_number as usize + i - skip;
        let page = overlay::prepare_page(&mut doc, page_id, b"Font", FONT_NAME, font).map_err(|e| PdfError::invalid(input, e))?;
        let mut content = String::new();
        for stamp in options.stamps.iter().filter(|s| !s.text.trim().is_empty()) {
            let text = stamp.text.replace("{page}", &number.to_string()).replace("{total}", &total.to_string());
            let encoded = helvetica::encode(&text);
            let width = helvetica::text_width(&encoded, font_size);
            let (x, y) = layout::text_origin(&page, stamp.position, width, font_size, margin);
            let [a, b, c, d, e, f] = page.matrix;
            let hex: String = encoded.iter().map(|byte| format!("{byte:02X}")).collect();
            let _ = writeln!(
                content,
                "q {a} {b} {c} {d} {e} {f} cm BT 0 g /{FONT_NAME} {font_size} Tf {x:.2} {y:.2} Td <{hex}> Tj ET Q"
            );
        }
        overlay::wrap_contents(&mut doc, page_id, save_state, content).map_err(|e| PdfError::invalid(input, e))?;
    }

    doc.compress();
    document::save(&mut doc, output)?;
    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(StampReport { page_count: pages.len(), output_size })
}
