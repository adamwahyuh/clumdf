//! Pipeline OCR dites dengan tesseract palsu (script yang mengeluarkan TSV tetap),
//! jadi tidak butuh tesseract terpasang. Tes pipeline butuh PDFIUM_DIR (lihat render.rs).
mod common;

use std::path::{Path, PathBuf};

use image::{Rgb, RgbImage};
use lopdf::Document;
use pdf_core::image_to_pdf::{ImageQuality, Margin, PageOrientation, PageSize};
use pdf_core::ocr::text_layer::place_word;
use pdf_core::ocr::tsv::{Word, parse_words};
use pdf_core::{ImageToPdfOptions, OcrTools, images_to_pdf, merge_pdfs, ocr_pdf};

use common::{temp_dir, write_pdf};

const HEADER: &str = "level\tpage_num\tblock_num\tpar_num\tline_num\tword_num\tleft\ttop\twidth\theight\tconf\ttext";

fn no_progress(_: usize, _: usize) {}

#[test]
fn parses_words_and_skips_layout_rows() {
    let tsv = format!(
        "{HEADER}\n1\t1\t0\t0\t0\t0\t0\t0\t2480\t3508\t-1\t\n\
         5\t1\t1\t1\t1\t1\t300\t400\t600\t100\t96.5\tHalo\n\
         5\t1\t1\t1\t1\t2\t950\t400\t10\t100\t-1\t \n\
         5\t1\t1\t1\t1\t3\t1000\t400\t900\t100\t91\tIndonesia\n"
    );
    let words = parse_words(&tsv);
    assert_eq!(words.iter().map(|w| w.text.as_str()).collect::<Vec<_>>(), ["Halo", "Indonesia"]);
    assert_eq!((words[0].left, words[0].top, words[0].width, words[0].height), (300, 400, 600, 100));
}

#[test]
fn places_word_in_page_points_with_matching_width() {
    let word = Word { text: "Hal".into(), left: 600, top: 1000, width: 300, height: 100 };
    // Gambar 1200x1600 px untuk halaman 600x800 pt → skala 0.5
    let placed = place_word(&word, (1200, 1600), (600.0, 800.0));
    assert_eq!((placed.x, placed.y, placed.font_size), (300.0, 800.0 - 550.0, 50.0));
    // "Hal" pada 50 pt = 75 pt alami; lebar kata 150 pt → skala 200%
    assert!((placed.horizontal_scale - 200.0).abs() < 0.01);
}

// ---------- pipeline lengkap ----------

fn pdfium_dir() -> Option<PathBuf> {
    let dir = std::env::var_os("PDFIUM_DIR").map(PathBuf::from);
    if dir.is_none() {
        eprintln!("dilewati: PDFIUM_DIR belum diset");
    }
    dir.map(|d| if d.is_absolute() { d } else { PathBuf::from("../..").join(d) })
}

/// Script pengganti tesseract: abaikan gambar di stdin, keluarkan TSV tetap.
#[cfg(unix)]
fn fake_tesseract(dir: &Path, body: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let path = dir.join("tesseract");
    std::fs::write(&path, format!("#!/bin/sh\ncat > /dev/null\n{body}\n")).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    path
}

/// PDF 2 halaman: halaman 1 hasil "scan" (gambar tanpa teks), halaman 2 sudah berteks.
fn scanned_plus_digital(dir: &Path) -> PathBuf {
    let photo = dir.join("scan.png");
    RgbImage::from_pixel(400, 560, Rgb([240, 240, 240])).save(&photo).unwrap();
    let scanned = dir.join("scan.pdf");
    let options = ImageToPdfOptions {
        page_size: PageSize::A4,
        orientation: PageOrientation::Auto,
        margin: Margin::None,
        quality: ImageQuality::Medium,
        scan: false,
        compress: None,
    };
    images_to_pdf(&[photo], &scanned, &options, &no_progress).unwrap();
    let digital = write_pdf(dir, "teks-digital-yang-sudah-ada", 1);
    let both = dir.join("campur.pdf");
    merge_pdfs(&[scanned, digital], &both, &no_progress).unwrap();
    both
}

#[cfg(unix)]
#[test]
fn adds_invisible_text_only_to_pages_without_text() {
    let Some(pdfium_dir) = pdfium_dir() else { return };
    let dir = temp_dir("ocr-pipeline");
    let input = scanned_plus_digital(&dir);
    let tsv = format!(
        "{HEADER}\n5\t1\t1\t1\t1\t1\t300\t400\t600\t100\t96\tHalo\n5\t1\t1\t1\t1\t2\t1000\t400\t900\t100\t91\tIndonesia"
    );
    let tesseract = fake_tesseract(&dir, &format!("printf '%s\\n' '{}'", tsv.replace('\n', "' '")));
    let tools = OcrTools { pdfium_dir: &pdfium_dir, tesseract: &tesseract, tessdata_dir: &dir };
    let out = dir.join("out.pdf");

    let report = ocr_pdf(&tools, &input, &out, &no_progress).unwrap();

    assert_eq!((report.pages_recognized, report.pages_skipped, report.words), (1, 1, 2));
    let doc = Document::load(&out).unwrap();
    let pages: Vec<String> =
        doc.get_pages().values().map(|&id| String::from_utf8_lossy(&doc.get_page_content(id)).into_owned()).collect();
    assert!(pages[0].contains("BT 3 Tr"), "teks harus tak terlihat (mode render 3)");
    assert!(pages[0].contains("<48616C6F> Tj"), "kata 'Halo' harus ada");
    assert!(!pages[1].contains("3 Tr"), "halaman yang sudah berteks dilewati");
}

#[cfg(unix)]
#[test]
fn tesseract_errors_are_reported() {
    let Some(pdfium_dir) = pdfium_dir() else { return };
    let dir = temp_dir("ocr-errors");
    let input = scanned_plus_digital(&dir);
    let out = dir.join("out.pdf");

    let broken = fake_tesseract(&dir, "echo \"Failed loading language 'ind'\" >&2; exit 1");
    let tools = OcrTools { pdfium_dir: &pdfium_dir, tesseract: &broken, tessdata_dir: &dir };
    assert_eq!(ocr_pdf(&tools, &input, &out, &no_progress).unwrap_err().code(), "ocr_failed");

    let missing = dir.join("tidak-ada");
    let tools = OcrTools { pdfium_dir: &pdfium_dir, tesseract: &missing, tessdata_dir: &dir };
    assert_eq!(ocr_pdf(&tools, &input, &out, &no_progress).unwrap_err().code(), "ocr_missing");
    assert!(!out.exists());
}
