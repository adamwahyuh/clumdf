//! Memecah teks berformat (span) menjadi baris yang muat di lebar tertentu.

use crate::fonts::{self, Font};
use crate::markdown::blocks::{Span, Style};

/// Potongan teks dengan satu font & gaya.
#[derive(Debug, Clone)]
pub struct Piece {
    pub bytes: Vec<u8>,
    pub font: Font,
    pub size: f32,
    pub width: f32,
    pub style: Style,
}

#[derive(Debug, Clone, Default)]
pub struct Line {
    /// (posisi x relatif awal baris, potongan)
    pub pieces: Vec<(f32, Piece)>,
    pub width: f32,
    pub height: f32,
}

/// Jarak antarbaris relatif ukuran huruf.
pub const LINE_SPACING: f32 = 1.4;

pub fn font_for(style: &Style, serif: bool) -> Font {
    match (style.code, serif, style.bold, style.italic) {
        (true, ..) => Font::Courier,
        (_, false, false, false) => Font::Helvetica,
        (_, false, true, false) => Font::HelveticaBold,
        (_, false, false, true) => Font::HelveticaOblique,
        (_, false, true, true) => Font::HelveticaBoldOblique,
        (_, true, false, false) => Font::TimesRoman,
        (_, true, true, false) => Font::TimesBold,
        (_, true, false, true) => Font::TimesItalic,
        (_, true, true, true) => Font::TimesBoldItalic,
    }
}

fn piece(text: &str, style: &Style, size: f32, serif: bool) -> Piece {
    let font = font_for(style, serif);
    // Kode inline sedikit lebih kecil supaya tidak tampak menonjol
    let size = if style.code { size * 0.9 } else { size };
    let bytes = fonts::encode(text);
    Piece { width: font.text_width(&bytes, size), bytes, font, size, style: style.clone() }
}

enum Token {
    /// Satu kata; bisa terdiri dari beberapa gaya, mis. "**ka**ta"
    Word(Vec<Piece>),
    Space(Piece),
    Newline,
}

fn tokenize(spans: &[Span], size: f32, serif: bool) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut word: Vec<Piece> = Vec::new();
    for span in spans {
        let mut current = String::new();
        let finish_piece = |current: &mut String, word: &mut Vec<Piece>| {
            if !current.is_empty() {
                word.push(piece(current, &span.style, size, serif));
                current.clear();
            }
        };
        for c in span.text.chars() {
            if c == ' ' || c == '\n' {
                finish_piece(&mut current, &mut word);
                if !word.is_empty() {
                    tokens.push(Token::Word(std::mem::take(&mut word)));
                }
                tokens.push(if c == ' ' { Token::Space(piece(" ", &span.style, size, serif)) } else { Token::Newline });
            } else {
                current.push(c);
            }
        }
        finish_piece(&mut current, &mut word);
    }
    if !word.is_empty() {
        tokens.push(Token::Word(word));
    }
    tokens
}

/// Pecah kata yang lebih lebar dari baris (mis. URL panjang) per karakter.
fn split_long(word: Vec<Piece>, max: f32) -> Vec<Vec<Piece>> {
    let mut parts: Vec<Vec<Piece>> = vec![Vec::new()];
    let mut width = 0.0;
    for p in word {
        for &byte in &p.bytes {
            let w = p.font.text_width(&[byte], p.size);
            if width + w > max && width > 0.0 {
                parts.push(Vec::new());
                width = 0.0;
            }
            let part = parts.last_mut().expect("selalu ada");
            match part.last_mut() {
                Some(last) if last.font == p.font && last.style == p.style => {
                    last.bytes.push(byte);
                    last.width += w;
                }
                _ => part.push(Piece { bytes: vec![byte], width: w, ..p.clone() }),
            }
            width += w;
        }
    }
    parts
}

/// Susun teks menjadi baris rata kiri selebar maksimal `max_width`.
pub fn break_lines(spans: &[Span], max_width: f32, size: f32, serif: bool) -> Vec<Line> {
    let mut lines = vec![Line::default()];
    let mut pending_space: Option<Piece> = None;

    for token in tokenize(spans, size, serif) {
        match token {
            Token::Newline => {
                lines.push(Line::default());
                pending_space = None;
            }
            Token::Space(space) => pending_space = Some(space),
            Token::Word(word) => {
                let word_width: f32 = word.iter().map(|p| p.width).sum();
                let words = if word_width > max_width { split_long(word, max_width) } else { vec![word] };
                for word in words {
                    let width: f32 = word.iter().map(|p| p.width).sum();
                    let line = lines.last_mut().expect("selalu ada");
                    let space = pending_space.take().filter(|_| !line.pieces.is_empty());
                    let gap = space.as_ref().map_or(0.0, |s| s.width);
                    if line.width + gap + width > max_width && !line.pieces.is_empty() {
                        lines.push(Line::default());
                    } else if let Some(space) = space {
                        // Spasi disimpan supaya garis bawah link/coret tetap menyambung
                        line.pieces.push((line.width, space));
                        line.width += gap;
                    }
                    let line = lines.last_mut().expect("selalu ada");
                    for p in word {
                        line.height = line.height.max(p.size * LINE_SPACING);
                        line.pieces.push((line.width, p.clone()));
                        line.width += p.width;
                    }
                }
            }
        }
    }
    for line in &mut lines {
        if line.height == 0.0 {
            line.height = size * LINE_SPACING; // baris kosong dari ganti baris manual
        }
    }
    lines
}
