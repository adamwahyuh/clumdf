//! Menyusun blok Markdown ke halaman: pindah halaman, daftar, kutipan, kode, gambar.

mod elements;
pub mod lines;
mod table;

use crate::fonts::{self, Font};
use crate::markdown::blocks::{Block, Span, Style};
use lines::{LINE_SPACING, Line, break_lines};

pub const TEXT: [f32; 3] = [0.1, 0.1, 0.1];
const MUTED: [f32; 3] = [0.4, 0.4, 0.4];
const LINK: [f32; 3] = [0.04, 0.36, 0.85];
const CODE_BG: [f32; 3] = [0.95, 0.95, 0.94];
const RULE: [f32; 3] = [0.8, 0.8, 0.8];

#[derive(Debug, Clone)]
pub struct PageSetup {
    pub width: f32,
    pub height: f32,
    pub margin: f32,
    pub font_size: f32,
    pub serif: bool,
    pub page_numbers: bool,
}

/// Operasi gambar dalam koordinat PDF (point, asal kiri-bawah).
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Text { x: f32, y: f32, font: Font, size: f32, bytes: Vec<u8>, color: [f32; 3] },
    Line { x1: f32, y1: f32, x2: f32, y2: f32, width: f32, color: [f32; 3] },
    Rect { x: f32, y: f32, w: f32, h: f32, color: [f32; 3] },
    Image { index: usize, x: f32, y: f32, w: f32, h: f32 },
    Link { x: f32, y: f32, w: f32, h: f32, uri: String },
}

#[derive(Debug, Clone, Default)]
pub struct Page {
    pub ops: Vec<Op>,
}

/// Pemuat gambar; mengembalikan (indeks, lebar px, tinggi px) atau `None` bila gagal.
pub trait ImageSource {
    fn load(&mut self, src: &str) -> Option<(usize, u32, u32)>;
}

/// Posisi & gaya untuk blok yang sedang disusun (berubah di dalam daftar/kutipan).
#[derive(Clone)]
struct Ctx {
    x: f32,
    width: f32,
    color: [f32; 3],
    /// Posisi x garis kiri kutipan (bisa bersarang)
    quote_bars: Vec<f32>,
    /// Jarak setelah blok; lebih rapat di dalam daftar
    gap: f32,
    depth: usize,
}

pub(crate) struct Layouter<'a> {
    s: &'a PageSetup,
    pages: Vec<Page>,
    /// Kursor dari tepi atas halaman
    y: f32,
    images: &'a mut dyn ImageSource,
}

pub fn layout(blocks: &[Block], setup: &PageSetup, images: &mut dyn ImageSource) -> Vec<Page> {
    let mut l = Layouter { s: setup, pages: Vec::new(), y: 0.0, images };
    l.new_page();
    let ctx = Ctx {
        x: setup.margin,
        width: setup.width - 2.0 * setup.margin,
        color: TEXT,
        quote_bars: Vec::new(),
        gap: setup.font_size * 0.7,
        depth: 0,
    };
    l.blocks(blocks, &ctx);

    if setup.page_numbers {
        let size = setup.font_size * 0.8;
        for (i, page) in l.pages.iter_mut().enumerate() {
            let bytes = fonts::encode(&(i + 1).to_string());
            let x = (setup.width - Font::Helvetica.text_width(&bytes, size)) / 2.0;
            page.ops.push(Op::Text { x, y: setup.margin / 2.0, font: Font::Helvetica, size, bytes, color: MUTED });
        }
    }
    l.pages
}

