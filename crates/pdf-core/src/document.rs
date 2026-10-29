use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use lopdf::{Document, LoadOptions};

use crate::error::PdfError;

/// Batas dekompresi per stream, melindungi dari "decompression bomb".
const MAX_DECOMPRESSED_SIZE: usize = 512 * 1024 * 1024;

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

/// Simpan dengan object streams, lewat file sementara + rename supaya file
/// tujuan tidak pernah setengah jadi kalau proses gagal.
pub fn save(doc: &mut Document, path: &Path) -> Result<(), PdfError> {
    let tmp = temp_path(path);
    let result = write_to(doc, &tmp).and_then(|()| fs::rename(&tmp, path));
    if let Err(source) = result {
        let _ = fs::remove_file(&tmp);
        return Err(PdfError::io(path, source));
    }
    Ok(())
}

fn write_to(doc: &mut Document, path: &Path) -> std::io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    doc.save_modern(&mut writer).map_err(std::io::Error::other)?;
    writer.flush()?;
    writer.get_ref().sync_all()
}

/// File sementara di folder yang sama, supaya `rename` atomik (satu filesystem).
fn temp_path(path: &Path) -> PathBuf {
    let name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
    path.with_file_name(format!(".{name}.{}.tmp", std::process::id()))
}
