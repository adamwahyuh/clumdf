use std::path::Path;

use lopdf::Object;
use serde::Serialize;

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
