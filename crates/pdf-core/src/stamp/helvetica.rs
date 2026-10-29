//! Helvetica (font standar PDF, tidak perlu ditanam) dengan WinAnsiEncoding.

/// Lebar glyph ASCII 32..=126 dalam 1/1000 em (sama dengan metrik AFM / pdf.js).
const ASCII_WIDTHS: [u16; 95] = [
    278, 278, 355, 556, 556, 889, 667, 191, 333, 333, 389, 584, 278, 333, 278, 278, // spasi .. /
    556, 556, 556, 556, 556, 556, 556, 556, 556, 556, // 0..9
    278, 278, 584, 584, 584, 556, 1015, // : ; < = > ? @
    667, 667, 722, 722, 667, 611, 778, 722, 278, 500, 667, 556, 833, // A..M
    722, 778, 667, 778, 722, 667, 611, 722, 667, 944, 667, 667, 611, // N..Z
    278, 278, 278, 469, 556, 333, // [ \ ] ^ _ `
    556, 556, 500, 556, 556, 278, 556, 556, 222, 222, 500, 222, 833, // a..m
    556, 556, 556, 556, 333, 500, 278, 556, 500, 722, 500, 500, 500, // n..z
    334, 260, 334, 584, // { | } ~
];

/// Perkiraan untuk huruf Latin-1 beraksen (jarang dipakai di teks Indonesia).
const FALLBACK_WIDTH: u16 = 556;

/// Byte WinAnsi untuk teks; karakter di luar Latin-1 diganti "?".
pub fn encode(text: &str) -> Vec<u8> {
    text.chars()
        .map(|c| match c as u32 {
            code @ (32..=126 | 160..=255) => code as u8,
            _ => b'?',
        })
        .collect()
}

/// Lebar teks (point) pada ukuran huruf `size`, untuk rata tengah/kanan.
pub fn text_width(encoded: &[u8], size: f32) -> f32 {
    let units: u32 = encoded
        .iter()
        .map(|&b| match b {
            32..=126 => ASCII_WIDTHS[(b - 32) as usize],
            _ => FALLBACK_WIDTH,
        } as u32)
        .sum();
    units as f32 * size / 1000.0
}
