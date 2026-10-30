//! Daftar file dari folder, untuk memproses satu folder sekaligus.

use std::cmp::Ordering;
use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpandedPaths {
    /// File dengan ekstensi yang didukung, folder sudah dibuka isinya
    pub accepted: Vec<PathBuf>,
    /// Jumlah file yang dilewati karena formatnya tidak didukung
    pub rejected: usize,
}

/// Buka folder (tidak rekursif) dan saring berdasarkan ekstensi.
/// Isi folder diurutkan alami: "scan2" sebelum "scan10", seperti di Finder.
pub fn expand_paths(paths: &[PathBuf], extensions: &[String]) -> ExpandedPaths {
    let matches = |p: &Path| {
        p.extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| extensions.iter().any(|x| x.eq_ignore_ascii_case(e)))
    };
    let mut result = ExpandedPaths::default();

    for path in paths {
        if path.is_dir() {
            let Ok(entries) = std::fs::read_dir(path) else { continue };
            let mut files: Vec<PathBuf> = entries
                .filter_map(|e| e.ok().map(|e| e.path()))
                // File tersembunyi (mis. ".DS_Store", "._foto.jpg" dari macOS) bukan milik user
                .filter(|p| p.is_file() && !p.file_name().is_some_and(|n| n.to_string_lossy().starts_with('.')))
                .collect();
            files.sort_by(|a, b| natural_cmp(&a.to_string_lossy(), &b.to_string_lossy()));
            for file in files {
                if matches(&file) {
                    result.accepted.push(file);
                } else {
                    result.rejected += 1;
                }
            }
        } else if matches(path) {
            result.accepted.push(path.clone());
        } else {
            result.rejected += 1;
        }
    }
    result
}

/// Bandingkan teks dengan angka dibaca sebagai bilangan, tanpa beda huruf besar/kecil.
pub fn natural_cmp(a: &str, b: &str) -> Ordering {
    let (mut a, mut b) = (a.chars().peekable(), b.chars().peekable());
    loop {
        match (a.peek().copied(), b.peek().copied()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) if x.is_ascii_digit() && y.is_ascii_digit() => {
                let take = |it: &mut std::iter::Peekable<std::str::Chars>| {
                    let mut digits = String::new();
                    while let Some(c) = it.next_if(char::is_ascii_digit) {
                        digits.push(c);
                    }
                    digits
                };
                let (da, db) = (take(&mut a), take(&mut b));
                let (ta, tb) = (da.trim_start_matches('0'), db.trim_start_matches('0'));
                let order = ta.len().cmp(&tb.len()).then_with(|| ta.cmp(tb)).then_with(|| da.len().cmp(&db.len()));
                if order != Ordering::Equal {
                    return order;
                }
            }
            (Some(x), Some(y)) => {
                let order = x.to_lowercase().cmp(y.to_lowercase());
                if order != Ordering::Equal {
                    return order;
                }
                a.next();
                b.next();
            }
        }
    }
}
