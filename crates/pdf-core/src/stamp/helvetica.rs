//! Helvetica dengan WinAnsiEncoding; tipis di atas modul `fonts`.

use crate::fonts::Font;

pub use crate::fonts::encode;

/// Lebar teks (point) pada ukuran huruf `size`, untuk rata tengah/kanan.
pub fn text_width(encoded: &[u8], size: f32) -> f32 {
    Font::Helvetica.text_width(encoded, size)
}
