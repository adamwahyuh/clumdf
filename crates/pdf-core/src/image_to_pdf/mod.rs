mod embed;
pub mod layout;
mod options;

pub use options::{ImageQuality, ImageToPdfOptions, Margin, PageOrientation, PageSize};

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use lopdf::content::{Content, Operation};
use lopdf::{Dictionary, Document, Object, Stream};
use rayon::prelude::*;
use serde::Serialize;

use crate::document;
use crate::error::PdfError;
use crate::progress::ProgressReporter;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageToPdfReport {
    pub page_count: usize,
    pub output_size: u64,
}

/// Satu gambar = satu halaman, sesuai urutan `inputs`.
/// Progress dilaporkan per gambar (+1 langkah untuk menyimpan).
pub fn images_to_pdf(
    inputs: &[PathBuf],
    output: &Path,
    options: &ImageToPdfOptions,
    reporter: &impl ProgressReporter,
) -> Result<ImageToPdfReport, PdfError> {
    if inputs.is_empty() {
        return Err(PdfError::NoInput);
    }
    let total = inputs.len() + 1;
    let done = AtomicUsize::new(0);
    // par_iter + collect tetap menjaga urutan input
    let embedded: Vec<_> = inputs
        .par_iter()
        .map(|path| {
            let result = embed::embed(path, options.quality);
            reporter.report(done.fetch_add(1, Ordering::Relaxed) + 1, total);
            result
        })
        .collect();

    let mut doc = Document::with_version("1.7");
    let pages_id = doc.new_object_id();
    let mut kids = Vec::with_capacity(inputs.len());

    for item in embedded {
        let (image, size_px) = item?;
        let p = layout::place(size_px, options.page_size, options.orientation, options.margin);
        let image_id = doc.add_object(image);
        let content = Content {
            operations: vec![
                Operation::new("q", vec![]),
                Operation::new("cm", [p.width, 0.0, 0.0, p.height, p.x, p.y].map(Object::Real).to_vec()),
                Operation::new("Do", vec![Object::Name(b"Im0".to_vec())]),
                Operation::new("Q", vec![]),
            ],
        };
        let content_bytes = content.encode().map_err(|e| PdfError::io(output, std::io::Error::other(e)))?;
        let content_id = doc.add_object(Stream::new(Dictionary::new(), content_bytes));
        let resources = Dictionary::from_iter([(
            "XObject",
            Object::from(Dictionary::from_iter([("Im0", Object::Reference(image_id))])),
        )]);
        let page_id = doc.add_object(Dictionary::from_iter([
            ("Type", Object::Name(b"Page".to_vec())),
            ("Parent", Object::Reference(pages_id)),
            ("MediaBox", [0.0, 0.0, p.page_width, p.page_height].map(Object::Real).to_vec().into()),
            ("Contents", Object::Reference(content_id)),
            ("Resources", resources.into()),
        ]));
        kids.push(Object::Reference(page_id));
    }

    let page_count = kids.len();
    doc.objects.insert(
        pages_id,
        Dictionary::from_iter([
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Count", Object::Integer(page_count as i64)),
            ("Kids", Object::Array(kids)),
        ])
        .into(),
    );
    let catalog_id = doc.add_object(Dictionary::from_iter([
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
    ]));
    doc.trailer.set("Root", catalog_id);
    doc.compress();

    document::save(&mut doc, output)?;
    reporter.report(total, total);
    let output_size = std::fs::metadata(output).map_err(|e| PdfError::io(output, e))?.len();
    Ok(ImageToPdfReport { page_count, output_size })
}
