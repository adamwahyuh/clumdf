use image::{GrayImage, Luma};
use pdf_core::imaging::scan::{
    crop_to_paper, detect_skew, normalize_lighting, rotate, whiten_background,
};

/// Halaman putih dengan "baris teks" hitam setiap 40 px, miring `angle` derajat.
fn text_page(width: u32, height: u32, angle: f32) -> GrayImage {
    let slope = angle.to_radians().tan();
    GrayImage::from_fn(width, height, |x, y| {
        let line_y = y as f32 - x as f32 * slope;
        let in_line = line_y.rem_euclid(40.0) < 8.0 && (100..width - 100).contains(&x);
        Luma([if in_line { 20 } else { 235 }])
    })
}

#[test]
fn detects_skew_of_text_lines() {
    for angle in [-6.0f32, 0.0, 4.0] {
        let detected = detect_skew(&text_page(1200, 1600, angle));
        assert!(
            (detected - angle).abs() <= 0.3,
            "miring {angle}°, terdeteksi {detected}°"
        );
    }
}

#[test]
fn rotating_back_straightens_the_page() {
    let page = text_page(1200, 1600, 5.0);
    let straight = rotate(&page, detect_skew(&page));
    assert!(detect_skew(&straight).abs() <= 0.3);
}

#[test]
fn crops_dark_table_around_paper() {
    // Meja gelap 1000x1400, kertas terang 600x900 di posisi (200, 250)
    let photo = GrayImage::from_fn(1000, 1400, |x, y| {
        let on_paper = (200..800).contains(&x) && (250..1150).contains(&y);
        Luma([if on_paper { 230 } else { 45 }])
    });
    let paper = crop_to_paper(&photo);
    let (w, h) = paper.dimensions();
    assert!(
        (590..=612).contains(&w) && (890..=912).contains(&h),
        "hasil potong {w}x{h}"
    );
}

#[test]
fn page_without_background_is_not_cropped() {
    let page = text_page(800, 1000, 0.0);
    assert_eq!(crop_to_paper(&page).dimensions(), (800, 1000));
}

#[test]
fn removes_shadow_and_keeps_text_dark() {
    // Kertas dengan bayangan: kiri gelap (140), kanan terang (235); teks hitam di kedua sisi
    let photo = GrayImage::from_fn(800, 600, |x, y| {
        let paper = 140 + (x * 95 / 799) as u8;
        let ink = (y % 50) < 6 && (x % 200) > 20;
        Luma([if ink { paper / 6 } else { paper }])
    });
    let clean = normalize_lighting(&photo);
    for x in [30, 430, 770] {
        assert!(
            clean.get_pixel(x, 25).0[0] >= 245,
            "kertas di x={x} harus putih: {}",
            clean.get_pixel(x, 25).0[0]
        );
        assert!(
            clean.get_pixel(x, 2).0[0] <= 60,
            "teks di x={x} harus gelap: {}",
            clean.get_pixel(x, 2).0[0]
        );
    }
}

#[test]
fn table_corners_become_white_but_ink_stays() {
    // Sudut kiri-atas gelap (sisa meja) tersambung ke tepi; garis tinta di tengah tidak
    let img = GrayImage::from_fn(300, 300, |x, y| {
        if x + y < 60 {
            Luma([40])
        } else if (140..160).contains(&y) && (50..250).contains(&x) {
            Luma([20])
        } else {
            Luma([230])
        }
    });
    let out = whiten_background(&img);
    assert_eq!(out.get_pixel(5, 5).0[0], 255, "sudut meja harus putih");
    assert_eq!(out.get_pixel(150, 150).0[0], 20, "tinta tetap");
}
