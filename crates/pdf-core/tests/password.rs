mod common;

use pdf_core::{inspect, protect_pdf, unlock_pdf};

use common::{page_labels, temp_dir, write_pdf};

#[test]
fn protect_then_unlock_round_trip() {
    let dir = temp_dir("password-roundtrip");
    let input = write_pdf(&dir, "rahasia", 2);
    let locked = dir.join("locked.pdf");
    let unlocked = dir.join("unlocked.pdf");

    protect_pdf(&input, &locked, "s3cret!").unwrap();

    // V4 (AES-128): output V5 lopdf tidak terbaca viewer lain, jangan kembali ke V5
    let doc = lopdf::Document::load_with_options(&locked, lopdf::LoadOptions::with_password("s3cret!")).unwrap();
    let encrypt = doc.encryption_state.as_ref().map(|s| s.version());
    assert_eq!(encrypt, Some(4));

    // Terkunci: tidak bisa dibuka tanpa password dan isinya tidak terbaca polos
    assert_eq!(inspect(&locked).unwrap_err().code(), "encrypted");
    let raw = std::fs::read(&locked).unwrap();
    assert!(!String::from_utf8_lossy(&raw).contains("rahasia-p1"));

    unlock_pdf(&locked, &unlocked, "s3cret!").unwrap();

    assert_eq!(inspect(&unlocked).unwrap().page_count, 2);
    assert_eq!(page_labels(&unlocked), ["rahasia-p1", "rahasia-p2"]);
}

#[test]
fn wrong_password_is_reported() {
    let dir = temp_dir("password-wrong");
    let input = write_pdf(&dir, "a", 1);
    let locked = dir.join("locked.pdf");
    protect_pdf(&input, &locked, "benar").unwrap();
    let out = dir.join("out.pdf");

    assert_eq!(unlock_pdf(&locked, &out, "salah").unwrap_err().code(), "wrong_password");
    assert!(!out.exists());
}

#[test]
fn unlocking_a_plain_pdf_says_it_is_not_locked() {
    let dir = temp_dir("password-plain");
    let input = write_pdf(&dir, "a", 1);
    assert_eq!(unlock_pdf(&input, &dir.join("out.pdf"), "apa saja").unwrap_err().code(), "not_encrypted");
}

#[test]
fn empty_password_is_rejected() {
    let dir = temp_dir("password-empty");
    let input = write_pdf(&dir, "a", 1);
    assert_eq!(protect_pdf(&input, &dir.join("out.pdf"), "").unwrap_err().code(), "empty_password");
}
