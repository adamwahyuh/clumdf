pub mod helvetica;
pub mod layout;

pub use layout::Position;

use std::fmt::Write as _;
use std::path::Path;

use lopdf::{Dictionary, Document, Object, ObjectId, Stream};
use serde::{Deserialize, Serialize};

use crate::document;
use crate::error::PdfError;
use crate::page_tree;

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
    // Konten asli dibungkus q … Q supaya keadaan grafisnya tidak memengaruhi teks kita
    let save_state = doc.add_object(Stream::new(Dictionary::new(), b"q\n".to_vec()));

    for (i, &page_id) in pages.iter().enumerate().skip(skip) {
        let number = options.first_number as usize + i - skip;
        let page = prepare_page(&mut doc, page_id, font).map_err(|e| PdfError::invalid(input, e))?;
        let mut content = String::from("Q\n");
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
        let stamp_id = doc.add_object(Stream::new(Dictionary::new(), content.into_bytes()));
        wrap_contents(&mut doc, page_id, save_state, stamp_id).map_err(|e| PdfError::invalid(input, e))?;
    }

    doc.compress();
    document::save(&mut doc, output)?;
    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(StampReport { page_count: pages.len(), output_size })
}

/// Salin atribut warisan ke halaman, pasang font kita di Resources halaman,
/// dan hitung bentuk halaman yang terlihat.
fn prepare_page(doc: &mut Document, page_id: ObjectId, font: ObjectId) -> lopdf::Result<layout::VisiblePage> {
    let inherited = page_tree::inherited_attributes(doc, page_id);
    let resolve = |doc: &Document, value: Option<&Object>| -> Dictionary {
        match value {
            Some(Object::Reference(id)) => doc.get_dictionary(*id).cloned().unwrap_or_default(),
            Some(Object::Dictionary(dict)) => dict.clone(),
            _ => Dictionary::new(),
        }
    };

    let page = doc.get_dictionary(page_id)?;
    let lookup = |key: &[u8]| page.get(key).ok().or_else(|| inherited.iter().find(|(k, _)| *k == key).map(|(_, v)| v));
    // Resources bisa dipakai bersama halaman lain; salinan langsung di halaman aman diubah
    let mut resources = resolve(doc, lookup(b"Resources"));
    let mut fonts = resolve(doc, resources.get(b"Font").ok());
    fonts.set(FONT_NAME, font);
    resources.set("Font", fonts);

    let number = |o: &Object| o.as_float().ok().or_else(|| o.as_i64().ok().map(|n| n as f32));
    let bbox = lookup(b"CropBox")
        .or_else(|| lookup(b"MediaBox"))
        .and_then(|o| o.as_array().ok())
        .and_then(|a| Some([number(a.first()?)?, number(a.get(1)?)?, number(a.get(2)?)?, number(a.get(3)?)?]))
        .unwrap_or([0.0, 0.0, 595.0, 842.0]); // A4 bila PDF tidak menyebut ukuran
    let rotate = lookup(b"Rotate").and_then(|o| o.as_i64().ok()).unwrap_or(0);

    let page = doc.get_dictionary_mut(page_id)?;
    for (key, value) in inherited {
        page.set(key, value);
    }
    page.set("Resources", resources);
    Ok(layout::visible_page(bbox, rotate))
}

/// Contents baru: [q, …konten asli…, Q + teks kita].
fn wrap_contents(doc: &mut Document, page_id: ObjectId, save_state: ObjectId, stamp: ObjectId) -> lopdf::Result<()> {
    let page = doc.get_dictionary_mut(page_id)?;
    let mut contents = vec![Object::Reference(save_state)];
    match page.get(b"Contents") {
        Ok(Object::Array(items)) => contents.extend(items.iter().cloned()),
        Ok(other) => contents.push(other.clone()),
        Err(_) => {}
    }
    contents.push(Object::Reference(stamp));
    page.set("Contents", contents);
    Ok(())
}
