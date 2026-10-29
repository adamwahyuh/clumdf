//! Logika inti Berkas: semua pemrosesan PDF ada di sini supaya bisa dites
//! dengan `cargo test` tanpa Tauri.

pub mod document;
pub mod error;
pub mod inspect;
pub mod merge;
pub mod progress;

pub use error::PdfError;
pub use inspect::{PdfInfo, inspect};
pub use merge::{MergeReport, merge_pdfs};
pub use progress::ProgressReporter;
