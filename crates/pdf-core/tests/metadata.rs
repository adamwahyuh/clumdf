mod common;

use lopdf::{Dictionary, Document, Object, Stream};
use pdf_core::strip_metadata;

use common::{build_pdf, page_labels, temp_dir, write_pdf};

#[test]
fn removes_info_and_xmp_but_keeps_pages() {
    let dir = temp_dir("metadata-strip");
    let input = dir.join("in.pdf");
    let mut doc = build_pdf("isi", 2);
    let info = doc.add_object(Dictionary::from_iter([
        ("Author", Object::string_literal("Budi Santoso")),
        ("Producer", Object::string_literal("Aplikasi Rahasia")),
    ]));
    doc.trailer.set("Info", info);
    let xmp = doc.add_object(Stream::new(
        Dictionary::from_iter([("Type", Object::Name(b"Metadata".to_vec())), ("Subtype", Object::Name(b"XML".to_vec()))]),
        b"<x:xmpmeta><dc:creator>Budi Santoso</dc:creator></x:xmpmeta>".to_vec(),
    ));
    let catalog = doc.trailer.get(b"Root").unwrap().as_reference().unwrap();
    doc.get_dictionary_mut(catalog).unwrap().set("Metadata", xmp);
    doc.save(&input).unwrap();
    let out = dir.join("out.pdf");

    let report = strip_metadata(&input, &out).unwrap();

    assert_eq!(report.removed, 2);
    let bytes = std::fs::read(&out).unwrap();
    assert!(!String::from_utf8_lossy(&bytes).contains("Budi"), "nama penulis masih ada");
    let result = Document::load(&out).unwrap();
    assert!(!result.trailer.has(b"Info"));
    assert_eq!(page_labels(&out), ["isi-p1", "isi-p2"]);
}

#[test]
fn clean_file_reports_nothing_removed() {
    let dir = temp_dir("metadata-clean");
    let input = write_pdf(&dir, "a", 1);
    let report = strip_metadata(&input, &dir.join("out.pdf")).unwrap();
    assert_eq!(report.removed, 0);
}
