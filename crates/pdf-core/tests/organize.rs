mod common;

use lopdf::{Dictionary, Document, Object};
use pdf_core::{PageEdit, edit_pages};

use common::{build_pdf, page_labels, temp_dir, write_pdf};

fn page(page: u32, rotate: i64) -> PageEdit {
    PageEdit { page, rotate }
}

fn rotations(path: &std::path::Path) -> Vec<i64> {
    let doc = Document::load(path).unwrap();
    doc.get_pages()
        .values()
        .map(|&id| doc.get_dictionary(id).unwrap().get(b"Rotate").unwrap().as_i64().unwrap())
        .collect()
}

#[test]
fn reorders_and_drops_pages() {
    let dir = temp_dir("organize-order");
    let input = write_pdf(&dir, "a", 4);
    let out = dir.join("out.pdf");

    let report = edit_pages(&input, &out, &[page(3, 0), page(1, 0)]).unwrap();

    assert_eq!(report.page_count, 2);
    assert_eq!(page_labels(&out), ["a-p3", "a-p1"]);
}

#[test]
fn rotates_relative_to_current_rotation() {
    let dir = temp_dir("organize-rotate");
    let input = dir.join("in.pdf");
    let mut doc = build_pdf("a", 3);
    // Halaman 2 sudah diputar 90°; rotasi baru ditambahkan, bukan menggantikan
    let second = doc.get_pages()[&2];
    doc.get_dictionary_mut(second).unwrap().set("Rotate", 90);
    doc.save(&input).unwrap();
    let out = dir.join("out.pdf");

    edit_pages(&input, &out, &[page(1, 90), page(2, 90), page(3, -90)]).unwrap();

    assert_eq!(rotations(&out), [90, 180, 270]);
}

#[test]
fn pages_keep_inherited_media_box() {
    let dir = temp_dir("organize-inherit");
    let input = write_pdf(&dir, "a", 2);
    let out = dir.join("out.pdf");

    edit_pages(&input, &out, &[page(2, 0)]).unwrap();

    let doc = Document::load(&out).unwrap();
    let id = doc.get_pages()[&1];
    assert!(doc.get_dictionary(id).unwrap().has(b"MediaBox"));
}

#[test]
fn deleted_pages_leave_no_content_behind_even_when_linked() {
    let dir = temp_dir("organize-privacy");
    let input = dir.join("in.pdf");
    let mut doc = build_pdf("rahasia", 2);
    let pages = doc.get_pages();
    let (first, second) = (pages[&1], pages[&2]);
    // Link di halaman 1 menunjuk ke halaman 2 yang akan dihapus
    let link = doc.add_object(Dictionary::from_iter([
        ("Type", Object::Name(b"Annot".to_vec())),
        ("Subtype", Object::Name(b"Link".to_vec())),
        ("Rect", vec![0.into(), 0.into(), 10.into(), 10.into()].into()),
        ("Dest", vec![second.into(), Object::Name(b"Fit".to_vec())].into()),
    ]));
    doc.get_dictionary_mut(first).unwrap().set("Annots", vec![Object::from(link)]);
    doc.save(&input).unwrap();
    let out = dir.join("out.pdf");

    edit_pages(&input, &out, &[page(1, 0)]).unwrap();

    let result = Document::load(&out).unwrap();
    for object in result.objects.values() {
        if let Ok(stream) = object.as_stream() {
            let content = stream.decompressed_content().unwrap_or_else(|_| stream.content.clone());
            assert!(!String::from_utf8_lossy(&content).contains("rahasia-p2"), "isi halaman 2 masih ada");
        }
    }
}

#[test]
fn rejects_invalid_edits() {
    let dir = temp_dir("organize-invalid");
    let input = write_pdf(&dir, "a", 2);
    let out = dir.join("out.pdf");

    for edits in [vec![page(3, 0)], vec![page(1, 0), page(1, 0)], vec![page(1, 45)]] {
        assert_eq!(edit_pages(&input, &out, &edits).unwrap_err().code(), "invalid_range", "{edits:?}");
    }
    assert_eq!(edit_pages(&input, &out, &[]).unwrap_err().code(), "no_input");
    assert!(!out.exists());
}