impl Layouter<'_> {
    fn new_page(&mut self) {
        self.pages.push(Page::default());
        self.y = self.s.margin;
    }

    fn bottom(&self) -> f32 {
        self.s.height - self.s.margin
    }

    fn at_page_top(&self) -> bool {
        self.y <= self.s.margin + 0.01
    }

    fn op(&mut self, op: Op) {
        self.pages.last_mut().expect("selalu ada halaman").ops.push(op);
    }

    /// Koordinat PDF untuk jarak `y_top` dari tepi atas.
    fn pdf_y(&self, y_top: f32) -> f32 {
        self.s.height - y_top
    }

    /// Pindah halaman bila tinggi `h` tidak muat lagi.
    fn ensure(&mut self, h: f32) {
        if self.y + h > self.bottom() && !self.at_page_top() {
            self.new_page();
        }
    }

    /// Turunkan kursor; garis kiri kutipan ikut digambar sepanjang jarak ini.
    fn advance(&mut self, dy: f32, ctx: &Ctx) {
        let end = (self.y + dy).min(self.bottom());
        for &x in &ctx.quote_bars {
            let (y1, y2) = (self.pdf_y(self.y), self.pdf_y(end));
            self.op(Op::Rect { x, y: y2, w: 3.0, h: y1 - y2, color: RULE });
        }
        self.y += dy;
    }

    fn blocks(&mut self, blocks: &[Block], ctx: &Ctx) {
        for block in blocks {
            self.block(block, ctx);
        }
    }

    fn block(&mut self, block: &Block, ctx: &Ctx) {
        let size = self.s.font_size;
        match block {
            Block::Paragraph(spans) => {
                self.paragraph(spans, size, ctx, 0.0);
                self.advance(ctx.gap, ctx);
            }
            Block::Heading(level, spans) => {
                let scale = [2.0, 1.6, 1.3, 1.15, 1.0, 1.0][(*level as usize).clamp(1, 6) - 1];
                let size = size * scale;
                if !self.at_page_top() {
                    self.advance(size * 0.45, ctx);
                }
                let bold: Vec<Span> = spans.iter().map(|s| Span { style: Style { bold: true, ..s.style.clone() }, ..s.clone() }).collect();
                // Jangan biarkan judul sendirian di dasar halaman
                self.paragraph(&bold, size, ctx, self.s.font_size * LINE_SPACING * 2.0);
                self.advance(size * 0.35, ctx);
            }
            Block::List { start, items } => self.list(*start, items, ctx),
            Block::Quote(children) => {
                let mut inner = ctx.clone();
                inner.quote_bars.push(ctx.x + 2.0);
                inner.x += 16.0;
                inner.width -= 16.0;
                inner.color = MUTED;
                self.blocks(children, &inner);
            }
            Block::Code(code) => self.code(code, ctx),
            Block::Rule => {
                self.ensure(size * 1.5);
                let y = self.pdf_y(self.y + size * 0.75);
                self.op(Op::Line { x1: ctx.x, y1: y, x2: ctx.x + ctx.width, y2: y, width: 0.75, color: RULE });
                self.advance(size * 1.5, ctx);
            }
            Block::PageBreak => {
                if !self.at_page_top() {
                    self.new_page();
                }
            }
            Block::Image { src, alt } => self.image(src, alt, ctx),
            Block::Table { header, rows } => table::table(self, header, rows, ctx),
        }
    }

    /// Susun teks per baris; `keep_with_next` = ruang ekstra yang harus muat di bawah baris pertama.
    fn paragraph(&mut self, spans: &[Span], size: f32, ctx: &Ctx, keep_with_next: f32) {
        for (i, line) in break_lines(spans, ctx.width, size, self.s.serif).into_iter().enumerate() {
            self.ensure(line.height + if i == 0 { keep_with_next } else { 0.0 });
            self.line(&line, ctx.x, self.y, ctx.color);
            self.advance(line.height, ctx);
        }
    }

    /// Gambar satu baris dengan tepi atas di `y_top`.
    pub(crate) fn line(&mut self, line: &Line, x: f32, y_top: f32, color: [f32; 3]) {
        let baseline = self.pdf_y(y_top + line.height * 0.75);
        for (dx, p) in &line.pieces {
            let px = x + dx;
            if p.style.code && p.bytes != b" " {
                self.op(Op::Rect { x: px - 1.0, y: baseline - p.size * 0.25, w: p.width + 2.0, h: p.size * 1.1, color: CODE_BG });
            }
            let color = if p.style.link.is_some() { LINK } else { color };
            if p.style.strike {
                let y = baseline + p.size * 0.3;
                self.op(Op::Line { x1: px, y1: y, x2: px + p.width, y2: y, width: p.size * 0.06, color });
            }
            if let Some(uri) = &p.style.link {
                let y = baseline - p.size * 0.12;
                self.op(Op::Line { x1: px, y1: y, x2: px + p.width, y2: y, width: p.size * 0.05, color });
                let (h, w) = (line.height, p.width);
                self.op(Op::Link { x: px, y: baseline - p.size * 0.25, w, h: h * 0.8, uri: uri.clone() });
            }
            self.op(Op::Text { x: px, y: baseline, font: p.font, size: p.size, bytes: p.bytes.clone(), color });
        }
    }
}
