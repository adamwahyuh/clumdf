pub mod compress;
pub mod files;
pub mod image_to_pdf;
pub mod inspect;
pub mod merge;
pub mod metadata;
pub mod organize;
pub mod password;
pub mod signature;
pub mod split;
pub mod stamp;

use crate::error::CommandResult;

/// Jalankan pekerjaan berat di thread terpisah supaya UI tidak freeze.
async fn run_blocking<T, F>(job: F) -> CommandResult<T>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, pdf_core::PdfError> + Send + 'static,
{
    Ok(tauri::async_runtime::spawn_blocking(job).await??)
}
