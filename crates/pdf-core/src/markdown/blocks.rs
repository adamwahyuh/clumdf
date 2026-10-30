//! Markdown → model blok sederhana yang mudah ditata letak.

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Penanda pemisah halaman; komentar HTML supaya tak terlihat di penampil Markdown lain.
pub const PAGE_BREAK: &str = "<!-- pagebreak -->";

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub code: bool,
    pub strike: bool,
    pub link: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub text: String,
    pub style: Style,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Heading(u8, Vec<Span>),
    Paragraph(Vec<Span>),
    /// `start` = nomor awal untuk daftar bernomor, `None` untuk bullet
    List { start: Option<u64>, items: Vec<Vec<Block>> },
    Quote(Vec<Block>),
    Code(String),
    Rule,
    PageBreak,
    Image { src: String, alt: String },
    Table { header: Vec<Vec<Span>>, rows: Vec<Vec<Vec<Span>>> },
}

/// Wadah blok yang sedang dibangun (akar, kutipan, atau butir daftar).
enum Frame {
    Root(Vec<Block>),
    Quote(Vec<Block>),
    List { start: Option<u64>, items: Vec<Vec<Block>> },
    Item(Vec<Block>),
}

#[derive(Default)]
struct Builder {
    frames: Vec<Frame>,
    /// Teks inline yang sedang dikumpulkan (paragraf, judul, sel tabel, atau alt gambar)
    spans: Vec<Span>,
    style: Style,
    heading: Option<u8>,
    code: Option<String>,
    image: Option<String>,
    table: Option<(Vec<Vec<Span>>, Vec<Vec<Vec<Span>>>)>,
    row: Vec<Vec<Span>>,
}

impl Builder {
    fn push(&mut self, block: Block) {
        match self.frames.last_mut() {
            Some(Frame::Root(b) | Frame::Quote(b) | Frame::Item(b)) => b.push(block),
            // Teks langsung di dalam daftar tanpa butir tidak mungkin terjadi di CommonMark
            Some(Frame::List { .. }) | None => {}
        }
    }

    /// Tutup teks inline yang menggantung (mis. butir daftar "rapat" tanpa paragraf).
    fn flush(&mut self) {
        if self.spans.iter().any(|s| !s.text.trim().is_empty()) {
            let spans = std::mem::take(&mut self.spans);
            self.push(Block::Paragraph(spans));
        }
        self.spans.clear();
    }

    fn text(&mut self, text: &str) {
        if let Some(code) = &mut self.code {
            code.push_str(text);
        } else {
            self.spans.push(Span { text: text.to_string(), style: self.style.clone() });
        }
    }

    fn start(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => self.flush(),
            Tag::Heading { level, .. } => {
                self.flush();
                self.heading = Some(heading_level(level));
            }
            Tag::BlockQuote(_) => {
                self.flush();
                self.frames.push(Frame::Quote(Vec::new()));
            }
            Tag::List(start) => {
                self.flush();
                self.frames.push(Frame::List { start, items: Vec::new() });
            }
            Tag::Item => self.frames.push(Frame::Item(Vec::new())),
            Tag::CodeBlock(_) => {
                self.flush();
                self.code = Some(String::new());
            }
            Tag::Table(_) => {
                self.flush();
                self.table = Some((Vec::new(), Vec::new()));
            }
            Tag::TableCell => self.spans.clear(),
            Tag::Emphasis => self.style.italic = true,
            Tag::Strong => self.style.bold = true,
            Tag::Strikethrough => self.style.strike = true,
            Tag::Link { dest_url, .. } => self.style.link = Some(dest_url.to_string()),
            Tag::Image { dest_url, .. } => {
                // Gambar diangkat jadi blok sendiri; teks sebelumnya jadi paragraf
                self.flush();
                self.image = Some(dest_url.to_string());
            }
            _ => {}
        }
    }

    fn end(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => self.flush(),
            TagEnd::Heading(_) => {
                let (level, spans) = (self.heading.take().unwrap_or(1), std::mem::take(&mut self.spans));
                self.push(Block::Heading(level, spans));
            }
            TagEnd::BlockQuote(_) => {
                self.flush();
                if let Some(Frame::Quote(blocks)) = self.frames.pop() {
                    self.push(Block::Quote(blocks));
                }
            }
            TagEnd::Item => {
                self.flush();
                if let (Some(Frame::Item(blocks)), Some(Frame::List { items, .. })) = (self.frames.pop(), self.frames.last_mut()) {
                    items.push(blocks);
                }
            }
            TagEnd::List(_) => {
                if let Some(Frame::List { start, items }) = self.frames.pop() {
                    self.push(Block::List { start, items });
                }
            }
            TagEnd::CodeBlock => {
                let code = self.code.take().unwrap_or_default();
                self.push(Block::Code(code.trim_end_matches('\n').to_string()));
            }
            TagEnd::TableCell => self.row.push(std::mem::take(&mut self.spans)),
            TagEnd::TableHead => {
                if let Some((header, _)) = &mut self.table {
                    *header = std::mem::take(&mut self.row);
                }
            }
            TagEnd::TableRow => {
                if let Some((_, rows)) = &mut self.table {
                    rows.push(std::mem::take(&mut self.row));
                }
            }
            TagEnd::Table => {
                if let Some((header, rows)) = self.table.take() {
                    self.push(Block::Table { header, rows });
                }
            }
            TagEnd::Emphasis => self.style.italic = false,
            TagEnd::Strong => self.style.bold = false,
            TagEnd::Strikethrough => self.style.strike = false,
            TagEnd::Link => self.style.link = None,
            TagEnd::Image => {
                let alt: String = std::mem::take(&mut self.spans).into_iter().map(|s| s.text).collect();
                if let Some(src) = self.image.take() {
                    self.push(Block::Image { src, alt });
                }
            }
            _ => {}
        }
    }
}

fn heading_level(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

pub fn parse(markdown: &str) -> Vec<Block> {
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    let mut b = Builder { frames: vec![Frame::Root(Vec::new())], ..Default::default() };

    for event in Parser::new_ext(markdown, options) {
        match event {
            Event::Start(tag) => b.start(tag),
            Event::End(tag) => b.end(tag),
            Event::Text(text) => b.text(&text),
            Event::Code(code) => {
                let style = Style { code: true, ..b.style.clone() };
                b.spans.push(Span { text: code.to_string(), style });
            }
            // Satu Enter = baris baru (seperti di GitHub/Word), bukan spasi seperti CommonMark murni:
            // lebih sesuai harapan user untuk surat, alamat, dan tanda tangan
            Event::SoftBreak => b.text("\n"),
            Event::HardBreak => b.text("\n"),
            Event::Rule => {
                b.flush();
                b.push(Block::Rule);
            }
            Event::Html(html) | Event::InlineHtml(html) if html.trim() == PAGE_BREAK => {
                b.flush();
                b.push(Block::PageBreak);
            }
            _ => {}
        }
    }
    b.flush();
    match b.frames.into_iter().next() {
        Some(Frame::Root(blocks)) => blocks,
        _ => Vec::new(),
    }
}
