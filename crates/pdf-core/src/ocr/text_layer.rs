//! Teks tak terlihat di posisi kata hasil OCR. Fungsi murni supaya mudah dites.

use super::tsv::Word;
use crate::stamp::helvetica;

/// Posisi satu kata di halaman terlihat (point, asal kiri-bawah).
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedWord {
    pub encoded: Vec<u8>,
    pub x: f32,
    pub y: f32,
    pub font_size: f32,
    /// Skala horizontal (persen) supaya lebar teks sama dengan lebar kata di gambar
    pub horizontal_scale: f32,
}

/// Petakan kotak kata (piksel gambar `image_px`) ke halaman berukuran `page_pt`.
pub fn place_word(word: &Word, image_px: (u32, u32), page_pt: (f32, f32)) -> PlacedWord {
    let sx = page_pt.0 / image_px.0 as f32;
    let sy = page_pt.1 / image_px.1 as f32;
    let font_size = (word.height as f32 * sy).max(1.0);
    let encoded = helvetica::encode(&word.text);
    let natural = helvetica::text_width(&encoded, font_size).max(0.01);
    PlacedWord {
        x: word.left as f32 * sx,
        // Baseline ≈ tepi bawah kotak kata (abaikan ekor huruf seperti g/p)
        y: page_pt.1 - (word.top + word.height) as f32 * sy,
        horizontal_scale: (word.width as f32 * sx / natural * 100.0).clamp(10.0, 1000.0),
        font_size,
        encoded,
    }
}

/// Operator konten untuk semua kata, dengan mode render 3 (tak terlihat):
/// tampilan halaman tidak berubah, tetapi teks bisa dicari, dipilih, dan di-copy.
pub fn content(words: &[PlacedWord], font_name: &str) -> String {
    let mut out = String::from("BT 3 Tr\n");
    for w in words {
        let hex: String = w.encoded.iter().map(|b| format!("{b:02X}")).collect();
        out.push_str(&format!(
            "/{font_name} {:.2} Tf {:.1} Tz 1 0 0 1 {:.2} {:.2} Tm <{hex}> Tj\n",
            w.font_size, w.horizontal_scale, w.x, w.y
        ));
    }
    out.push_str("ET\n");
    out
}
