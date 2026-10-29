use std::path::Path;

use image::ImageFormat;
use lopdf::Stream;

use super::options::ImageQuality;
use crate::error::PdfError;
use crate::imaging::encode::{self, Encoded};
use crate::imaging::{load, transform};

/// Gambar → image XObject, plus ukuran (px, setelah rotasi EXIF) untuk layout.
pub fn embed(path: &Path, quality: ImageQuality) -> Result<(Stream, (u32, u32)), PdfError> {
    let loaded = load::load(path)?;
    let size = (loaded.image.width(), loaded.image.height());

    // Kualitas asli + JPEG tanpa rotasi: pakai byte JPEG apa adanya, tanpa re-encode
    if quality == ImageQuality::Original && loaded.format == ImageFormat::Jpeg && !loaded.reoriented {
        let bytes = std::fs::read(path).map_err(|e| PdfError::io(path, e))?;
        if let Some(components @ (1 | 3)) = jpeg_components(&bytes) {
            let encoded = Encoded { bytes, width: size.0, height: size.1, gray: components == 1, jpeg: true };
            return Ok((encoded.into_stream(), size));
        }
    }

    let img = transform::flatten_on_white(loaded.image);
    let encoded = match quality.limits() {
        None => encode::encode_lossless(&img),
        Some((max, q)) => encode::encode_jpeg(&transform::fit_within(img, max), q)
            .map_err(|e| PdfError::invalid_image(path, e))?,
    };
    Ok((encoded.into_stream(), size))
}

/// Jumlah komponen warna dari header SOF JPEG (1 = gray, 3 = RGB, 4 = CMYK).
/// Dipakai supaya JPEG CMYK tidak dimasukkan mentah sebagai DeviceRGB.
fn jpeg_components(bytes: &[u8]) -> Option<u8> {
    let mut i = 2; // lewati SOI (FF D8)
    while i + 4 <= bytes.len() {
        if bytes[i] != 0xFF {
            return None;
        }
        let marker = bytes[i + 1];
        let len = u16::from_be_bytes([bytes[i + 2], bytes[i + 3]]) as usize;
        // SOF0..SOF15, kecuali DHT (C4), JPG (C8), DAC (CC)
        if (0xC0..=0xCF).contains(&marker) && ![0xC4, 0xC8, 0xCC].contains(&marker) {
            return bytes.get(i + 9).copied();
        }
        i += 2 + len;
    }
    None
}
