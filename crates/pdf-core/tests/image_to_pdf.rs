mod common;

use std::path::{Path, PathBuf};

use image::codecs::jpeg::JpegEncoder;
use image::{ExtendedColorType, ImageEncoder, Rgb, RgbImage, Rgba, RgbaImage};
use lopdf::{Document, Object};
use pdf_core::image_to_pdf::layout::{PT_PER_MM, Placement, place};
use pdf_core::image_to_pdf::{ImageQuality, ImageToPdfOptions, Margin, PageOrientation, PageSize};
use pdf_core::images_to_pdf;

use common::{images_in, temp_dir};

fn no_progress(_: usize, _: usize) {}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.01
}

fn assert_placement(actual: Placement, expected: Placement) {
    let pairs = [
        (actual.page_width, expected.page_width),
        (actual.page_height, expected.page_height),
        (actual.x, expected.x),
        (actual.y, expected.y),
        (actual.width, expected.width),
        (actual.height, expected.height),
    ];
    assert!(pairs.iter().all(|&(a, b)| close(a, b)), "{actual:?} != {expected:?}");
}

// ---------- layout.rs (fungsi murni) ----------

#[test]
fn fit_mode_uses_96_dpi_and_ignores_orientation() {
    let p = place((800, 600), PageSize::Fit, PageOrientation::Portrait, Margin::None);
    assert_placement(p, Placement { page_width: 600.0, page_height: 450.0, x: 0.0, y: 0.0, width: 600.0, height: 450.0 });
}

#[test]
fn fit_mode_adds_margin_around_image() {
    let m = 10.0 * PT_PER_MM;
    let p = place((800, 600), PageSize::Fit, PageOrientation::Auto, Margin::Small);
    assert_placement(p, Placement { page_width: 600.0 + 2.0 * m, page_height: 450.0 + 2.0 * m, x: m, y: m, width: 600.0, height: 450.0 });
}

#[test]
fn auto_orientation_turns_page_for_wide_images() {
    let p = place((4000, 3000), PageSize::A4, PageOrientation::Auto, Margin::None);
    assert!(close(p.page_width, 297.0 * PT_PER_MM) && close(p.page_height, 210.0 * PT_PER_MM));
    // 4:3 di A4 landscape: tinggi penuh, lebar di tengah
    assert!(close(p.height, p.page_height) && close(p.y, 0.0));
    assert!(close(p.x, (p.page_width - p.width) / 2.0));
}

#[test]
fn forced_portrait_fits_wide_image_by_width_and_centers_it() {
    let m = 20.0 * PT_PER_MM;
    let p = place((2000, 1000), PageSize::A4, PageOrientation::Portrait, Margin::Medium);
    let page_w = 210.0 * PT_PER_MM;
    assert!(close(p.page_width, page_w) && close(p.page_height, 297.0 * PT_PER_MM));
    assert!(close(p.width, page_w - 2.0 * m));
    assert!(close(p.height, p.width / 2.0));
    assert!(close(p.y, (p.page_height - p.height) / 2.0));
}

#[test]
fn small_images_are_scaled_up_to_fill_the_page() {
    let p = place((100, 100), PageSize::Letter, PageOrientation::Portrait, Margin::None);
    assert!(close(p.width, 215.9 * PT_PER_MM));
}

#[test]
fn f4_folio_is_215_by_330_mm() {
    let p = place((10, 20), PageSize::F4, PageOrientation::Auto, Margin::None);
    assert!(close(p.page_width, 215.0 * PT_PER_MM) && close(p.page_height, 330.0 * PT_PER_MM));
}

// ---------- images_to_pdf ----------

fn options(page_size: PageSize, quality: ImageQuality) -> ImageToPdfOptions {
    ImageToPdfOptions { page_size, orientation: PageOrientation::Auto, margin: Margin::None, quality }
}

fn save_jpeg(path: &Path, img: &RgbImage, exif: Option<Vec<u8>>) {
    let mut bytes = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut bytes, 90);
    if let Some(exif) = exif {
        encoder.set_exif_metadata(exif).unwrap();
    }
    encoder.write_image(img.as_raw(), img.width(), img.height(), ExtendedColorType::Rgb8).unwrap();
    std::fs::write(path, bytes).unwrap();
}

/// EXIF minimal (TIFF little-endian) berisi Orientation = 6 (putar 90° searah jarum jam).
fn exif_rotate_90() -> Vec<u8> {
    let mut exif = b"II*\0\x08\0\0\0\x01\0".to_vec();
    exif.extend_from_slice(&[0x12, 0x01, 3, 0, 1, 0, 0, 0, 6, 0, 0, 0]);
    exif.extend_from_slice(&[0, 0, 0, 0]);
    exif
}

