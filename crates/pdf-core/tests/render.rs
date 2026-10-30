//! Butuh library PDFium: jalankan scripts/fetch-pdfium.sh lalu
//! `PDFIUM_DIR=src-tauri/pdfium cargo test -p pdf-core` (CI menyetelnya otomatis).
mod common;

use std::path::PathBuf;

use pdf_core::{RenderFormat, RenderOptions, pdf_to_images};

use common::{build_pdf, encrypt, temp_dir, write_pdf};

fn no_progress(_: usize, _: usize) {}

fn library_dir() -> Option<PathBuf> {
    let dir = std::env::var_os("PDFIUM_DIR").map(PathBuf::from);
    if dir.is_none() {
        eprintln!("dilewati: PDFIUM_DIR belum diset");
    }
    // Path relatif dari root repo; tes berjalan di folder crate
    dir.map(|d| if d.is_absolute() { d } else { PathBuf::from("../..").join(d) })
}

#[test]
fn renders_every_page_with_padded_names_and_requested_dpi() {
    let Some(lib) = library_dir() else { return };
    let dir = temp_dir("render-pages");
    let input = write_pdf(&dir, "surat", 12);
    let out = dir.join("out");
    std::fs::create_dir(&out).unwrap();

    let options = RenderOptions { format: RenderFormat::Png, dpi: 144 };
    let report = pdf_to_images(&lib, &input, &out, &options, &no_progress).unwrap();

    assert_eq!(report.files.len(), 12);
    assert_eq!(report.files[0], out.join("surat-01.png"));
    assert_eq!(report.files[11], out.join("surat-12.png"));
    let page = image::open(&report.files[0]).unwrap().to_luma8();
    // A4 595x842 pt × 144/72 = 1190x1684 px
    assert_eq!(page.dimensions(), (1190, 1684));
    assert!(page.pixels().any(|p| p.0[0] < 100), "teks halaman harus ikut ter-render");
}

#[test]
fn jpg_output_is_a_valid_jpeg() {
    let Some(lib) = library_dir() else { return };
    let dir = temp_dir("render-jpg");
    let input = write_pdf(&dir, "a", 1);

    let options = RenderOptions { format: RenderFormat::Jpg, dpi: 72 };
    let report = pdf_to_images(&lib, &input, &dir, &options, &no_progress).unwrap();

    assert_eq!(report.files[0], dir.join("a-1.jpg"));
    let format = image::ImageReader::open(&report.files[0]).unwrap().with_guessed_format().unwrap().format();
    assert_eq!(format, Some(image::ImageFormat::Jpeg));
}

#[test]
fn locked_pdf_is_reported_as_encrypted() {
    let Some(lib) = library_dir() else { return };
    let dir = temp_dir("render-locked");
    let input = dir.join("locked.pdf");
    let mut doc = build_pdf("x", 1);
    encrypt(&mut doc);
    doc.save(&input).unwrap();

    let options = RenderOptions { format: RenderFormat::Png, dpi: 72 };
    let err = pdf_to_images(&lib, &input, &dir, &options, &no_progress).unwrap_err();

    assert_eq!(err.code(), "encrypted");
}
