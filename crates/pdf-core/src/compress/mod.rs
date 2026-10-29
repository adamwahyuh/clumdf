mod colorspace;
mod image_stream;
mod options;

pub use options::CompressOptions;

use std::collections::HashSet;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use lopdf::{Document, Object, ObjectId, Stream};
use rayon::prelude::*;
use serde::Serialize;

use crate::document;
use crate::error::PdfError;
use crate::progress::ProgressReporter;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompressReport {
    pub original_size: u64,
    pub compressed_size: u64,
    pub images_optimized: usize,
    /// Hasil tidak lebih kecil, jadi file asli yang disalin ke `output`
    pub already_optimal: bool,
}

/// Kecilkan PDF dengan meng-encode ulang gambar yang aman diproses.
/// Progress dilaporkan per gambar (+1 langkah untuk menyimpan).
pub fn compress_pdf(
    input: &Path,
    output: &Path,
    options: &CompressOptions,
    reporter: &impl ProgressReporter,
) -> Result<CompressReport, PdfError> {
    let original = std::fs::read(input).map_err(|e| PdfError::io(input, e))?;
    let mut doc = document::open(input)?;

    let masks = mask_targets(&doc);
    let candidates: Vec<(ObjectId, u8)> = doc
        .objects
        .iter()
        .filter_map(|(&id, obj)| {
            let stream = obj.as_stream().ok()?;
            if masks.contains(&id) {
                return None;
            }
            Some((id, safe_image_channels(&doc, stream)?))
        })
        .collect();

    let total = candidates.len() + 1;
    let done = AtomicUsize::new(0);
    let replacements: Vec<(ObjectId, Stream)> = candidates
        .par_iter()
        .filter_map(|&(id, channels)| {
            let stream = doc.objects.get(&id)?.as_stream().ok()?;
            let result = image_stream::recompress(stream, channels, options);
            reporter.report(done.fetch_add(1, Ordering::Relaxed) + 1, total);
            result.map(|new| (id, new))
        })
        .collect();

    let images_optimized = replacements.len();
    for (id, stream) in replacements {
        doc.objects.insert(id, Object::Stream(stream));
    }
    doc.compress();
    doc.delete_zero_length_streams();
    doc.prune_objects();

    let bytes = document::to_bytes(&mut doc, output)?;
    let already_optimal = bytes.len() >= original.len();
    let written = if already_optimal { &original } else { &bytes };
    document::write_atomic(output, written)?;
    reporter.report(total, total);

    Ok(CompressReport {
        original_size: original.len() as u64,
        compressed_size: written.len() as u64,
        images_optimized: if already_optimal { 0 } else { images_optimized },
        already_optimal,
    })
}

/// Channel gambar bila aman di-encode ulang, `None` bila harus dilewati.
fn safe_image_channels(doc: &Document, stream: &Stream) -> Option<u8> {
    let dict = &stream.dict;
    let is = |key: &[u8], expected: &Object| dict.get(key).ok() == Some(expected);

    if !is(b"Subtype", &Object::Name(b"Image".to_vec()))
        || is(b"ImageMask", &Object::Boolean(true))
        || !is(b"BitsPerComponent", &Object::Integer(8))
        || dict.has(b"Decode")
        // Color-key mask bergantung pada nilai piksel persis; JPEG akan merusaknya
        || matches!(dict.get(b"Mask"), Ok(Object::Array(_)))
    {
        return None;
    }
    colorspace::safe_channels(doc, dict.get(b"ColorSpace").ok()?)
}

/// Gambar yang dipakai sebagai SMask/Mask gambar lain tidak boleh disentuh.
fn mask_targets(doc: &Document) -> HashSet<ObjectId> {
    doc.objects
        .values()
        .filter_map(|obj| obj.as_stream().ok())
        .flat_map(|s| [s.dict.get(b"SMask"), s.dict.get(b"Mask")])
        .filter_map(|v| v.ok()?.as_reference().ok())
        .collect()
}
