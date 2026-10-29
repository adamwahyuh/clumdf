//! Pembuat PDF uji yang dipakai bersama oleh file tes.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use lopdf::content::{Content, Operation};
use lopdf::{Dictionary, Document, Object, Stream};

/// Folder kosong unik per tes di temp dir sistem.
pub fn temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdf-core-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// PDF berisi `pages` halaman bertuliskan "{label}-p{n}".
/// Sengaja dibuat "sulit": MediaBox & Resources hanya ada di node `Pages`
/// (harus diwariskan), dan nomor objek halaman terbalik dari urutan halaman.
pub fn build_pdf(label: &str, pages: usize) -> Document {
    let mut doc = Document::with_version("1.7");
    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(Dictionary::from_iter([
        ("Type", Object::Name(b"Font".to_vec())),
        ("Subtype", Object::Name(b"Type1".to_vec())),
        ("BaseFont", Object::Name(b"Helvetica".to_vec())),
    ]));

    // Buat dari halaman terakhir dulu supaya ID objek berlawanan dengan urutan halaman.
    let mut kids: Vec<Object> = (1..=pages)
        .rev()
        .map(|n| {
            let content = Content {
                operations: vec![
                    Operation::new("BT", vec![]),
                    Operation::new("Tf", vec!["F1".into(), 24.into()]),
                    Operation::new("Td", vec![72.into(), 720.into()]),
                    Operation::new("Tj", vec![Object::string_literal(format!("{label}-p{n}"))]),
                    Operation::new("ET", vec![]),
                ],
            };
            let content_id = doc.add_object(Stream::new(Dictionary::new(), content.encode().unwrap()));
            doc.add_object(Dictionary::from_iter([
                ("Type", Object::Name(b"Page".to_vec())),
                ("Parent", pages_id.into()),
                ("Contents", content_id.into()),
            ]))
            .into()
        })
        .collect();
    kids.reverse();

    doc.objects.insert(
        pages_id,
        Dictionary::from_iter([
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Count", Object::Integer(pages as i64)),
            ("Kids", Object::Array(kids)),
            ("MediaBox", vec![0.into(), 0.into(), 595.into(), 842.into()].into()),
            (
                "Resources",
                Dictionary::from_iter([(
                    "Font",
                    Object::from(Dictionary::from_iter([("F1", Object::from(font_id))])),
                )])
                .into(),
            ),
        ])
        .into(),
    );
    let catalog_id = doc.add_object(Dictionary::from_iter([
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", pages_id.into()),
    ]));
    doc.trailer.set("Root", catalog_id);
    doc
}

pub fn write_pdf(dir: &Path, label: &str, pages: usize) -> PathBuf {
    let path = dir.join(format!("{label}.pdf"));
    build_pdf(label, pages).save(&path).unwrap();
    path
}

/// Teks penanda tiap halaman, sesuai urutan halaman.
pub fn page_labels(path: &Path) -> Vec<String> {
    let doc = Document::load(path).unwrap();
    doc.get_pages()
        .values()
        .map(|&id| {
            let content = String::from_utf8_lossy(&doc.get_page_content(id)).into_owned();
            let start = content.find('(').unwrap() + 1;
            let end = content.find(')').unwrap();
            content[start..end].to_string()
        })
        .collect()
}
