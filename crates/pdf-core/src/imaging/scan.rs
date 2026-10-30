//! Mode scan dokumen: foto HP → hitam-putih bersih, terpotong, dan lurus.

use image::imageops::{self, FilterType};
use image::{DynamicImage, GrayImage, Luma};

/// Ukuran kerja untuk analisis (potong & luruskan); cukup akurat dan cepat.
const ANALYSIS_PX: u32 = 600;
/// Batas pencarian kemiringan (derajat); foto dokumen jarang lebih miring dari ini.
const MAX_SKEW: f32 = 10.0;

/// Pipeline lengkap: abu-abu → potong ke kertas → ratakan cahaya & kontras →
/// putihkan sisa meja → luruskan.
///
/// Cahaya diratakan sebelum sisa meja dicat putih: area putih buatan akan
/// terbaca sebagai "latar terang" dan membuat kertas berbayang tampak abu-abu.
pub fn clean_document(img: &DynamicImage) -> GrayImage {
    let gray = img.to_luma8();
    let gray = whiten_background(&normalize_lighting(&crop_to_paper(&gray)));
    let angle = detect_skew(&gray);
    if angle.abs() >= 0.3 {
        rotate(&gray, angle)
    } else {
        gray
    }
}

/// Ambang Otsu: piksel ≤ t gelap (tinta/meja), > t terang (kertas).
pub fn otsu_threshold(img: &GrayImage) -> u8 {
    let mut histogram = [0u64; 256];
    img.pixels().for_each(|p| histogram[p.0[0] as usize] += 1);
    let total = img.len() as f64;
    let sum: f64 = histogram
        .iter()
        .enumerate()
        .map(|(i, &c)| i as f64 * c as f64)
        .sum();
    let (mut weight_bg, mut sum_bg, mut best, mut threshold) = (0.0, 0.0, 0.0, 128u8);
    for (t, &count) in histogram.iter().enumerate() {
        weight_bg += count as f64;
        if weight_bg == 0.0 || weight_bg == total {
            continue;
        }
        sum_bg += t as f64 * count as f64;
        let mean_bg = sum_bg / weight_bg;
        let mean_fg = (sum - sum_bg) / (total - weight_bg);
        let between = weight_bg * (total - weight_bg) * (mean_bg - mean_fg).powi(2);
        if between > best {
            (best, threshold) = (between, t as u8);
        }
    }
    threshold
}

fn analysis_copy(img: &GrayImage) -> (GrayImage, f32) {
    let scale = (ANALYSIS_PX as f32 / img.width().max(img.height()) as f32).min(1.0);
    let (w, h) = (
        ((img.width() as f32 * scale) as u32).max(1),
        ((img.height() as f32 * scale) as u32).max(1),
    );
    (imageops::resize(img, w, h, FilterType::Triangle), scale)
}

/// Buang latar gelap (meja) di sekitar kertas yang terang.
/// Bila tidak ada latar gelap yang jelas, gambar dikembalikan apa adanya.
pub fn crop_to_paper(img: &GrayImage) -> GrayImage {
    let (small, scale) = analysis_copy(img);
    let t = otsu_threshold(&small);
    let (w, h) = small.dimensions();
    let bright = |x: u32, y: u32| small.get_pixel(x, y).0[0] > t;
    // Sengaja longgar (10% terang): kertas miring membuat kolom/baris di sudutnya
    // hanya sedikit terang. Sisa meja yang ikut terpotong dicat putih oleh
    // `whiten_background`; memotong terlalu ketat justru bisa memotong teks.
    let is_paper_col = |x: u32| (0..h).filter(|&y| bright(x, y)).count() as f32 > h as f32 * 0.1;
    let is_paper_row = |y: u32| (0..w).filter(|&x| bright(x, y)).count() as f32 > w as f32 * 0.1;
    let (Some(x0), Some(x1)) = (
        (0..w).find(|&x| is_paper_col(x)),
        (0..w).rev().find(|&x| is_paper_col(x)),
    ) else {
        return img.clone();
    };
    let (Some(y0), Some(y1)) = (
        (0..h).find(|&y| is_paper_row(y)),
        (0..h).rev().find(|&y| is_paper_row(y)),
    ) else {
        return img.clone();
    };
    let area = ((x1 - x0 + 1) * (y1 - y0 + 1)) as f32 / (w * h) as f32;
    // Terlalu kecil = kemungkinan salah deteksi; hampir penuh = memang tidak ada latar
    if !(0.2..0.97).contains(&area) {
        return img.clone();
    }
    let to_full = |v: u32| (v as f32 / scale) as u32;
    let (fx0, fy0) = (to_full(x0), to_full(y0));
    let (fx1, fy1) = (
        to_full(x1 + 1).min(img.width()),
        to_full(y1 + 1).min(img.height()),
    );
    imageops::crop_imm(img, fx0, fy0, fx1 - fx0, fy1 - fy0).to_image()
}

