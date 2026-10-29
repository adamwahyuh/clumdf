//! Operasi bersama pada pohon halaman: atribut warisan & menyalin sebagian halaman.

use std::collections::HashSet;

use lopdf::{Dictionary, Document, Object, ObjectId};

/// Atribut halaman yang boleh diwariskan dari node `Pages` induk.
const INHERITABLE: [&[u8]; 4] = [b"Resources", b"MediaBox", b"CropBox", b"Rotate"];

/// Atribut warisan yang belum ada di halaman, dicari ke atas lewat `Parent`.
/// Wajib disalin sebelum halaman dipindah ke pohon `Pages` baru.
pub(crate) fn inherited_attributes(doc: &Document, page_id: ObjectId) -> Vec<(&'static [u8], Object)> {
    let Ok(page) = doc.get_dictionary(page_id) else { return Vec::new() };
    let mut found = Vec::new();

    for key in INHERITABLE.into_iter().filter(|key| !page.has(key)) {
        let mut parent = page.get(b"Parent").and_then(Object::as_reference).ok();
        // Batas kedalaman melindungi dari rantai Parent yang melingkar di PDF rusak.
        for _ in 0..64 {
            let Some(node) = parent.and_then(|id| doc.get_dictionary(id).ok()) else { break };
            if let Ok(value) = node.get(key) {
                found.push((key, value.clone()));
                break;
            }
            parent = node.get(b"Parent").and_then(Object::as_reference).ok();
        }
    }
    found
}

/// Dokumen baru yang hanya berisi `pages` (urutan sesuai slice) beserta objek
/// yang dipakainya; `rotate` ditambahkan ke rotasi halaman (kelipatan 90).
///
/// Referensi ke halaman yang tidak dipilih (mis. link atau bookmark) diganti
/// `null`, supaya isi halaman yang dibuang benar-benar tidak ikut tersimpan.
/// shortcut: bookmark, form, dan metadata tidak dibawa; dukung bila ada permintaan.
pub(crate) fn extract_pages(src: &Document, pages: &[(ObjectId, i64)]) -> Document {
    let all_pages: HashSet<ObjectId> = src.get_pages().into_values().collect();
    let keep: HashSet<ObjectId> = pages.iter().map(|&(id, _)| id).collect();

    let mut out = Document::with_version(src.version.clone());
    out.max_id = src.max_id;

    for &(id, rotate) in pages {
        let Ok(page) = src.get_dictionary(id) else { continue };
        let mut page = page.clone();
        // Parent lama menunjuk ke seluruh pohon halaman asli; diganti di bawah
        page.remove(b"Parent");
        for (key, value) in inherited_attributes(src, id) {
            page.set(key, value);
        }
        let current = page.get(b"Rotate").and_then(Object::as_i64).unwrap_or(0);
        page.set("Rotate", (current + rotate).rem_euclid(360));
        out.objects.insert(id, Object::Dictionary(page));
    }

    // Salin semua objek yang terjangkau dari halaman terpilih
    let mut seen = keep.clone();
    let mut queue: Vec<ObjectId> = keep.iter().copied().collect();
    while let Some(id) = queue.pop() {
        let Some(mut object) = out.objects.remove(&id).or_else(|| src.objects.get(&id).cloned()) else {
            continue;
        };
        visit_references(&mut object, &mut |reference| {
            let Object::Reference(target) = *reference else { return };
            if all_pages.contains(&target) && !keep.contains(&target) {
                *reference = Object::Null;
            } else if seen.insert(target) {
                queue.push(target);
            }
        });
        out.objects.insert(id, object);
    }

    let pages_id = out.new_object_id();
    for &(id, _) in pages {
        if let Ok(page) = out.get_dictionary_mut(id) {
            page.set("Parent", pages_id);
        }
    }
    let kids: Vec<Object> = pages.iter().map(|&(id, _)| id.into()).collect();
    out.objects.insert(
        pages_id,
        Dictionary::from_iter([
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Count", Object::Integer(kids.len() as i64)),
            ("Kids", Object::Array(kids)),
        ])
        .into(),
    );
    let catalog_id = out.add_object(Dictionary::from_iter([
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
    ]));
    out.trailer.set("Root", catalog_id);
    out
}

fn visit_references(object: &mut Object, f: &mut impl FnMut(&mut Object)) {
    match object {
        Object::Reference(_) => f(object),
        Object::Array(items) => items.iter_mut().for_each(|item| visit_references(item, f)),
        Object::Dictionary(dict) => dict.iter_mut().for_each(|(_, v)| visit_references(v, f)),
        Object::Stream(stream) => stream.dict.iter_mut().for_each(|(_, v)| visit_references(v, f)),
        _ => {}
    }
}
