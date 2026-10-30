mod common;

use std::cmp::Ordering;

use pdf_core::files::{expand_paths, natural_cmp};

use common::temp_dir;

#[test]
fn natural_order_reads_numbers_as_numbers() {
    let mut names = vec!["scan10.jpg", "Scan2.jpg", "scan1.jpg", "scan02.jpg", "a.jpg"];
    names.sort_by(|a, b| natural_cmp(a, b));
    assert_eq!(names, ["a.jpg", "scan1.jpg", "Scan2.jpg", "scan02.jpg", "scan10.jpg"]);
    assert_eq!(natural_cmp("x", "x"), Ordering::Equal);
}

#[test]
fn expands_folders_filters_extensions_and_skips_hidden_files() {
    let dir = temp_dir("files-expand");
    let folder = dir.join("berkas");
    std::fs::create_dir(&folder).unwrap();
    for name in ["hal10.JPG", "hal2.png", "catatan.txt", ".DS_Store", "._hal3.jpg"] {
        std::fs::write(folder.join(name), b"x").unwrap();
    }
    let loose = dir.join("lepas.jpg");
    std::fs::write(&loose, b"x").unwrap();
    let extensions = vec!["jpg".to_string(), "png".to_string()];

    let result = expand_paths(&[folder.clone(), loose.clone(), dir.join("lain.pdf")], &extensions);

    assert_eq!(result.accepted, [folder.join("hal2.png"), folder.join("hal10.JPG"), loose]);
    assert_eq!(result.rejected, 2, "catatan.txt dan lain.pdf");
}
