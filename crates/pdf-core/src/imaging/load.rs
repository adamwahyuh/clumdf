use std::path::Path;

use image::metadata::Orientation;
use image::{DynamicImage, ImageDecoder, ImageFormat, ImageReader};

use crate::error::PdfError;

pub struct Loaded {
    /// Sudah diputar sesuai EXIF
    pub image: DynamicImage,
    pub format: ImageFormat,
    /// true jika EXIF meminta rotasi/flip (byte asli tidak bisa dipakai apa adanya)
    pub reoriented: bool,
}

/// Buka gambar dan terapkan rotasi EXIF (foto HP sering tersimpan "miring").
pub fn load(path: &Path) -> Result<Loaded, PdfError> {
    let reader = ImageReader::open(path)
        .and_then(|r| r.with_guessed_format())
        .map_err(|e| PdfError::io(path, e))?;
    let format = reader.format().ok_or_else(|| PdfError::invalid_image(path, "format tidak dikenal"))?;
    let mut decoder = reader.into_decoder().map_err(|e| PdfError::invalid_image(path, e))?;
    let orientation = decoder.orientation().unwrap_or(Orientation::NoTransforms);
    let mut image = DynamicImage::from_decoder(decoder).map_err(|e| PdfError::invalid_image(path, e))?;
    image.apply_orientation(orientation);

    Ok(Loaded { image, format, reoriented: orientation != Orientation::NoTransforms })
}
