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

/// Latar terang jadi transparan, untuk foto tanda tangan di kertas putih.
/// Tepi goresan dibuat setengah transparan supaya tidak bergerigi.
pub fn white_to_transparent(img: DynamicImage) -> DynamicImage {
    const OPAQUE_BELOW: u32 = 200;
    const CLEAR_ABOVE: u32 = 235;
    let mut rgba = img.to_rgba8();
    for pixel in rgba.pixels_mut() {
        let [r, g, b, a] = pixel.0;
        let luma = (r as u32 * 299 + g as u32 * 587 + b as u32 * 114) / 1000;
        let alpha = match luma {
            l if l >= CLEAR_ABOVE => 0,
            l if l <= OPAQUE_BELOW => 255,
            l => (CLEAR_ABOVE - l) * 255 / (CLEAR_ABOVE - OPAQUE_BELOW),
        };
        pixel.0[3] = (alpha.min(a as u32)) as u8;
    }
    DynamicImage::ImageRgba8(rgba)
}

/// Buang tepi transparan; `None` bila seluruh gambar transparan.
pub fn trim_transparent(img: DynamicImage) -> Option<DynamicImage> {
    let rgba = img.to_rgba8();
    let visible = rgba.enumerate_pixels().filter(|(_, _, p)| p.0[3] > 8);
    let (mut x0, mut y0, mut x1, mut y1) = (u32::MAX, u32::MAX, 0, 0);
    for (x, y, _) in visible {
        (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
    }
    (x0 != u32::MAX).then(|| img.crop_imm(x0, y0, x1 - x0 + 1, y1 - y0 + 1))
}
