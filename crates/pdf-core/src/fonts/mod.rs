//! Font standar PDF (base-14): tidak perlu ditanam, cukup lebar hurufnya untuk tata letak.

mod metrics;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Font {
    Helvetica,
    HelveticaBold,
    HelveticaOblique,
    HelveticaBoldOblique,
    TimesRoman,
    TimesBold,
    TimesItalic,
    TimesBoldItalic,
    Courier,
}

impl Font {
    pub fn base_name(self) -> &'static str {
        match self {
            Font::Helvetica => "Helvetica",
            Font::HelveticaBold => "Helvetica-Bold",
            Font::HelveticaOblique => "Helvetica-Oblique",
            Font::HelveticaBoldOblique => "Helvetica-BoldOblique",
            Font::TimesRoman => "Times-Roman",
            Font::TimesBold => "Times-Bold",
            Font::TimesItalic => "Times-Italic",
            Font::TimesBoldItalic => "Times-BoldItalic",
            Font::Courier => "Courier",
        }
    }

    fn widths(self) -> Option<&'static [u16; 256]> {
        Some(match self {
            Font::Helvetica => &metrics::HELVETICA,
            Font::HelveticaBold => &metrics::HELVETICA_BOLD,
            Font::HelveticaOblique => &metrics::HELVETICA_OBLIQUE,
            Font::HelveticaBoldOblique => &metrics::HELVETICA_BOLD_OBLIQUE,
            Font::TimesRoman => &metrics::TIMES_ROMAN,
            Font::TimesBold => &metrics::TIMES_BOLD,
            Font::TimesItalic => &metrics::TIMES_ITALIC,
            Font::TimesBoldItalic => &metrics::TIMES_BOLD_ITALIC,
            Font::Courier => return None, // monospace: semua 600
        })
    }

    /// Lebar teks ter-encode (point) pada ukuran `size`.
    pub fn text_width(self, encoded: &[u8], size: f32) -> f32 {
        let units: u32 = match self.widths() {
            Some(table) => encoded.iter().map(|&b| table[b as usize] as u32).sum(),
            None => encoded.len() as u32 * 600,
        };
        units as f32 * size / 1000.0
    }
}

/// Byte WinAnsiEncoding untuk teks; tanda baca tipografis (“ ” – — • … €) ikut
/// dipetakan, karakter lain di luar Latin-1 diganti "?".
pub fn encode(text: &str) -> Vec<u8> {
    text.chars()
        .map(|c| match c as u32 {
            code @ (32..=126 | 160..=255) => code as u8,
            _ => match c {
                '€' => 0x80,
                '‚' => 0x82,
                '„' => 0x84,
                '…' => 0x85,
                '‘' => 0x91,
                '’' => 0x92,
                '“' => 0x93,
                '”' => 0x94,
                '•' => 0x95,
                '–' => 0x96,
                '—' => 0x97,
                '™' => 0x99,
                '\t' => b' ',
                _ => b'?',
            },
        })
        .collect()
}
