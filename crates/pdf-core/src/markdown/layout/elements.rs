//! Daftar, blok kode, dan gambar.

use super::lines::{self, LINE_SPACING};
use super::{CODE_BG, Ctx, Layouter, MUTED, Op, TEXT};
use crate::fonts::{self, Font};
use crate::markdown::blocks::{Block, Span, Style};

impl Layouter<'_> {
    pub(super) fn list(&mut self, start: Option<u64>, items: &[Vec<Block>], ctx: &Ctx) {
        let size = self.s.font_size;
        let indent = size * 1.6;
        let mut inner = ctx.clone();
        inner.x += indent;
        inner.width -= indent;
        inner.gap = size * 0.25;
        inner.depth += 1;

        for (i, item) in items.iter().enumerate() {
            let line_height = size * LINE_SPACING;
            self.ensure(line_height);
            let marker = match start {
                Some(n) => format!("{}.", n + i as u64),
                None if ctx.depth % 2 == 1 => "–".to_string(),
                None => "•".to_string(),
            };
            let font = lines::font_for(&Style::default(), self.s.serif);
            let bytes = fonts::encode(&marker);
            // Nomor rata kanan supaya titiknya sejajar (9. dan 10.)
            let x = inner.x - size * 0.5 - font.text_width(&bytes, size);
            let y = self.pdf_y(self.y + line_height * 0.75);
            self.op(Op::Text { x, y, font, size, bytes, color: ctx.color });
            self.blocks(item, &inner);
        }
        self.advance(ctx.gap - inner.gap, ctx);
    }

    pub(super) fn code(&mut self, code: &str, ctx: &Ctx) {
        let size = self.s.font_size * 0.88;
        let line_height = size * 1.35;
        let pad = 8.0;
        let per_line = (((ctx.width - 2.0 * pad) / (size * 0.6)) as usize).max(1);
        let rows: Vec<String> = code
            .split('\n')
            .flat_map(|row| {
                let chars: Vec<char> = row.chars().collect();
                if chars.is_empty() { vec![String::new()] } else { chars.chunks(per_line).map(|c| c.iter().collect()).collect() }
            })
            .collect();
        let count = rows.len();

        for (i, row) in rows.into_iter().enumerate() {
            // Latar abu-abu per baris supaya tetap menyambung saat pindah halaman
            let top = if i == 0 { pad } else { 0.0 };
            let bottom = if i + 1 == count { pad } else { 0.0 };
            self.ensure(top + line_height + bottom);
            let h = top + line_height + bottom;
            let y = self.pdf_y(self.y + h);
            self.op(Op::Rect { x: ctx.x, y, w: ctx.width, h, color: CODE_BG });
            let baseline = self.pdf_y(self.y + top + line_height * 0.75);
            let bytes = fonts::encode(&row);
            self.op(Op::Text { x: ctx.x + pad, y: baseline, font: Font::Courier, size, bytes, color: TEXT });
            self.advance(h, ctx);
        }
        self.advance(ctx.gap, ctx);
    }

    pub(super) fn image(&mut self, src: &str, alt: &str, ctx: &Ctx) {
        let Some((index, w_px, h_px)) = self.images.load(src) else {
            let text = format!("[Gambar tidak ditemukan: {}]", if alt.is_empty() { src } else { alt });
            let style = Style { italic: true, ..Style::default() };
            let muted = Ctx { color: MUTED, ..ctx.clone() };
            self.paragraph(&[Span { text, style }], self.s.font_size, &muted, 0.0);
            self.advance(ctx.gap, ctx);
            return;
        };
        // 96 DPI seperti tampilan layar; diperkecil bila lebih lebar/tinggi dari area tulis
        let mut w = (w_px as f32 * 0.75).min(ctx.width);
        let mut h = w * h_px as f32 / w_px.max(1) as f32;
        let max_h = self.s.height - 2.0 * self.s.margin;
        if h > max_h {
            w *= max_h / h;
            h = max_h;
        }
        self.ensure(h);
        let y = self.pdf_y(self.y + h);
        self.op(Op::Image { index, x: ctx.x, y, w, h });
        self.advance(h + ctx.gap, ctx);
    }
}
