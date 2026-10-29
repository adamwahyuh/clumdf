//! Hitung ukuran halaman & posisi gambar. Fungsi murni supaya mudah dites.

use super::options::{Margin, PageOrientation, PageSize};

pub const PT_PER_MM: f32 = 72.0 / 25.4;
/// Mode Fit menganggap gambar 96 DPI: 1 px = 72/96 pt.
pub const PT_PER_PX: f32 = 0.75;

/// Semua nilai dalam point PDF; (x, y) dihitung dari kiri-bawah halaman.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub page_width: f32,
    pub page_height: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub fn place(
    image_px: (u32, u32),
    size: PageSize,
    orientation: PageOrientation,
    margin: Margin,
) -> Placement {
    let (img_w, img_h) = (image_px.0.max(1) as f32, image_px.1.max(1) as f32);
    let m = margin.mm() * PT_PER_MM;

    let Some((short_mm, long_mm)) = size.portrait_mm() else {
        // Fit: halaman = gambar + margin; orientasi tidak berlaku
        let (width, height) = (img_w * PT_PER_PX, img_h * PT_PER_PX);
        return Placement { page_width: width + 2.0 * m, page_height: height + 2.0 * m, x: m, y: m, width, height };
    };

    let landscape = match orientation {
        PageOrientation::Auto => img_w > img_h,
        PageOrientation::Portrait => false,
        PageOrientation::Landscape => true,
    };
    let (short, long) = (short_mm * PT_PER_MM, long_mm * PT_PER_MM);
    let (page_width, page_height) = if landscape { (long, short) } else { (short, long) };

    // Perbesar/perkecil supaya pas di area dalam margin, rasio tetap, posisi di tengah
    let (avail_w, avail_h) = ((page_width - 2.0 * m).max(1.0), (page_height - 2.0 * m).max(1.0));
    let scale = (avail_w / img_w).min(avail_h / img_h);
    let (width, height) = (img_w * scale, img_h * scale);
    Placement {
        page_width,
        page_height,
        x: (page_width - width) / 2.0,
        y: (page_height - height) / 2.0,
        width,
        height,
    }
}