fn mixed_images(dir: &Path) -> Vec<PathBuf> {
    let transparent = dir.join("transparent.png");
    RgbaImage::from_fn(400, 300, |x, _| if x < 200 { Rgba([0, 0, 0, 0]) } else { Rgba([255, 0, 0, 255]) })
        .save(&transparent)
        .unwrap();
    let portrait = dir.join("portrait.jpg");
    save_jpeg(&portrait, &RgbImage::from_pixel(300, 400, Rgb([10, 120, 200])), None);
    let landscape = dir.join("landscape.png");
    RgbImage::from_pixel(600, 200, Rgb([0, 200, 0])).save(&landscape).unwrap();
    let phone = dir.join("phone.jpg");
    save_jpeg(&phone, &RgbImage::from_pixel(40, 20, Rgb([200, 200, 0])), Some(exif_rotate_90()));
    vec![transparent, portrait, landscape, phone]
}

/// (lebar, tinggi) MediaBox tiap halaman sesuai urutan.
fn page_sizes(path: &Path) -> Vec<(f32, f32)> {
    let doc = Document::load(path).unwrap();
    doc.get_pages()
        .values()
        .map(|&id| {
            let mb = doc.get_dictionary(id).unwrap().get(b"MediaBox").unwrap().as_array().unwrap().clone();
            (mb[2].as_float().unwrap(), mb[3].as_float().unwrap())
        })
        .collect()
}

#[test]
fn one_page_per_image_in_input_order_with_exif_rotation() {
    let dir = temp_dir("img2pdf-order");
    let inputs = mixed_images(&dir);
    let out = dir.join("out.pdf");

    let report = images_to_pdf(&inputs, &out, &options(PageSize::Fit, ImageQuality::Medium), &no_progress).unwrap();

    assert_eq!(report.page_count, 4);
    let sizes = page_sizes(&out);
    let expected = [(300.0, 225.0), (225.0, 300.0), (450.0, 150.0), (15.0, 30.0)]; // foto HP 40x20 → 20x40
    for (actual, expected) in sizes.iter().zip(expected) {
        assert!(close(actual.0, expected.0) && close(actual.1, expected.1), "{sizes:?}");
    }
}

#[test]
fn original_quality_embeds_jpeg_bytes_unchanged() {
    let dir = temp_dir("img2pdf-passthrough");
    let inputs = mixed_images(&dir);
    let out = dir.join("out.pdf");

    images_to_pdf(&inputs[1..2], &out, &options(PageSize::A4, ImageQuality::Original), &no_progress).unwrap();

    let images = images_in(&out);
    assert_eq!(images[0].dict.get(b"Filter").unwrap().as_name().unwrap(), b"DCTDecode");
    assert_eq!(images[0].content, std::fs::read(&inputs[1]).unwrap());
}

#[test]
fn rotated_jpeg_is_reencoded_even_at_original_quality() {
    let dir = temp_dir("img2pdf-rotated");
    let inputs = mixed_images(&dir);
    let out = dir.join("out.pdf");

    images_to_pdf(&inputs[3..4], &out, &options(PageSize::Fit, ImageQuality::Original), &no_progress).unwrap();

    let image = &images_in(&out)[0];
    assert_eq!(image.dict.get(b"Width").unwrap(), &Object::Integer(20));
    assert_eq!(image.dict.get(b"Height").unwrap(), &Object::Integer(40));
}

#[test]
fn transparent_pixels_become_white() {
    let dir = temp_dir("img2pdf-alpha");
    let inputs = mixed_images(&dir);
    let out = dir.join("out.pdf");

    images_to_pdf(&inputs[0..1], &out, &options(PageSize::Fit, ImageQuality::Original), &no_progress).unwrap();

    let image = &images_in(&out)[0];
    assert!(!image.dict.has(b"SMask"));
    let pixels = image.decompressed_content().unwrap();
    assert_eq!(&pixels[0..3], &[255, 255, 255], "kiri transparan → putih");
    assert_eq!(&pixels[399 * 3..400 * 3], &[255, 0, 0], "kanan tetap merah");
}

#[test]
fn quality_presets_limit_image_size() {
    let dir = temp_dir("img2pdf-resize");
    let big = dir.join("big.png");
    RgbImage::from_pixel(3200, 1600, Rgb([90, 90, 90])).save(&big).unwrap();
    let out = dir.join("out.pdf");

    images_to_pdf(&[big], &out, &options(PageSize::A4, ImageQuality::Medium), &no_progress).unwrap();

    let image = &images_in(&out)[0];
    assert_eq!(image.dict.get(b"Width").unwrap(), &Object::Integer(2000));
    assert_eq!(image.dict.get(b"Height").unwrap(), &Object::Integer(1000));
}

#[test]
fn broken_image_reports_invalid_image() {
    let dir = temp_dir("img2pdf-broken");
    let broken = dir.join("broken.jpg");
    std::fs::write(&broken, b"\xFF\xD8\xFF bukan jpeg").unwrap();

    let err = images_to_pdf(&[broken.clone()], &dir.join("out.pdf"), &options(PageSize::A4, ImageQuality::Medium), &no_progress)
        .unwrap_err();

    assert_eq!(err.code(), "invalid_image");
    assert_eq!(err.path(), Some(broken.as_path()));
}
