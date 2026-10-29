mod common;

use lopdf::{Document, Object};
use pdf_core::{CompressOptions, compress_pdf};

use common::{build_pdf, encrypt, images_in, pdf_with_images, photo_samples, raw_image, temp_dir, write_pdf};

fn no_progress(_: usize, _: usize) {}

/// Preset "Seimbang"
fn balanced() -> CompressOptions {
    CompressOptions { max_dimension: 1600, jpeg_quality: 65, grayscale: false }
}

fn photo_pdf(dir: &std::path::Path) -> std::path::PathBuf {
    let mut doc = Document::with_version("1.7");
    let photo = doc.add_object(raw_image(2400, 1800, "DeviceRGB", photo_samples(2400, 1800)));
    let path = dir.join("photo.pdf");
    pdf_with_images(doc, &[photo]).save(&path).unwrap();
    path
}

fn name(stream: &lopdf::Stream, key: &[u8]) -> Vec<u8> {
    stream.dict.get(key).and_then(Object::as_name).unwrap().to_vec()
}

#[test]
fn photo_pdf_gets_much_smaller() {
    let dir = temp_dir("compress-photo");
    let input = photo_pdf(&dir);
    let out = dir.join("out.pdf");

    let report = compress_pdf(&input, &out, &balanced(), &no_progress).unwrap();

    assert!(!report.already_optimal);
    assert_eq!(report.images_optimized, 1);
    assert!(report.compressed_size * 2 < report.original_size, "{report:?}");
    assert_eq!(report.compressed_size, std::fs::metadata(&out).unwrap().len());

    let images = images_in(&out);
    assert_eq!(images.len(), 1);
    assert_eq!(name(&images[0], b"Filter"), b"DCTDecode");
    assert_eq!(images[0].dict.get(b"Width").unwrap().as_i64().unwrap(), 1600);
    assert_eq!(images[0].dict.get(b"Height").unwrap().as_i64().unwrap(), 1200);
}

#[test]
fn text_only_pdf_never_grows() {
    let dir = temp_dir("compress-text");
    let input = write_pdf(&dir, "text", 3);
    let out = dir.join("out.pdf");

    let report = compress_pdf(&input, &out, &balanced(), &no_progress).unwrap();

    assert!(report.compressed_size <= report.original_size);
    assert_eq!(report.images_optimized, 0);
    if report.already_optimal {
        assert_eq!(std::fs::read(&out).unwrap(), std::fs::read(&input).unwrap());
    }
}

#[test]
fn optimal_pdf_is_copied_unchanged() {
    let dir = temp_dir("compress-optimal");
    let input = dir.join("small.pdf");
    // Hasil save_modern dari PDF yang sama tidak akan lebih kecil lagi
    let first = dir.join("first.pdf");
    build_pdf("x", 1).save(&first).unwrap();
    compress_pdf(&first, &input, &balanced(), &no_progress).unwrap();
    let out = dir.join("out.pdf");

    let report = compress_pdf(&input, &out, &balanced(), &no_progress).unwrap();

    assert!(report.already_optimal);
    assert_eq!(std::fs::read(&out).unwrap(), std::fs::read(&input).unwrap());
}

#[test]
fn grayscale_option_converts_images() {
    let dir = temp_dir("compress-gray");
    let input = photo_pdf(&dir);
    let out = dir.join("out.pdf");
    let options = CompressOptions { grayscale: true, ..balanced() };

    compress_pdf(&input, &out, &options, &no_progress).unwrap();

    assert_eq!(name(&images_in(&out)[0], b"ColorSpace"), b"DeviceGray");
}

#[test]
fn cmyk_images_are_left_alone() {
    let dir = temp_dir("compress-cmyk");
    let mut doc = Document::with_version("1.7");
    let cmyk = doc.add_object(raw_image(800, 600, "DeviceCMYK", vec![128; 800 * 600 * 4]));
    let input = dir.join("cmyk.pdf");
    pdf_with_images(doc, &[cmyk]).save(&input).unwrap();
    let out = dir.join("out.pdf");

    let report = compress_pdf(&input, &out, &balanced(), &no_progress).unwrap();

    assert_eq!(report.images_optimized, 0);
    assert_eq!(name(&images_in(&out)[0], b"ColorSpace"), b"DeviceCMYK");
}

#[test]
fn soft_masks_are_left_alone() {
    let dir = temp_dir("compress-smask");
    let mut doc = Document::with_version("1.7");
    let mask = doc.add_object(raw_image(2400, 1800, "DeviceGray", photo_samples(2400, 600)));
    let mut photo = raw_image(2400, 1800, "DeviceRGB", photo_samples(2400, 1800));
    photo.dict.set("SMask", mask);
    let photo = doc.add_object(photo);
    let input = dir.join("smask.pdf");
    pdf_with_images(doc, &[photo]).save(&input).unwrap();
    let out = dir.join("out.pdf");

    let report = compress_pdf(&input, &out, &balanced(), &no_progress).unwrap();

    assert_eq!(report.images_optimized, 1, "hanya gambar utama yang diproses");
    let images = images_in(&out);
    let mask = images.iter().find(|s| name(s, b"ColorSpace") == b"DeviceGray").unwrap();
    assert_eq!(name(mask, b"Filter"), b"FlateDecode");
    let photo = images.iter().find(|s| s.dict.has(b"SMask")).unwrap();
    assert_eq!(name(photo, b"Filter"), b"DCTDecode");
}

#[test]
fn encrypted_pdf_is_rejected() {
    let dir = temp_dir("compress-encrypted");
    let input = dir.join("locked.pdf");
    let mut doc = build_pdf("locked", 1);
    encrypt(&mut doc);
    doc.save(&input).unwrap();

    let err = compress_pdf(&input, &dir.join("out.pdf"), &balanced(), &no_progress).unwrap_err();

    assert_eq!(err.code(), "encrypted");
}
