//! Menjalankan program tesseract yang terpasang di sistem.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::error::PdfError;

/// Lokasi umum tesseract. App GUI di macOS tidak mewarisi PATH dari terminal,
/// jadi lokasi Homebrew perlu dicek langsung.
const KNOWN_PATHS: [&str; 5] = [
    "/opt/homebrew/bin/tesseract",
    "/usr/local/bin/tesseract",
    "/usr/bin/tesseract",
    r"C:\Program Files\Tesseract-OCR\tesseract.exe",
    r"C:\Program Files (x86)\Tesseract-OCR\tesseract.exe",
];

/// Cari tesseract: env `BERKAS_TESSERACT`, lokasi umum, lalu PATH.
pub fn find() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("BERKAS_TESSERACT").map(PathBuf::from) {
        return path.is_file().then_some(path);
    }
    let exe = if cfg!(windows) { "tesseract.exe" } else { "tesseract" };
    KNOWN_PATHS
        .iter()
        .map(PathBuf::from)
        .chain(std::env::var_os("PATH").iter().flat_map(std::env::split_paths).map(|dir| dir.join(exe)))
        .find(|p| p.is_file())
}

/// OCR satu gambar PNG (lewat stdin) dan kembalikan TSV hasilnya.
pub fn recognize(tesseract: &Path, tessdata_dir: &Path, languages: &str, png: &[u8]) -> Result<String, PdfError> {
    let failed = |reason: String| PdfError::OcrFailed { reason };
    let mut command = Command::new(tesseract);
    command
        .args(["stdin", "stdout", "--tessdata-dir"])
        .arg(tessdata_dir)
        .args(["-l", languages, "--psm", "3", "tsv"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(windows)]
    {
        // Jangan munculkan jendela konsol untuk tiap halaman
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }

    let mut child = command.spawn().map_err(|e| PdfError::OcrMissing { reason: e.to_string() })?;
    // Tulis di thread terpisah: tesseract bisa mulai menulis stdout sebelum stdin habis dibaca
    let mut stdin = child.stdin.take().ok_or_else(|| failed("stdin tidak tersedia".into()))?;
    let input = png.to_vec();
    let writer = std::thread::spawn(move || stdin.write_all(&input));
    let output = child.wait_with_output().map_err(|e| failed(e.to_string()))?;
    let _ = writer.join();

    if !output.status.success() {
        return Err(failed(String::from_utf8_lossy(&output.stderr).trim().to_string()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
