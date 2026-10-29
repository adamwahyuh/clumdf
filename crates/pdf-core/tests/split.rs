mod common;

use pdf_core::split::{SplitMode, parse_ranges, plan};
use pdf_core::split_pdf;

use common::{page_labels, temp_dir, write_pdf};

fn no_progress(_: usize, _: usize) {}

#[test]
fn parses_ranges_singles_and_open_ends() {
    assert_eq!(parse_ranges("1-3, 5, 8-10", 10).unwrap(), [(1, 3), (5, 5), (8, 10)]);
    assert_eq!(parse_ranges("8-", 10).unwrap(), [(8, 10)]);
    assert_eq!(parse_ranges(" 2 – 4 ,, ", 10).unwrap(), [(2, 4)]);
}

#[test]
fn rejects_bad_ranges() {
    for bad in ["0", "11", "3-1", "abc", "1-x", "", " , "] {
        let err = parse_ranges(bad, 10).unwrap_err();
        assert_eq!(err.code(), "invalid_range", "input {bad:?}");
    }
}

#[test]
fn plans_every_n_pages_with_shorter_last_group() {
    let groups = plan(&SplitMode::EveryN { n: 3 }, 10).unwrap();
    assert_eq!(groups, [(1, 3), (4, 6), (7, 9), (10, 10)]);
    assert_eq!(plan(&SplitMode::EveryN { n: 0 }, 10).unwrap_err().code(), "invalid_range");
}

#[test]
fn split_each_page_into_named_files() {
    let dir = temp_dir("split-each");
    let input = write_pdf(&dir, "surat", 3);
    let out_dir = dir.join("out");
    std::fs::create_dir(&out_dir).unwrap();

    let report = split_pdf(&input, &out_dir, &SplitMode::EachPage, &no_progress).unwrap();

    let names: Vec<_> = report.files.iter().map(|f| f.file_name().unwrap().to_string_lossy().into_owned()).collect();
    assert_eq!(names, ["surat-1.pdf", "surat-2.pdf", "surat-3.pdf"]);
    for (i, file) in report.files.iter().enumerate() {
        assert_eq!(page_labels(file), [format!("surat-p{}", i + 1)]);
    }
}

#[test]
fn split_ranges_keeps_page_order_and_never_overwrites() {
    let dir = temp_dir("split-ranges");
    let input = write_pdf(&dir, "doc", 5);
    let out_dir = dir.join("out");
    std::fs::create_dir(&out_dir).unwrap();
    std::fs::write(out_dir.join("doc-2-4.pdf"), b"punya user").unwrap();
    let mode = SplitMode::Ranges { ranges: "2-4, 5".into() };

    let report = split_pdf(&input, &out_dir, &mode, &no_progress).unwrap();

    assert_eq!(report.files[0], out_dir.join("doc-2-4 (2).pdf"));
    assert_eq!(std::fs::read(out_dir.join("doc-2-4.pdf")).unwrap(), b"punya user");
    assert_eq!(page_labels(&report.files[0]), ["doc-p2", "doc-p3", "doc-p4"]);
    assert_eq!(page_labels(&report.files[1]), ["doc-p5"]);
}

#[test]
fn split_reports_bad_range_before_writing() {
    let dir = temp_dir("split-bad");
    let input = write_pdf(&dir, "doc", 2);
    let mode = SplitMode::Ranges { ranges: "1-9".into() };

    assert_eq!(split_pdf(&input, &dir, &mode, &no_progress).unwrap_err().code(), "invalid_range");
    assert!(!dir.join("doc-1-9.pdf").exists());
}
