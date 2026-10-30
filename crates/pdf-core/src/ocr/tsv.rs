//! Membaca keluaran `tesseract … tsv`: satu baris per elemen, level 5 = kata.

/// Satu kata hasil OCR, kotak dalam piksel gambar (asal kiri-atas).
#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub text: String,
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

/// Kolom: level page block par line word left top width height conf text.
/// Kata kosong atau dengan keyakinan < 0 (penanda bukan-kata) dilewati.
pub fn parse_words(tsv: &str) -> Vec<Word> {
    tsv.lines()
        .skip(1) // baris judul kolom
        .filter_map(|line| {
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 12 || cols[0] != "5" {
                return None;
            }
            let number = |i: usize| cols[i].trim().parse::<u32>().ok();
            let confidence: f32 = cols[10].trim().parse().ok()?;
            let text = cols[11..].join("\t").trim().to_string();
            if text.is_empty() || confidence < 0.0 {
                return None;
            }
            Some(Word { text, left: number(6)?, top: number(7)?, width: number(8)?, height: number(9)? })
        })
        .filter(|w| w.width > 0 && w.height > 0)
        .collect()
}
