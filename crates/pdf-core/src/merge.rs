use std::path::{Path, PathBuf};

use lopdf::{Dictionary, Document, Object, ObjectId};
use serde::Serialize;

use crate::document;
use crate::error::PdfError;
use crate::page_tree;
use crate::progress::ProgressReporter;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeReport {
    pub page_count: usize,
    pub output_size: u64,
}

/// Gabungkan `inputs` sesuai urutan menjadi satu PDF di `output`.
/// Progress dilaporkan per file input (+1 langkah untuk menyimpan).
pub fn merge_pdfs(
    inputs: &[PathBuf],
    output: &Path,
    reporter: &impl ProgressReporter,
) -> Result<MergeReport, PdfError> {
    if inputs.is_empty() {
        return Err(PdfError::NoInput);
    }
    let total = inputs.len() + 1;

    let mut merged = Document::with_version("1.7");
    let mut page_ids: Vec<ObjectId> = Vec::new();

    for (i, path) in inputs.iter().enumerate() {
        let mut doc = document::open(path)?;
        // Harus sebelum halaman dipindah: setelah itu node induk lamanya hilang.
        for &page_id in doc.get_pages().values() {
            let inherited = page_tree::inherited_attributes(&doc, page_id);
            let page = doc.get_dictionary_mut(page_id).map_err(|e| PdfError::invalid(path, e))?;
            for (key, value) in inherited {
                page.set(key, value);
            }
        }
        // Geser nomor objek supaya tidak bentrok dengan dokumen sebelumnya.
        doc.renumber_objects_with(merged.max_id + 1);
        merged.max_id = doc.max_id;
        // get_pages() berurutan sesuai nomor halaman, bukan nomor objek.
        page_ids.extend(doc.get_pages().into_values());
        merged.objects.extend(doc.objects);
        reporter.report(i + 1, total);
    }

    let pages_id = merged.new_object_id();
    for &page_id in &page_ids {
        if let Ok(page) = merged.get_dictionary_mut(page_id) {
            page.set("Parent", pages_id);
        }
    }
    let kids: Vec<Object> = page_ids.iter().map(|&id| id.into()).collect();
    merged.objects.insert(
        pages_id,
        Dictionary::from_iter([
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Count", Object::Integer(page_ids.len() as i64)),
            ("Kids", Object::Array(kids)),
        ])
        .into(),
    );
    let catalog_id = merged.add_object(Dictionary::from_iter([
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
    ]));
    merged.trailer.set("Root", catalog_id);

    // Catalog & pohon Pages lama tidak lagi direferensikan; buang beserta isinya.
    // shortcut: outline/bookmark, form, dan link antar-dokumen ikut terbuang; dukung bila ada permintaan.
    merged.prune_objects();

    document::save(&mut merged, output)?;
    reporter.report(total, total);

    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(MergeReport { page_count: page_ids.len(), output_size })
}
