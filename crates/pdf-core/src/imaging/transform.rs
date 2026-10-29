use image::imageops::FilterType;
use image::{DynamicImage, Rgb, RgbImage};

/// Perkecil supaya sisi terpanjang ≤ `max`, rasio tetap. Gambar kecil tidak diperbesar.
pub fn fit_within(img: DynamicImage, max: u32) -> DynamicImage {
    if img.width() <= max && img.height() <= max {
        return img;
    }
    img.resize(max, max, FilterType::CatmullRom)
}

/// Gambar transparan diberi latar putih (PDF tidak menampilkan alpha tanpa SMask).
pub fn flatten_on_white(img: DynamicImage) -> DynamicImage {
    if !img.color().has_alpha() {
        return img;
    }
    let rgba = img.to_rgba8();
    let rgb = RgbImage::from_fn(rgba.width(), rgba.height(), |x, y| {
        let [r, g, b, a] = rgba.get_pixel(x, y).0;
        let blend = |c: u8| ((c as u16 * a as u16 + 255 * (255 - a as u16)) / 255) as u8;
        Rgb([blend(r), blend(g), blend(b)])
    });
    DynamicImage::ImageRgb8(rgb)
}

pub fn to_grayscale(img: DynamicImage) -> DynamicImage {
    DynamicImage::ImageLuma8(img.to_luma8())
}
