use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use lopdf::encryption::crypt_filters::{Aes128CryptFilter, CryptFilter};
use lopdf::{Document, EncryptionState, EncryptionVersion, LoadOptions, Object, Permissions};
use serde::Serialize;

use crate::document::{self, MAX_DECOMPRESSED_SIZE};
use crate::error::PdfError;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordReport {
    pub output_size: u64,
}

/// Kunci PDF dengan AES-128: password diperlukan untuk membuka file.
pub fn protect_pdf(input: &Path, output: &Path, password: &str) -> Result<PasswordReport, PdfError> {
    if password.is_empty() {
        return Err(PdfError::EmptyPassword);
    }
    let mut doc = document::open(input)?;
    let io = |e: std::io::Error| PdfError::io(output, e);

    // Enkripsi butuh ID dokumen; PDF buatan sebagian aplikasi tidak punya
    if !doc.trailer.has(b"ID") {
        let id = Object::String(random_bytes::<16>().map_err(io)?.to_vec(), lopdf::StringFormat::Hexadecimal);
        doc.trailer.set("ID", vec![id.clone(), id]);
    }
    // AES-128 (V4), bukan AES-256 (V5): output V5 dari lopdf 0.46 tidak bisa
    // didekripsi viewer lain (diuji dengan poppler), sedangkan V4 terbaca di mana pun.
    let filters: BTreeMap<Vec<u8>, Arc<dyn CryptFilter>> = BTreeMap::from([(b"StdCF".to_vec(), Arc::new(Aes128CryptFilter) as _)]);
    let state = EncryptionState::try_from(EncryptionVersion::V4 {
        document: &doc,
        encrypt_metadata: true,
        crypt_filters: filters,
        stream_filter: b"StdCF".to_vec(),
        string_filter: b"StdCF".to_vec(),
        // shortcut: password pemilik = password pembuka; pisahkan bila perlu izin cetak/salin
        owner_password: password,
        user_password: password,
        permissions: Permissions::all(),
    })
    .map_err(|e| PdfError::invalid(input, e))?;
    doc.encrypt(&state).map_err(|e| PdfError::invalid(input, e))?;

    finish(&mut doc, output)
}

/// Buka kunci PDF dengan password lalu simpan tanpa enkripsi.
pub fn unlock_pdf(input: &Path, output: &Path, password: &str) -> Result<PasswordReport, PdfError> {
    if password.is_empty() {
        return Err(PdfError::EmptyPassword);
    }
    let options = LoadOptions { max_decompressed_size: Some(MAX_DECOMPRESSED_SIZE), ..LoadOptions::with_password(password) };
    let wrong = || PdfError::WrongPassword { path: input.to_path_buf() };
    let mut doc = Document::load_with_options(input, options).map_err(|err| match err {
        lopdf::Error::IO(source) => PdfError::io(input, source),
        lopdf::Error::InvalidPassword | lopdf::Error::Decryption(_) => wrong(),
        other => PdfError::invalid(input, other),
    })?;

    if !doc.was_encrypted() {
        return Err(PdfError::NotEncrypted { path: input.to_path_buf() });
    }
    if doc.is_encrypted() {
        return Err(wrong());
    }
    // State lama hanya untuk incremental save; tanpa ini hasil bisa ikut terenkripsi lagi
    doc.encryption_state = None;
    finish(&mut doc, output)
}

fn finish(doc: &mut Document, output: &Path) -> Result<PasswordReport, PdfError> {
    document::save(doc, output)?;
    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(PasswordReport { output_size })
}

fn random_bytes<const N: usize>() -> std::io::Result<[u8; N]> {
    let mut bytes = [0u8; N];
    getrandom::fill(&mut bytes).map_err(|e| std::io::Error::other(e.to_string()))?;
    Ok(bytes)
}
