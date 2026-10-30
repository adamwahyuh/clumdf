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

mod edit {
    use super::common::{build_pdf, page_labels, temp_dir};
    use lopdf::{Dictionary, Document, Object, Stream};
    use pdf_core::metadata::{decode_text, encode_text};
    use pdf_core::{PdfMetadata, read_metadata, write_metadata};

    #[test]
    fn text_strings_round_trip_in_any_script() {
        for text in ["Laporan Tahunan", "Pérez — “kutip”", "日本語"] {
            let Object::String(bytes, _) = encode_text(text) else { panic!() };
            assert_eq!(decode_text(&bytes), text);
        }
        assert_eq!(decode_text(b"\xEF\xBB\xBFUTF-8 \xE2\x80\x94"), "UTF-8 —");
        assert_eq!(decode_text(b"Caf\xE9"), "Café", "PDFDocEncoding ≈ Latin-1");
    }

    #[test]
    fn edits_fields_keeps_dates_and_drops_stale_xmp() {
        let dir = temp_dir("metadata-edit");
        let input = dir.join("in.pdf");
        let mut doc = build_pdf("isi", 1);
        let info = doc.add_object(Dictionary::from_iter([
            ("Title", Object::string_literal("Draf lama")),
            ("Author", Object::string_literal("Tanpa Nama")),
            ("Keywords", Object::string_literal("hapus saya")),
            ("CreationDate", Object::string_literal("D:20240131120000+07'00'")),
        ]));
        doc.trailer.set("Info", info);
        let xmp = doc.add_object(Stream::new(Dictionary::new(), b"<dc:title>Draf lama</dc:title>".to_vec()));
        let catalog = doc.trailer.get(b"Root").unwrap().as_reference().unwrap();
        doc.get_dictionary_mut(catalog).unwrap().set("Metadata", xmp);
        doc.save(&input).unwrap();
        let out = dir.join("out.pdf");

        let before = read_metadata(&input).unwrap();
        assert_eq!(before.title.as_deref(), Some("Draf lama"));
        let edited = PdfMetadata {
            title: Some("Laporan — Final".into()),
            author: Some("Budi Santoso".into()),
            keywords: Some("  ".into()), // kosong = hapus
            ..before
        };
        write_metadata(&input, &out, &edited).unwrap();

        let after = read_metadata(&out).unwrap();
        assert_eq!(after.title.as_deref(), Some("Laporan — Final"));
        assert_eq!(after.author.as_deref(), Some("Budi Santoso"));
        assert_eq!(after.keywords, None);
        assert_eq!(after.creation_date.as_deref(), Some("D:20240131120000+07'00'"));
        let bytes = std::fs::read(&out).unwrap();
        assert!(!String::from_utf8_lossy(&bytes).contains("Draf lama"), "XMP lama harus ikut hilang");
        assert_eq!(page_labels(&out), ["isi-p1"]);
        assert!(Document::load(&out).unwrap().trailer.has(b"Info"));
    }

    #[test]
    fn pdf_without_info_reads_as_empty() {
        let dir = temp_dir("metadata-empty");
        let input = super::common::write_pdf(&dir, "a", 1);
        assert_eq!(read_metadata(&input).unwrap(), PdfMetadata::default());
    }
}
