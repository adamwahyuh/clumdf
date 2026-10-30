mod common;

use image::{Rgb, RgbImage};
use lopdf::Document;
use pdf_core::image_to_pdf::PageSize;
use pdf_core::markdown::blocks::{Block, Span, Style, parse};
use pdf_core::markdown::layout::lines::break_lines;
use pdf_core::markdown::{FontFamily, MarkdownOptions};
use pdf_core::markdown_to_pdf;

use common::temp_dir;

fn text(spans: &[Span]) -> String {
    spans.iter().map(|s| s.text.as_str()).collect()
}

#[test]
fn parses_common_blocks() {
    let md = "# Judul\n\nTeks **tebal** dan *miring*.\n\n- satu\n- dua\n  1. sub\n\n> kutip\n\n```\nkode\n```\n\n---\n\n<!-- pagebreak -->\n\n| A | B |\n|---|---|\n| 1 | 2 |\n";
    let blocks = parse(md);

    assert!(matches!(&blocks[0], Block::Heading(1, s) if text(s) == "Judul"));
    let Block::Paragraph(p) = &blocks[1] else { panic!("{:?}", blocks[1]) };
    assert_eq!(p[1], Span { text: "tebal".into(), style: Style { bold: true, ..Style::default() } });
    assert!(p[3].style.italic);
    let Block::List { start: None, items } = &blocks[2] else { panic!("{:?}", blocks[2]) };
    assert_eq!(items.len(), 2);
    assert!(matches!(&items[1][1], Block::List { start: Some(1), .. }), "daftar bersarang di butir kedua");
    assert!(matches!(&blocks[3], Block::Quote(q) if matches!(&q[0], Block::Paragraph(_))));
    assert_eq!(blocks[4], Block::Code("kode".into()));
    assert_eq!(blocks[5], Block::Rule);
    assert_eq!(blocks[6], Block::PageBreak);
    let Block::Table { header, rows } = &blocks[7] else { panic!("{:?}", blocks[7]) };
    assert_eq!((header.len(), rows.len(), text(&rows[0][1])), (2, 1, "2".into()));
}

#[test]
fn single_newline_is_a_line_break() {
    let blocks = parse("Yth. Bapak/Ibu HRD\nDi tempat");
    assert!(matches!(&blocks[0], Block::Paragraph(p) if text(p) == "Yth. Bapak/Ibu HRD\nDi tempat"));
}

#[test]
fn images_and_links_are_kept() {
    let blocks = parse("Lihat [situs](https://contoh.id) ini.\n\n![logo](logo.png)");
    let Block::Paragraph(p) = &blocks[0] else { panic!() };
    assert_eq!(p[1].style.link.as_deref(), Some("https://contoh.id"));
    assert_eq!(blocks[1], Block::Image { src: "logo.png".into(), alt: "logo".into() });
}

#[test]
fn wraps_lines_within_width_and_splits_long_words() {
    let spans = [Span { text: "kata ".repeat(40), style: Style::default() }];
    let lines = break_lines(&spans, 200.0, 12.0, false);
    assert!(lines.len() > 1);
    assert!(lines.iter().all(|l| l.width <= 200.0), "{:?}", lines.iter().map(|l| l.width).collect::<Vec<_>>());

    let url = [Span { text: "x".repeat(300), style: Style::default() }];
    assert!(break_lines(&url, 100.0, 12.0, false).iter().all(|l| l.width <= 100.0));
}

fn options() -> MarkdownOptions {
    MarkdownOptions { page_size: PageSize::A4, font: FontFamily::Sans, font_size: 11.0, margin_mm: 20.0, page_numbers: true }
}

fn contents(path: &std::path::Path) -> Vec<String> {
    let doc = Document::load(path).unwrap();
    doc.get_pages().values().map(|&id| String::from_utf8_lossy(&doc.get_page_content(id)).into_owned()).collect()
}

fn hex(text: &str) -> String {
    text.bytes().map(|b| format!("{b:02X}")).collect()
}

#[test]
fn long_document_flows_onto_new_pages_with_numbers() {
    let dir = temp_dir("md-flow");
    let md = (1..=120).map(|i| format!("Paragraf nomor {i} dengan sedikit teks.\n\n")).collect::<String>();
    let out = dir.join("out.pdf");

    let report = markdown_to_pdf(&md, None, &out, &options()).unwrap();

    assert!(report.page_count >= 3, "{report:?}");
    let pages = contents(&out);
    assert_eq!(pages.len(), report.page_count);
    assert!(pages[0].contains(&hex("Paragraf")));
    assert!(pages[1].contains(&format!("<{}> Tj", hex("2"))), "nomor halaman 2");
}

#[test]
fn page_break_link_and_images_end_up_in_the_pdf() {
    let dir = temp_dir("md-features");
    RgbImage::from_pixel(400, 200, Rgb([200, 40, 40])).save(dir.join("foto.png")).unwrap();
    let md = "Halaman satu, lihat [Berkas](https://berkas.id).\n\n<!-- pagebreak -->\n\n![foto](foto.png)\n\n![hilang](tidak-ada.png)";
    let out = dir.join("out.pdf");

    let report = markdown_to_pdf(md, Some(&dir), &out, &options()).unwrap();

    assert_eq!(report.page_count, 2);
    let doc = Document::load(&out).unwrap();
    let first = doc.get_pages()[&1];
    let annots = doc.get_dictionary(first).unwrap().get(b"Annots").unwrap().as_array().unwrap().len();
    assert!(annots >= 1, "link harus bisa diklik");
    let second = &contents(&out)[1];
    assert!(second.contains("/Im0 Do"), "gambar relatif terhadap folder Markdown");
    // Teks ditulis per kata
    assert!(second.contains(&format!("<{}>", hex("[Gambar"))) && second.contains(&format!("<{}>", hex("hilang]"))));
}

#[test]
fn typographic_characters_use_winansi() {
    let dir = temp_dir("md-typo");
    let out = dir.join("out.pdf");
    markdown_to_pdf("“Kutip” – • …", None, &out, &options()).unwrap();
    let page = &contents(&out)[0];
    for word in ["<934B7574697094>", "<96>", "<95>", "<85>"] {
        assert!(page.contains(word), "{word} tidak ada di {page}");
    }
}
