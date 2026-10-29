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

    #[error("rentang halaman tidak valid: {reason}")]
    InvalidRange { reason: String },

    #[error("gagal membaca/menulis file ({source}): {path}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("tidak ada file untuk diproses")]
    NoInput,
}

impl PdfError {
    pub fn code(&self) -> &'static str {
        match self {
            PdfError::Encrypted { .. } => "encrypted",
            PdfError::Invalid { .. } => "invalid",
            PdfError::InvalidImage { .. } => "invalid_image",
            PdfError::InvalidRange { .. } => "invalid_range",
            PdfError::Io { .. } => "io",
            PdfError::NoInput => "no_input",
        }
    }

    pub fn path(&self) -> Option<&Path> {
        match self {
            PdfError::Encrypted { path }
            | PdfError::Invalid { path, .. }
            | PdfError::InvalidImage { path, .. }
            | PdfError::Io { path, .. } => Some(path),
            PdfError::InvalidRange { .. } | PdfError::NoInput => None,
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
