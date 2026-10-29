mod common;

use lopdf::Document;
use pdf_core::stamp::helvetica::{encode, text_width};
use pdf_core::stamp::layout::{Position, text_origin, visible_page};
use pdf_core::{StampOptions, TextStamp, stamp_text};

use common::{build_pdf, page_labels, temp_dir, write_pdf};

fn apply(m: [f32; 6], (u, v): (f32, f32)) -> (f32, f32) {
    (m[0] * u + m[2] * v + m[4], m[1] * u + m[3] * v + m[5])
}

// ---------- fungsi murni ----------

#[test]
fn helvetica_widths_and_encoding() {
    assert_eq!(text_width(&encode("Hal"), 10.0), 15.0); // H 722 + a 556 + l 222
    assert_eq!(encode("é😀"), [0xE9, b'?']);
}

#[test]
fn visible_page_maps_corners_for_each_rotation() {
    let bbox = [10.0, 20.0, 610.0, 820.0]; // 600 x 800, tidak di titik nol
    // r = 0: kiri-bawah terlihat = kiri-bawah halaman
    let p = visible_page(bbox, 0);
    assert_eq!((p.width, p.height), (600.0, 800.0));
    assert_eq!(apply(p.matrix, (0.0, 0.0)), (10.0, 20.0));
    // r = 90: tampil landscape; kiri-bawah terlihat = kanan-bawah halaman
    let p = visible_page(bbox, 90);
    assert_eq!((p.width, p.height), (800.0, 600.0));
    assert_eq!(apply(p.matrix, (0.0, 0.0)), (610.0, 20.0));
    assert_eq!(apply(p.matrix, (800.0, 600.0)), (10.0, 820.0));
    // r = 180: kiri-bawah terlihat = kanan-atas halaman
    assert_eq!(apply(visible_page(bbox, 180).matrix, (0.0, 0.0)), (610.0, 820.0));
    // r = 270 (= -90): kiri-bawah terlihat = kiri-atas halaman
    assert_eq!(apply(visible_page(bbox, -90).matrix, (0.0, 0.0)), (10.0, 820.0));
}

#[test]
fn text_origin_aligns_left_center_right() {
    let p = visible_page([0.0, 0.0, 600.0, 800.0], 0);
    assert_eq!(text_origin(&p, Position::BottomLeft, 100.0, 10.0, 30.0), (30.0, 30.0));
    assert_eq!(text_origin(&p, Position::BottomCenter, 100.0, 10.0, 30.0), (250.0, 30.0));
    assert_eq!(text_origin(&p, Position::TopRight, 100.0, 10.0, 30.0), (470.0, 800.0 - 30.0 - 7.2));
}

// ---------- stamp_text ----------

fn options(text: &str, skip_first: bool) -> StampOptions {
    StampOptions {
        stamps: vec![TextStamp { text: text.into(), position: Position::BottomCenter }],
        font_size: 11.0,
        margin_mm: 12.0,
        first_number: 1,
        skip_first,
    }
}

fn hex(text: &str) -> String {
    text.bytes().map(|b| format!("{b:02X}")).collect()
}

fn page_contents(path: &std::path::Path) -> Vec<String> {
    let doc = Document::load(path).unwrap();
    doc.get_pages().values().map(|&id| String::from_utf8_lossy(&doc.get_page_content(id)).into_owned()).collect()
}

#[test]
fn numbers_every_page_and_keeps_original_content() {
    let dir = temp_dir("stamp-numbers");
    let input = write_pdf(&dir, "isi", 3);
    let out = dir.join("out.pdf");

    stamp_text(&input, &out, &options("Halaman {page} dari {total}", false)).unwrap();

    let contents = page_contents(&out);
    for (i, content) in contents.iter().enumerate() {
        assert!(content.contains(&hex(&format!("Halaman {} dari 3", i + 1))), "halaman {}", i + 1);
        assert!(content.starts_with("q\n"), "konten asli harus dibungkus q … Q");
    }
    assert_eq!(page_labels(&out), ["isi-p1", "isi-p2", "isi-p3"]);
}

#[test]
fn skip_first_page_starts_numbering_on_page_two() {
    let dir = temp_dir("stamp-skip");
    let input = write_pdf(&dir, "isi", 3);
    let out = dir.join("out.pdf");

    stamp_text(&input, &out, &options("{page} / {total}", true)).unwrap();

    let contents = page_contents(&out);
    assert!(!contents[0].contains("Tj ET Q"), "sampul tidak diberi nomor");
    assert!(contents[1].contains(&hex("1 / 2")));
    assert!(contents[2].contains(&hex("2 / 2")));
}

#[test]
fn rotated_pages_get_upright_text() {
    let dir = temp_dir("stamp-rotated");
    let input = dir.join("in.pdf");
    let mut doc = build_pdf("isi", 1);
    let page = doc.get_pages()[&1];
    doc.get_dictionary_mut(page).unwrap().set("Rotate", 90);
    doc.save(&input).unwrap();
    let out = dir.join("out.pdf");

    stamp_text(&input, &out, &options("{page}", false)).unwrap();

    assert!(page_contents(&out)[0].contains("q 0 1 -1 0 595 0 cm"));
}

#[test]
fn empty_text_is_rejected() {
    let dir = temp_dir("stamp-empty");
    let input = write_pdf(&dir, "isi", 1);
    assert_eq!(stamp_text(&input, &dir.join("out.pdf"), &options("  ", false)).unwrap_err().code(), "no_input");
}
