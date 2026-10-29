mod common;

use image::{Rgb, RgbImage, Rgba, RgbaImage};
use lopdf::Document;
use pdf_core::{Placement, place_signature, prepare_signature};

use common::{images_in, page_labels, temp_dir, write_pdf};

/// Foto tanda tangan di kertas putih: goresan hitam 60x20 di tengah kanvas 200x100.
fn paper_signature(dir: &std::path::Path) -> std::path::PathBuf {
    let path = dir.join("ttd.jpg");
    RgbImage::from_fn(200, 100, |x, y| if (70..130).contains(&x) && (40..60).contains(&y) { Rgb([20, 20, 30]) } else { Rgb([250, 250, 248]) })
        .save(&path)
        .unwrap();
    path
}

#[test]
fn paper_photo_becomes_trimmed_transparent_png() {
    let dir = temp_dir("sig-paper");
    let dest = dir.join("signature.png");

    let info = prepare_signature(&paper_signature(&dir), &dest).unwrap();

    // JPEG bisa sedikit melebar di tepi goresan; potongannya harus di sekitar 60x20
    assert!((60..=64).contains(&info.width) && (20..=24).contains(&info.height), "{}x{}", info.width, info.height);
    let png = image::open(&dest).unwrap().to_rgba8();
    assert_eq!(png.dimensions(), (info.width, info.height));
    let center = png.get_pixel(info.width / 2, info.height / 2).0;
    assert_eq!(center[3], 255, "goresan tetap pekat");
}

#[test]
fn transparent_png_keeps_alpha_and_is_trimmed() {
    let dir = temp_dir("sig-png");
    let source = dir.join("ttd.png");
    RgbaImage::from_fn(100, 100, |x, y| if (10..30).contains(&x) && (50..60).contains(&y) { Rgba([0, 0, 200, 255]) } else { Rgba([0, 0, 0, 0]) })
        .save(&source)
        .unwrap();

    let info = prepare_signature(&source, &dir.join("signature.png")).unwrap();

    assert_eq!((info.width, info.height), (20, 10));
}

#[test]
fn blank_image_is_rejected() {
    let dir = temp_dir("sig-blank");
    let source = dir.join("kosong.png");
    RgbImage::from_pixel(50, 50, Rgb([255, 255, 255])).save(&source).unwrap();
    assert_eq!(prepare_signature(&source, &dir.join("s.png")).unwrap_err().code(), "invalid_image");
}

fn placement(page: u32, x: f32, y: f32, width: f32, height: f32) -> Placement {
    Placement { page, x, y, width, height }
}

#[test]
fn places_transparent_signature_on_chosen_pages() {
    let dir = temp_dir("sig-place");
    let input = write_pdf(&dir, "surat", 3);
    let signature = dir.join("signature.png");
    prepare_signature(&paper_signature(&dir), &signature).unwrap();
    let out = dir.join("out.pdf");

    let report = place_signature(
        &input,
        &out,
        &signature,
        &[placement(1, 0.5, 0.5, 0.25, 0.1), placement(2, 0.1, 0.8, 0.2, 0.1), placement(2, 0.6, 0.8, 0.2, 0.1)],
    )
    .unwrap();

    assert_eq!(report.signed_pages, 2);
    let doc = Document::load(&out).unwrap();
    let contents: Vec<String> =
        doc.get_pages().values().map(|&id| String::from_utf8_lossy(&doc.get_page_content(id)).into_owned()).collect();
    // A4 595x842: lebar 0.25 → 148.75, tinggi 0.1 → 84.2, x 0.5 → 297.5, y dari bawah = 842 - 0.6*842
    assert!(contents[0].contains("148.75 0 0 84.20 297.50 336.80 cm /BerkasSig Do"), "{}", contents[0]);
    assert_eq!(contents[1].matches("/BerkasSig Do").count(), 2);
    assert!(!contents[2].contains("BerkasSig"));
    assert!(images_in(&out).iter().any(|img| img.dict.has(b"SMask")), "latar harus tetap transparan");
    assert_eq!(page_labels(&out), ["surat-p1", "surat-p2", "surat-p3"]);
}

#[test]
fn rejects_placements_outside_the_page() {
    let dir = temp_dir("sig-invalid");
    let input = write_pdf(&dir, "surat", 1);
    let signature = dir.join("signature.png");
    prepare_signature(&paper_signature(&dir), &signature).unwrap();

    for bad in [placement(1, 0.9, 0.5, 0.2, 0.1), placement(2, 0.1, 0.1, 0.2, 0.1), placement(1, 0.1, 0.1, 0.0, 0.1)] {
        let err = place_signature(&input, &dir.join("out.pdf"), &signature, &[bad.clone()]).unwrap_err();
        assert_eq!(err.code(), "invalid_range", "{bad:?}");
    }
}
