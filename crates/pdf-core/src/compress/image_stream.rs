use image::{DynamicImage, GrayImage, ImageFormat, RgbImage};
use lopdf::{Object, Stream};

use super::CompressOptions;
use crate::document::MAX_DECOMPRESSED_SIZE;
use crate::imaging::{encode, transform};

/// Gambar hanya diganti bila hasilnya minimal 5% lebih kecil.
const MIN_SAVING: f64 = 0.05;

/// Kunci dari dict lama yang tetap berlaku untuk gambar baru.
const KEPT_KEYS: [&[u8]; 4] = [b"SMask", b"Mask", b"Intent", b"Interpolate"];

/// Encode ulang satu image XObject yang sudah lolos cek aman.
/// `None` = biarkan apa adanya (format tidak didukung atau tidak lebih kecil).
pub fn recompress(stream: &Stream, channels: u8, options: &CompressOptions) -> Option<Stream> {
    let img = decode(stream, channels)?;
    let img = transform::fit_within(img, options.max_dimension.max(16));
    let img = if options.grayscale { transform::to_grayscale(img) } else { img };
    let encoded = encode::encode_jpeg(&img, options.jpeg_quality).ok()?;

    if encoded.bytes.len() as f64 > stream.content.len() as f64 * (1.0 - MIN_SAVING) {
        return None;
    }
    let mut new = encoded.into_stream();
    for key in KEPT_KEYS {
        if let Ok(value) = stream.dict.get(key) {
            new.dict.set(key, value.clone());
        }
    }
    Some(new)
}

fn decode(stream: &Stream, channels: u8) -> Option<DynamicImage> {
    let width = dimension(stream, b"Width")?;
    let height = dimension(stream, b"Height")?;
    let filters = stream.filters().unwrap_or_default();

    match filters.as_slice() {
        [b"DCTDecode"] => image::load_from_memory_with_format(&stream.content, ImageFormat::Jpeg).ok(),
        [] | [b"FlateDecode"] => {
            let raw = if filters.is_empty() {
                stream.content.clone()
            } else {
                stream.decompressed_content_with_limit(MAX_DECOMPRESSED_SIZE).ok()?
            };
            if channels == 1 {
                GrayImage::from_raw(width, height, raw).map(DynamicImage::ImageLuma8)
            } else {
                RgbImage::from_raw(width, height, raw).map(DynamicImage::ImageRgb8)
            }
        }
        // JPX, JBIG2, CCITT, rantai filter, dll. dilewati
        _ => None,
    }
}

fn dimension(stream: &Stream, key: &[u8]) -> Option<u32> {
    let value = stream.dict.get(key).and_then(Object::as_i64).ok()?;
    u32::try_from(value).ok().filter(|&v| v > 0)
}
