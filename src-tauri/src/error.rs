use pdf_core::PdfError;
use serde::Serialize;

/// Bentuk error yang diterima UI: `{ code, message, path? }`.
#[derive(Debug, Serialize)]
pub struct CommandError {
    code: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

pub type CommandResult<T> = Result<T, CommandError>;

impl From<PdfError> for CommandError {
    fn from(err: PdfError) -> Self {
        CommandError {
            code: err.code(),
            message: err.to_string(),
            path: err.path().map(|p| p.to_string_lossy().into_owned()),
        }
    }
}

impl From<tauri::Error> for CommandError {
    fn from(err: tauri::Error) -> Self {
        CommandError { code: "internal", message: err.to_string(), path: None }
    }
}