/// Kertas miring menyisakan sudut meja gelap di tepi potongan. Piksel gelap yang
/// tersambung ke tepi gambar adalah meja (tinta ada di dalam kertas), jadi dicat putih.
pub fn whiten_background(img: &GrayImage) -> GrayImage {
    let t = otsu_threshold(&analysis_copy(img).0);
    let (w, h) = img.dimensions();
    let mut out = img.clone();
    let mut stack: Vec<(u32, u32)> = (0..w)
        .flat_map(|x| [(x, 0), (x, h - 1)])
        .chain((0..h).flat_map(|y| [(0, y), (w - 1, y)]))
        .collect();
    while let Some((x, y)) = stack.pop() {
        if out.get_pixel(x, y).0[0] > t {
            continue; // kertas, atau sudah dicat
        }
        out.put_pixel(x, y, Luma([255]));
        if x > 0 {
            stack.push((x - 1, y));
        }
        if x + 1 < w {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y + 1 < h {
            stack.push((x, y + 1));
        }
    }
    out
}

/// Kemiringan baris teks (derajat; positif = turun ke kanan), dicari dengan
/// proyeksi baris: sudut yang benar membuat baris teks menumpuk paling tajam.
pub fn detect_skew(img: &GrayImage) -> f32 {
    let (small, _) = analysis_copy(img);
    let t = otsu_threshold(&small);
    let dark: Vec<(f32, f32)> = small
        .enumerate_pixels()
        .filter(|(_, _, p)| p.0[0] <= t)
        .map(|(x, y, _)| (x as f32, y as f32))
        .collect();
    if dark.len() < 50 {
        return 0.0;
    }
    let sharpness = |angle: f32| {
        let (sin, cos) = angle.to_radians().sin_cos();
        let mut rows = std::collections::HashMap::<i32, u32>::new();
        for &(x, y) in &dark {
            *rows.entry((y * cos - x * sin).round() as i32).or_default() += 1;
        }
        rows.values().map(|&c| (c as u64).pow(2)).sum::<u64>()
    };
    let best_in = |from: f32, to: f32, step: f32| {
        let steps = ((to - from) / step).round() as i32;
        (0..=steps)
            .map(|i| from + i as f32 * step)
            .max_by_key(|&a| sharpness(a))
            .unwrap_or(0.0)
    };
    let coarse = best_in(-MAX_SKEW, MAX_SKEW, 0.5);
    best_in(coarse - 0.5, coarse + 0.5, 0.1)
}

/// Putar balik sebesar `angle` (derajat) supaya baris teks mendatar; tepi baru diisi putih.
pub fn rotate(img: &GrayImage, angle: f32) -> GrayImage {
    let (sin, cos) = angle.to_radians().sin_cos();
    let (w, h) = img.dimensions();
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    GrayImage::from_fn(w, h, |u, v| {
        let (du, dv) = (u as f32 - cx, v as f32 - cy);
        let (x, y) = (du * cos - dv * sin + cx, du * sin + dv * cos + cy);
        if x < 0.0 || y < 0.0 || x >= (w - 1) as f32 || y >= (h - 1) as f32 {
            return Luma([255]);
        }
        // Interpolasi bilinear supaya huruf tidak bergerigi
        let (x0, y0) = (x as u32, y as u32);
        let (fx, fy) = (x - x0 as f32, y - y0 as f32);
        let p = |x: u32, y: u32| img.get_pixel(x, y).0[0] as f32;
        let top = p(x0, y0) * (1.0 - fx) + p(x0 + 1, y0) * fx;
        let bottom = p(x0, y0 + 1) * (1.0 - fx) + p(x0 + 1, y0 + 1) * fx;
        Luma([(top * (1.0 - fy) + bottom * fy).round() as u8])
    })
}

/// Hilangkan bayangan & cahaya tak rata, lalu tegaskan kontras: kertas putih, tinta hitam.
pub fn normalize_lighting(img: &GrayImage) -> GrayImage {
    let (w, h) = img.dimensions();
    // Latar = versi kecil yang tintanya "dihapus" dengan filter maksimum, lalu diperbesar lagi
    let mut background =
        imageops::resize(img, (w / 16).max(1), (h / 16).max(1), FilterType::Triangle);
    for _ in 0..2 {
        background = max_filter(&background);
    }
    let background = imageops::resize(&background, w, h, FilterType::Triangle);

    const INK: u32 = 60;
    const PAPER: u32 = 220;
    GrayImage::from_fn(w, h, |x, y| {
        let pixel = img.get_pixel(x, y).0[0] as u32;
        let bg = (background.get_pixel(x, y).0[0] as u32).max(1);
        let flat = (pixel * 255 / bg).min(255);
        Luma([((flat.saturating_sub(INK)) * 255 / (PAPER - INK)).min(255) as u8])
    })
}

fn max_filter(img: &GrayImage) -> GrayImage {
    let (w, h) = img.dimensions();
    GrayImage::from_fn(w, h, |x, y| {
        let mut max = 0;
        for ny in y.saturating_sub(1)..=(y + 1).min(h - 1) {
            for nx in x.saturating_sub(1)..=(x + 1).min(w - 1) {
                max = max.max(img.get_pixel(nx, ny).0[0]);
            }
        }
        Luma([max])
    })
}
