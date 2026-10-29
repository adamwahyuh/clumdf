mod common;

use std::sync::Mutex;

use lopdf::encryption::{EncryptionState, EncryptionVersion, Permissions};
use lopdf::{Document, Object};
use pdf_core::{inspect, merge_pdfs};

use common::{build_pdf, page_labels, temp_dir, write_pdf};

fn no_progress(_: usize, _: usize) {}

#[test]
fn merge_keeps_input_order_and_page_order() {
    let dir = temp_dir("merge-order");
    let a = write_pdf(&dir, "a", 2);
    let b = write_pdf(&dir, "b", 1);
    let c = write_pdf(&dir, "c", 3);
    let out = dir.join("out.pdf");

    // Urutan input sengaja tidak alfabetis
    let report = merge_pdfs(&[c, a, b], &out, &no_progress).unwrap();

    assert_eq!(report.page_count, 6);
    assert_eq!(page_labels(&out), ["c-p1", "c-p2", "c-p3", "a-p1", "a-p2", "b-p1"]);
    assert_eq!(report.output_size, std::fs::metadata(&out).unwrap().len());
}

#[test]
fn merge_same_file_twice() {
    let dir = temp_dir("merge-twice");
    let a = write_pdf(&dir, "a", 2);
    let out = dir.join("out.pdf");

    merge_pdfs(&[a.clone(), a], &out, &no_progress).unwrap();

    assert_eq!(page_labels(&out), ["a-p1", "a-p2", "a-p1", "a-p2"]);
}

#[test]
fn merged_pages_carry_inherited_attributes() {
    let dir = temp_dir("merge-inherit");
    let a = write_pdf(&dir, "a", 2);
    let out = dir.join("out.pdf");

    merge_pdfs(&[a], &out, &no_progress).unwrap();

    let doc = Document::load(&out).unwrap();
    for &page_id in doc.get_pages().values() {
        let page = doc.get_dictionary(page_id).unwrap();
        let media_box = page.get(b"MediaBox").unwrap().as_array().unwrap();
        assert_eq!(media_box[2], Object::Integer(595));
        assert!(page.has(b"Resources"), "Resources harus disalin ke halaman");
    }
}

#[test]
fn merge_reports_progress_until_done() {
    let dir = temp_dir("merge-progress");
    let a = write_pdf(&dir, "a", 1);
    let b = write_pdf(&dir, "b", 1);
    let calls = Mutex::new(Vec::new());

    merge_pdfs(&[a, b], &dir.join("out.pdf"), &|cur, total| calls.lock().unwrap().push((cur, total)))
        .unwrap();

    assert_eq!(*calls.lock().unwrap(), [(1, 3), (2, 3), (3, 3)]);
}

#[test]
fn merge_rejects_encrypted_input_without_writing_output() {
    let dir = temp_dir("merge-encrypted");
    let a = write_pdf(&dir, "a", 1);
    let locked = dir.join("locked.pdf");
    let mut doc = build_pdf("locked", 1);
    let file_id = Object::string_literal(b"berkas-test-id0".to_vec());
    doc.trailer.set("ID", vec![file_id.clone(), file_id]);
    let state = EncryptionState::try_from(EncryptionVersion::V2 {
        document: &doc,
        owner_password: "owner",
        user_password: "user",
        key_length: 128,
        permissions: Permissions::all(),
    })
    .unwrap();
    doc.encrypt(&state).unwrap();
    doc.save(&locked).unwrap();
    let out = dir.join("out.pdf");

    let err = merge_pdfs(&[a, locked.clone()], &out, &no_progress).unwrap_err();

    assert_eq!(err.code(), "encrypted");
    assert_eq!(err.path(), Some(locked.as_path()));
    assert!(!out.exists());
}

#[test]
fn merge_rejects_corrupt_input() {
    let dir = temp_dir("merge-corrupt");
    let bad = dir.join("bad.pdf");
    std::fs::write(&bad, b"bukan pdf").unwrap();

    let err = merge_pdfs(&[bad], &dir.join("out.pdf"), &no_progress).unwrap_err();

    assert_eq!(err.code(), "invalid");
}

#[test]
fn merge_rejects_empty_input() {
    let dir = temp_dir("merge-empty");
    let err = merge_pdfs(&[], &dir.join("out.pdf"), &no_progress).unwrap_err();
    assert_eq!(err.code(), "no_input");
}

#[test]
fn inspect_reports_page_count_and_size() {
    let dir = temp_dir("inspect");
    let a = write_pdf(&dir, "a", 3);

    let info = inspect(&a).unwrap();

    assert_eq!(info.page_count, 3);
    assert_eq!(info.file_size, std::fs::metadata(&a).unwrap().len());
}

#[test]
fn inspect_missing_file_is_io_error() {
    let dir = temp_dir("inspect-missing");
    assert_eq!(inspect(&dir.join("tidak-ada.pdf")).unwrap_err().code(), "io");
}
