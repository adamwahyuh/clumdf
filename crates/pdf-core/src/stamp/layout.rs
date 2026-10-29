//! Posisi teks pada halaman yang terlihat. Fungsi murni supaya mudah dites.

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Position {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

/// Halaman sebagaimana terlihat: lebar, tinggi, dan matriks `cm` yang memetakan
/// koordinat terlihat (asal kiri-bawah, tegak) ke koordinat halaman PDF.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisiblePage {
    pub width: f32,
    pub height: f32,
    pub matrix: [f32; 6],
}

/// `bbox` = [x0, y0, x1, y1] dari CropBox/MediaBox; `rotate` = /Rotate halaman
/// (searah jarum jam saat ditampilkan).
pub fn visible_page(bbox: [f32; 4], rotate: i64) -> VisiblePage {
    let (x0, y0) = (bbox[0].min(bbox[2]), bbox[1].min(bbox[3]));
    let (w, h) = ((bbox[2] - bbox[0]).abs(), (bbox[3] - bbox[1]).abs());
    match rotate.rem_euclid(360) {
        // Tepi kiri halaman jadi tepi atas: (u, v) terlihat → (x0 + w - v, y0 + u)
        90 => VisiblePage { width: h, height: w, matrix: [0.0, 1.0, -1.0, 0.0, x0 + w, y0] },
        180 => VisiblePage { width: w, height: h, matrix: [-1.0, 0.0, 0.0, -1.0, x0 + w, y0 + h] },
        // (u, v) terlihat → (x0 + v, y0 + h - u)
        270 => VisiblePage { width: h, height: w, matrix: [0.0, -1.0, 1.0, 0.0, x0, y0 + h] },
        _ => VisiblePage { width: w, height: h, matrix: [1.0, 0.0, 0.0, 1.0, x0, y0] },
    }
}

/// Titik awal baseline teks (koordinat terlihat) untuk teks selebar `text_width`.
pub fn text_origin(page: &VisiblePage, position: Position, text_width: f32, font_size: f32, margin: f32) -> (f32, f32) {
    use Position::*;
    let x = match position {
        TopLeft | BottomLeft => margin,
        TopCenter | BottomCenter => (page.width - text_width) / 2.0,
        TopRight | BottomRight => page.width - margin - text_width,
    };
    let y = match position {
        // Kira-kira tinggi huruf kapital Helvetica (0.72 em) supaya jarak atas = margin
        TopLeft | TopCenter | TopRight => page.height - margin - font_size * 0.72,
        _ => margin,
    };
    (x, y)
}
