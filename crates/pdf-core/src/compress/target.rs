//! Target ukuran file: turunkan resolusi & kualitas bertahap sampai muat.

use std::path::Path;

use lopdf::{Document, ObjectId};

use super::{CompressOptions, optimize};
use crate::error::PdfError;
use crate::progress::ProgressReporter;

/// (sisi terpanjang px, kualitas JPEG), dari paling ringan ke paling agresif.
/// Ukuran hasil hampir selalu turun di tiap anak tangga, jadi bisa binary search.
const LADDER: [(u32, u8); 10] = [
    (2400, 80),
    (2000, 75),
    (1600, 65),
    (1400, 58),
    (1200, 50),
    (1000, 45),
    (850, 40),
    (700, 35),
    (600, 30),
    (500, 25),
];

/// Cari anak tangga paling ringan yang hasilnya ≤ `target`.
/// Bila bahkan yang paling agresif tidak muat, kembalikan hasil terkecil itu.
pub(super) fn search(
    doc: &Document,
    candidates: &[(ObjectId, u8)],
    options: &CompressOptions,
    target: u64,
    original: &[u8],
    reporter: &impl ProgressReporter,
    output: &Path,
) -> Result<(Vec<u8>, usize), PdfError> {
    // Sudah muat: jangan turunkan kualitas tanpa perlu
    if original.len() as u64 <= target {
        return Ok((original.to_vec(), 0));
    }
    // 1 percobaan paling agresif + ceil(log2(10)) langkah binary search
    let max_runs = 1 + LADDER.len().next_power_of_two().trailing_zeros() as usize;
    let mut runs = 0;
    let mut run = |step: usize| {
        let (max_dimension, jpeg_quality) = LADDER[step];
        let options = CompressOptions { max_dimension, jpeg_quality, ..options.clone() };
        let result = optimize(doc.clone(), candidates, &options, &|| {}, output);
        runs += 1;
        reporter.report(runs, max_runs);
        result
    };

    // Tanpa gambar, kualitas tidak berpengaruh: cukup sekali
    if candidates.is_empty() {
        return run(0);
    }
    let mut best = run(LADDER.len() - 1)?;
    if best.0.len() as u64 > target {
        return Ok(best);
    }
    let (mut lo, mut hi) = (0, LADDER.len() - 1);
    while lo < hi {
        let mid = (lo + hi) / 2;
        let result = run(mid)?;
        if result.0.len() as u64 <= target {
            best = result;
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    Ok(best)
}
