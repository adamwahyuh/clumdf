use std::path::Path;

use lopdf::{Dictionary, Document, Object, StringFormat};
use serde::{Deserialize, Serialize};

use crate::document;
use crate::error::PdfError;

/// Kunci yang menyimpan info pembuat/aplikasi: XMP (`Metadata`) dan data
/// aplikasi pengedit (`PieceInfo`), di katalog maupun objek lain.
const METADATA_KEYS: [&[u8]; 2] = [b"Metadata", b"PieceInfo"];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StripReport {
    pub output_size: u64,
    /// Jumlah entri metadata yang dibuang (0 = file memang sudah bersih)
    pub removed: usize,
}

/// Hapus info dokumen (penulis, judul, aplikasi, tanggal) dan metadata XMP.
pub fn strip_metadata(input: &Path, output: &Path) -> Result<StripReport, PdfError> {
    let mut doc = document::open(input)?;
    let mut removed = usize::from(doc.trailer.remove(b"Info").is_some());

    for object in doc.objects.values_mut() {
        let dict = match object {
            Object::Dictionary(dict) => dict,
            Object::Stream(stream) => &mut stream.dict,
            _ => continue,
        };
        removed += METADATA_KEYS.iter().filter(|key| dict.remove(key).is_some()).count();
    }
    // Stream XMP & kamus Info yang kini tak direferensikan ikut terbuang
    doc.prune_objects();

    document::save(&mut doc, output)?;
    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(StripReport { output_size, removed })
}

/// Info dokumen yang bisa diubah user. `None` / kosong = kolom tidak diisi.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    /// Aplikasi yang membuat dokumen asli (mis. Microsoft Word)
    pub creator: Option<String>,
    /// Aplikasi yang menghasilkan PDF-nya
    pub producer: Option<String>,
    /// Hanya dibaca: format PDF mentah, mis. "D:20240131120000+07'00'"
    #[serde(default)]
    pub creation_date: Option<String>,
    #[serde(default)]
    pub mod_date: Option<String>,
}

/// Kolom yang bisa diubah: (kunci PDF, akses field).
const EDITABLE: [(&str, fn(&PdfMetadata) -> &Option<String>); 6] = [
    ("Title", |m| &m.title),
    ("Author", |m| &m.author),
    ("Subject", |m| &m.subject),
    ("Keywords", |m| &m.keywords),
    ("Creator", |m| &m.creator),
    ("Producer", |m| &m.producer),
];

fn info_dict(doc: &Document) -> Option<&Dictionary> {
    match doc.trailer.get(b"Info").ok()? {
        Object::Reference(id) => doc.get_dictionary(*id).ok(),
        Object::Dictionary(dict) => Some(dict),
        _ => None,
    }
}

/// Teks PDF: UTF-16BE (BOM FE FF), UTF-8 (BOM EF BB BF, PDF 2.0), atau PDFDocEncoding (≈ Latin-1).
pub fn decode_text(bytes: &[u8]) -> String {
    if let Some(utf16) = bytes.strip_prefix(&[0xFE, 0xFF]) {
        let units: Vec<u16> = utf16.chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();
        String::from_utf16_lossy(&units)
    } else if let Some(utf8) = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]) {
        String::from_utf8_lossy(utf8).into_owned()
    } else {
        bytes.iter().map(|&b| b as char).collect()
    }
}

/// ASCII ditulis apa adanya; selain itu UTF-16BE dengan BOM supaya huruf apa pun aman.
pub fn encode_text(text: &str) -> Object {
    if text.is_ascii() {
        return Object::String(text.as_bytes().to_vec(), StringFormat::Literal);
    }
    let bytes = [0xFE, 0xFF].into_iter().chain(text.encode_utf16().flat_map(u16::to_be_bytes)).collect();
    Object::String(bytes, StringFormat::Hexadecimal)
}

pub fn read_metadata(input: &Path) -> Result<PdfMetadata, PdfError> {
    let doc = document::open(input)?;
    let Some(info) = info_dict(&doc) else { return Ok(PdfMetadata::default()) };
    let get = |key: &str| {
        let text = decode_text(info.get(key.as_bytes()).and_then(Object::as_str).ok()?);
        Some(text.trim().to_string()).filter(|t| !t.is_empty())
    };
    Ok(PdfMetadata {
        title: get("Title"),
        author: get("Author"),
        subject: get("Subject"),
        keywords: get("Keywords"),
        creator: get("Creator"),
        producer: get("Producer"),
        creation_date: get("CreationDate"),
        mod_date: get("ModDate"),
    })
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditReport {
    pub output_size: u64,
}

/// Simpan info dokumen baru. Kolom kosong dihapus; tanggal dibuat/diubah dipertahankan.
pub fn write_metadata(input: &Path, output: &Path, metadata: &PdfMetadata) -> Result<EditReport, PdfError> {
    let mut doc = document::open(input)?;
    let mut info = info_dict(&doc).cloned().unwrap_or_default();
    for (key, field) in EDITABLE {
        match field(metadata).as_deref().map(str::trim).filter(|v| !v.is_empty()) {
            Some(value) => info.set(key, encode_text(value)),
            None => {
                info.remove(key.as_bytes());
            }
        }
    }
    let info_id = doc.add_object(info);
    doc.trailer.set("Info", info_id);
    // XMP lama dibuang: Acrobat mengutamakan XMP, jadi perubahan di Info bisa tak terlihat
    if let Ok(catalog) = doc.catalog_mut() {
        catalog.remove(b"Metadata");
    }
    doc.prune_objects();

    document::save(&mut doc, output)?;
    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(EditReport { output_size })
}
