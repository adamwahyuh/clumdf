use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use lopdf::{Document, LoadOptions};

use crate::error::PdfError;

/// Batas dekompresi per stream, melindungi dari "decompression bomb".
pub(crate) const MAX_DECOMPRESSED_SIZE: usize = 512 * 1024 * 1024;

/// Buka PDF. PDF yang masih terkunci setelah dibuka dilaporkan sebagai `Encrypted`.
pub fn open(path: &Path) -> Result<Document, PdfError> {
    let options = LoadOptions::with_max_decompressed_size(MAX_DECOMPRESSED_SIZE);
    let doc = Document::load_with_options(path, options).map_err(|err| match err {
        lopdf::Error::IO(source) => PdfError::io(path, source),
        lopdf::Error::InvalidPassword | lopdf::Error::Decryption(_) => {
            PdfError::Encrypted { path: path.to_path_buf() }
        }
        other => PdfError::invalid(path, other),
    })?;

    if doc.is_encrypted() {
        return Err(PdfError::Encrypted { path: path.to_path_buf() });
    }
    if doc.get_pages().is_empty() {
        return Err(PdfError::invalid(path, "tidak ada halaman"));
    }
    Ok(doc)
}

/// Simpan PDF ke `path` (object streams + tulis atomik).
pub fn save(doc: &mut Document, path: &Path) -> Result<(), PdfError> {
    let bytes = to_bytes(doc, path)?;
    write_atomic(path, &bytes)
}

/// Serialisasi dengan object streams supaya file lebih kecil.
/// `path` hanya untuk pesan error.
pub fn to_bytes(doc: &mut Document, path: &Path) -> Result<Vec<u8>, PdfError> {
    let mut bytes = Vec::new();
    doc.save_modern(&mut bytes).map_err(|e| PdfError::io(path, std::io::Error::other(e)))?;
    Ok(bytes)
}

/// Tulis ke file sementara lalu rename, supaya file tujuan tidak pernah
/// setengah jadi kalau proses gagal.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<(), PdfError> {
    let tmp = temp_path(path);
    let result = write_synced(&tmp, bytes).and_then(|()| fs::rename(&tmp, path));
    if let Err(source) = result {
        let _ = fs::remove_file(&tmp);
        return Err(PdfError::io(path, source));
    }
    Ok(())
}

fn write_synced(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    file.sync_all()
}

/// File sementara di folder yang sama, supaya `rename` atomik (satu filesystem).
fn temp_path(path: &Path) -> PathBuf {
    let name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
    path.with_file_name(format!(".{name}.{}.tmp", std::process::id()))
}
