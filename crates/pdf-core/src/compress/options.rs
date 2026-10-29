use serde::Deserialize;

/// Nilai preset (Ringan/Seimbang/Maksimal) ditentukan UI di `presets.ts`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressOptions {
    /// Sisi terpanjang gambar dalam piksel
    pub max_dimension: u32,
    /// Kualitas JPEG 1–100
    pub jpeg_quality: u8,
    /// Ubah gambar menjadi hitam-putih
    pub grayscale: bool,
    /// Target ukuran file (byte). Jika diisi, `max_dimension` & `jpeg_quality`
    /// diabaikan dan dipilih otomatis.
    #[serde(default)]
    pub target_size: Option<u64>,
}
