//! Logika inti Berkas: semua pemrosesan PDF ada di sini supaya bisa dites
//! dengan `cargo test` tanpa Tauri.

pub mod compress;
pub mod document;
pub mod error;
pub mod files;
pub mod fonts;
pub mod image_to_pdf;
pub mod imaging;
pub mod inspect;
pub mod markdown;
pub mod merge;
pub mod metadata;
pub mod ocr;
pub mod organize;
mod overlay;
mod page_tree;
pub mod password;
pub mod progress;
pub mod render;
pub mod signature;
pub mod split;
pub mod stamp;

pub use compress::{CompressOptions, CompressReport, compress_pdf};
pub use error::PdfError;
pub use image_to_pdf::{ImageToPdfOptions, ImageToPdfReport, images_to_pdf};
pub use inspect::{PdfInfo, inspect};
pub use markdown::{MarkdownOptions, MarkdownReport, markdown_to_pdf};
pub use merge::{MergeReport, merge_pdfs};
pub use metadata::{StripReport, strip_metadata};
pub use ocr::{OcrReport, OcrTools, ocr_pdf};
pub use organize::{PageEdit, PagesReport, edit_pages};
pub use password::{PasswordReport, protect_pdf, unlock_pdf};
pub use progress::ProgressReporter;
pub use render::{RenderFormat, RenderOptions, RenderReport, pdf_to_images};
pub use signature::{Placement, SignReport, SignatureInfo, place_signature, prepare_signature};
pub use split::{SplitMode, SplitReport, split_pdf};
pub use stamp::{StampOptions, StampReport, TextStamp, stamp_text};
