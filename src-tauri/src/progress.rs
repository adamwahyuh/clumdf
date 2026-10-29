use pdf_core::ProgressReporter;
use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct JobProgress<'a> {
    job_id: &'a str,
    current: usize,
    total: usize,
}

/// Meneruskan progress pdf-core ke UI sebagai event "job-progress".
pub struct EventReporter {
    pub app: AppHandle,
    pub job_id: String,
}

impl ProgressReporter for EventReporter {
    fn report(&self, current: usize, total: usize) {
        // Progress hanya informasi; gagal emit tidak boleh menggagalkan proses.
        let _ = self.app.emit("job-progress", JobProgress { job_id: &self.job_id, current, total });
    }
}
