//! Menambahkan sesuatu di atas halaman (teks, gambar) tanpa mengganggu konten asli.

use lopdf::{Dictionary, Document, Object, ObjectId, Stream};

use crate::page_tree;
use crate::stamp::layout::{VisiblePage, visible_page};

/// Salin atribut warisan ke halaman, daftarkan `resource` (mis. font atau
/// gambar) di Resources halaman, lalu kembalikan bentuk halaman yang terlihat.
/// `category` = kunci di Resources, mis. b"Font" atau b"XObject".
pub(crate) fn prepare_page(
    doc: &mut Document,
    page_id: ObjectId,
    category: &[u8],
    name: &str,
    resource: ObjectId,
) -> lopdf::Result<VisiblePage> {
    let inherited = page_tree::inherited_attributes(doc, page_id);
    let resolve = |doc: &Document, value: Option<&Object>| -> Dictionary {
        match value {
            Some(Object::Reference(id)) => doc.get_dictionary(*id).cloned().unwrap_or_default(),
            Some(Object::Dictionary(dict)) => dict.clone(),
            _ => Dictionary::new(),
        }
    };

    let page = doc.get_dictionary(page_id)?;
    let lookup = |key: &[u8]| page.get(key).ok().or_else(|| inherited.iter().find(|(k, _)| *k == key).map(|(_, v)| v));
    // Resources bisa dipakai bersama halaman lain; salinan langsung di halaman aman diubah
    let mut resources = resolve(doc, lookup(b"Resources"));
    let mut entries = resolve(doc, resources.get(category).ok());
    entries.set(name, resource);
    resources.set(category.to_vec(), entries);

    let number = |o: &Object| o.as_float().ok().or_else(|| o.as_i64().ok().map(|n| n as f32));
    let bbox = lookup(b"CropBox")
        .or_else(|| lookup(b"MediaBox"))
        .and_then(|o| o.as_array().ok())
        .and_then(|a| Some([number(a.first()?)?, number(a.get(1)?)?, number(a.get(2)?)?, number(a.get(3)?)?]))
        .unwrap_or([0.0, 0.0, 595.0, 842.0]); // A4 bila PDF tidak menyebut ukuran
    let rotate = lookup(b"Rotate").and_then(|o| o.as_i64().ok()).unwrap_or(0);

    let page = doc.get_dictionary_mut(page_id)?;
    for (key, value) in inherited {
        page.set(key, value);
    }
    page.set("Resources", resources);
    Ok(visible_page(bbox, rotate))
}

/// Stream "q" yang membuka pembungkus konten asli; satu cukup untuk semua halaman.
pub(crate) fn save_state_stream(doc: &mut Document) -> ObjectId {
    doc.add_object(Stream::new(Dictionary::new(), b"q\n".to_vec()))
}

/// Contents baru: [q, …konten asli…, Q + `overlay`]. `overlay` diawali "Q\n"
/// supaya keadaan grafis konten asli tidak memengaruhi tambahan kita.
pub(crate) fn wrap_contents(doc: &mut Document, page_id: ObjectId, save_state: ObjectId, overlay: String) -> lopdf::Result<()> {
    let overlay_id = doc.add_object(Stream::new(Dictionary::new(), format!("Q\n{overlay}").into_bytes()));
    let page = doc.get_dictionary_mut(page_id)?;
    let mut contents = vec![Object::Reference(save_state)];
    match page.get(b"Contents") {
        Ok(Object::Array(items)) => contents.extend(items.iter().cloned()),
        Ok(other) => contents.push(other.clone()),
        Err(_) => {}
    }
    contents.push(Object::Reference(overlay_id));
    page.set("Contents", contents);
    Ok(())
}
