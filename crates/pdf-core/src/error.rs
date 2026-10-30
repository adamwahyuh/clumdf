use std::path::{Path, PathBuf};

/// Error dengan kode stabil (lihat `code()`); UI memetakan kode ke pesan ramah.
#[derive(Debug, thiserror::Error)]
pub enum PdfError {
    #[error("PDF dikunci dengan password: {path}")]
    Encrypted { path: PathBuf },

    #[error("PDF rusak atau tidak bisa dibaca ({reason}): {path}")]
    Invalid { path: PathBuf, reason: String },

    #[error("gambar rusak atau formatnya tidak didukung ({reason}): {path}")]
    InvalidImage { path: PathBuf, reason: String },

    #[error("password salah: {path}")]
    WrongPassword { path: PathBuf },

    #[error("PDF ini tidak dikunci: {path}")]
    NotEncrypted { path: PathBuf },

    #[error("password tidak boleh kosong")]
    EmptyPassword,

    #[error("rentang halaman tidak valid: {reason}")]
    InvalidRange { reason: String },

    #[error("gagal membaca/menulis file ({source}): {path}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("library PDFium tidak ditemukan ({reason})")]
    RendererMissing { reason: String },

    #[error("program OCR (tesseract) tidak ditemukan ({reason})")]
    OcrMissing { reason: String },

    #[error("OCR gagal: {reason}")]
    OcrFailed { reason: String },

    #[error("tidak ada file untuk diproses")]
    NoInput,
}

impl PdfError {
    pub fn code(&self) -> &'static str {
        match self {
            PdfError::Encrypted { .. } => "encrypted",
            PdfError::Invalid { .. } => "invalid",
            PdfError::InvalidImage { .. } => "invalid_image",
            PdfError::WrongPassword { .. } => "wrong_password",
            PdfError::NotEncrypted { .. } => "not_encrypted",
            PdfError::EmptyPassword => "empty_password",
            PdfError::InvalidRange { .. } => "invalid_range",
            PdfError::Io { .. } => "io",
            PdfError::RendererMissing { .. } => "renderer_missing",
            PdfError::OcrMissing { .. } => "ocr_missing",
            PdfError::OcrFailed { .. } => "ocr_failed",
            PdfError::NoInput => "no_input",
        }
    }

    pub fn path(&self) -> Option<&Path> {
        match self {
            PdfError::Encrypted { path }
            | PdfError::Invalid { path, .. }
            | PdfError::InvalidImage { path, .. }
            | PdfError::WrongPassword { path }
            | PdfError::NotEncrypted { path }
            | PdfError::Io { path, .. } => Some(path),
            PdfError::EmptyPassword
            | PdfError::InvalidRange { .. }
            | PdfError::RendererMissing { .. }
            | PdfError::OcrMissing { .. }
            | PdfError::OcrFailed { .. }
            | PdfError::NoInput => None,
        }
    }

    pub(crate) fn io(path: &Path, source: std::io::Error) -> Self {
        PdfError::Io { path: path.to_path_buf(), source }
    }

    pub(crate) fn invalid_image(path: &Path, reason: impl ToString) -> Self {
        PdfError::InvalidImage { path: path.to_path_buf(), reason: reason.to_string() }
    }

    pub(crate) fn invalid_range(reason: impl ToString) -> Self {
        PdfError::InvalidRange { reason: reason.to_string() }
    }

    pub(crate) fn invalid(path: &Path, reason: impl ToString) -> Self {
        PdfError::Invalid { path: path.to_path_buf(), reason: reason.to_string() }
    }
}
