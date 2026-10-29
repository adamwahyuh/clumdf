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

/// Kunci PDF dengan password (RC4-128) untuk menguji deteksi PDF terkunci.
pub fn encrypt(doc: &mut Document) {
    use lopdf::encryption::{EncryptionState, EncryptionVersion, Permissions};
    let file_id = Object::string_literal(b"berkas-test-id0".to_vec());
    doc.trailer.set("ID", vec![file_id.clone(), file_id]);
    let state = EncryptionState::try_from(EncryptionVersion::V2 {
        document: doc,
        owner_password: "owner",
        user_password: "user",
        key_length: 128,
        permissions: Permissions::all(),
    })
    .unwrap();
    doc.encrypt(&state).unwrap();
}

/// Sampel RGB mirip foto: gradien halus + sedikit noise (deterministik).
pub fn photo_samples(width: u32, height: u32) -> Vec<u8> {
    let mut seed: u32 = 12345;
    let mut out = Vec::with_capacity((width * height * 3) as usize);
    for y in 0..height {
        for x in 0..width {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
            let noise = (seed >> 24) as u8 / 16;
            out.push(((x * 255 / width) as u8).saturating_add(noise));
            out.push(((y * 255 / height) as u8).saturating_add(noise));
            out.push((((x + y) * 128 / (width + height)) as u8).saturating_add(noise));
        }
    }
    out
}

/// Image XObject mentah (Flate) dengan ruang warna `color_space`.
pub fn raw_image(width: u32, height: u32, color_space: &str, samples: Vec<u8>) -> Stream {
    let dict = Dictionary::from_iter([
        ("Type", Object::Name(b"XObject".to_vec())),
        ("Subtype", Object::Name(b"Image".to_vec())),
        ("Width", Object::Integer(width.into())),
        ("Height", Object::Integer(height.into())),
        ("ColorSpace", Object::Name(color_space.as_bytes().to_vec())),
        ("BitsPerComponent", Object::Integer(8)),
    ]);
    let mut stream = Stream::new(dict, samples);
    stream.compress().unwrap();
    stream
}

/// PDF satu halaman yang menggambar `images` (sudah ditambahkan ke `doc`).
pub fn pdf_with_images(mut doc: Document, images: &[lopdf::ObjectId]) -> Document {
    let mut xobjects = Dictionary::new();
    let mut ops = Vec::new();
    for (i, id) in images.iter().enumerate() {
        let name = format!("Im{i}");
        xobjects.set(name.clone(), *id);
        ops.push(Operation::new("q", vec![]));
        ops.push(Operation::new("cm", vec![500.into(), 0.into(), 0.into(), 300.into(), 40.into(), 400.into()]));
        ops.push(Operation::new("Do", vec![Object::Name(name.into_bytes())]));
        ops.push(Operation::new("Q", vec![]));
    }
    let content_id = doc.add_object(Stream::new(Dictionary::new(), Content { operations: ops }.encode().unwrap()));
    let pages_id = doc.new_object_id();
    let page_id = doc.add_object(Dictionary::from_iter([
        ("Type", Object::Name(b"Page".to_vec())),
        ("Parent", pages_id.into()),
        ("Contents", content_id.into()),
        ("MediaBox", vec![0.into(), 0.into(), 595.into(), 842.into()].into()),
        ("Resources", Dictionary::from_iter([("XObject", Object::from(xobjects))]).into()),
    ]));
    doc.objects.insert(
        pages_id,
        Dictionary::from_iter([
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Count", Object::Integer(1)),
            ("Kids", vec![Object::from(page_id)].into()),
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

/// Semua image XObject di file, untuk memeriksa hasil.
pub fn images_in(path: &Path) -> Vec<Stream> {
    let doc = Document::load(path).unwrap();
    doc.objects
        .values()
        .filter_map(|o| o.as_stream().ok())
        .filter(|s| s.dict.get(b"Subtype").ok() == Some(&Object::Name(b"Image".to_vec())))
        .cloned()
        .collect()
}
