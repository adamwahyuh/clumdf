//! Logika inti Berkas: semua pemrosesan PDF ada di sini supaya bisa dites
//! dengan `cargo test` tanpa Tauri.

pub mod compress;
pub mod document;
pub mod error;
pub mod image_to_pdf;
pub mod imaging;
pub mod inspect;
pub mod merge;
pub mod metadata;
pub mod organize;
mod page_tree;
pub mod password;
pub mod progress;
pub mod split;

pub use compress::{CompressOptions, CompressReport, compress_pdf};
pub use error::PdfError;
pub use image_to_pdf::{ImageToPdfOptions, ImageToPdfReport, images_to_pdf};
pub use inspect::{PdfInfo, inspect};
pub use merge::{MergeReport, merge_pdfs};
pub use metadata::{StripReport, strip_metadata};
pub use organize::{PageEdit, PagesReport, edit_pages};
pub use password::{PasswordReport, protect_pdf, unlock_pdf};
pub use progress::ProgressReporter;
pub use split::{SplitMode, SplitReport, split_pdf};
