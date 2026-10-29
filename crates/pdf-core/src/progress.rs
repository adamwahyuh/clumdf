/// Tujuan laporan progress. Tauri meneruskannya sebagai event, tes cukup memakai closure.
pub trait ProgressReporter: Sync {
    fn report(&self, current: usize, total: usize);
}

impl<F: Fn(usize, usize) + Sync> ProgressReporter for F {
    fn report(&self, current: usize, total: usize) {
        self(current, total)
    }
}
