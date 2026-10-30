//! Halaman hasil tata letak → dokumen PDF (lopdf).

use std::collections::BTreeMap;
use std::fmt::Write as _;

use lopdf::{Dictionary, Document, Object, ObjectId, Stream, StringFormat};

use super::layout::{Op, Page, PageSetup};
use crate::fonts::Font;

pub fn write(pages: &[Page], images: Vec<Stream>, setup: &PageSetup) -> Document {
    let mut doc = Document::with_version("1.7");
    let pages_id = doc.new_object_id();

    // Hanya font yang benar-benar dipakai; nama F0, F1, … sesuai urutan enum
    let used: BTreeMap<Font, ObjectId> = pages
        .iter()
        .flat_map(|p| &p.ops)
        .filter_map(|op| match op {
            Op::Text { font, .. } => Some(*font),
            _ => None,
        })
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .map(|font| {
            let id = doc.add_object(Dictionary::from_iter([
                ("Type", Object::Name(b"Font".to_vec())),
                ("Subtype", Object::Name(b"Type1".to_vec())),
                ("BaseFont", Object::Name(font.base_name().as_bytes().to_vec())),
                ("Encoding", Object::Name(b"WinAnsiEncoding".to_vec())),
            ]));
            (font, id)
        })
        .collect();
    let font_name = |font: Font| format!("F{}", font as u8);
    let fonts = Dictionary::from_iter(used.iter().map(|(f, id)| (font_name(*f), Object::Reference(*id))));
    let xobjects =
        Dictionary::from_iter(images.into_iter().enumerate().map(|(i, s)| (format!("Im{i}"), Object::Reference(doc.add_object(s)))));
    let resources = doc.add_object(Dictionary::from_iter([("Font", Object::from(fonts)), ("XObject", Object::from(xobjects))]));

    let mut kids = Vec::with_capacity(pages.len());
    for page in pages {
        let mut content = String::new();
        let mut annots = Vec::new();
        for op in &page.ops {
            let _ = match op {
                Op::Text { x, y, font, size, bytes, color: [r, g, b] } => {
                    let hex: String = bytes.iter().map(|b| format!("{b:02X}")).collect();
                    writeln!(content, "BT /{} {size:.2} Tf {r} {g} {b} rg {x:.2} {y:.2} Td <{hex}> Tj ET", font_name(*font))
                }
                Op::Line { x1, y1, x2, y2, width, color: [r, g, b] } => {
                    writeln!(content, "{r} {g} {b} RG {width:.2} w {x1:.2} {y1:.2} m {x2:.2} {y2:.2} l S")
                }
                Op::Rect { x, y, w, h, color: [r, g, b] } => writeln!(content, "{r} {g} {b} rg {x:.2} {y:.2} {w:.2} {h:.2} re f"),
                Op::Image { index, x, y, w, h } => writeln!(content, "q {w:.2} 0 0 {h:.2} {x:.2} {y:.2} cm /Im{index} Do Q"),
                Op::Link { x, y, w, h, uri } => {
                    annots.push(Object::Reference(doc.add_object(link_annotation(*x, *y, *w, *h, uri))));
                    Ok(())
                }
            };
        }
        let content_id = doc.add_object(Stream::new(Dictionary::new(), content.into_bytes()));
        let mut dict = Dictionary::from_iter([
            ("Type", Object::Name(b"Page".to_vec())),
            ("Parent", Object::Reference(pages_id)),
            ("MediaBox", vec![0.into(), 0.into(), Object::Real(setup.width), Object::Real(setup.height)].into()),
            ("Resources", Object::Reference(resources)),
            ("Contents", Object::Reference(content_id)),
        ]);
        if !annots.is_empty() {
            dict.set("Annots", annots);
        }
        kids.push(Object::Reference(doc.add_object(dict)));
    }

    doc.objects.insert(
        pages_id,
        Dictionary::from_iter([
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Count", Object::Integer(kids.len() as i64)),
            ("Kids", Object::Array(kids)),
        ])
        .into(),
    );
    let catalog = doc.add_object(Dictionary::from_iter([
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
    ]));
    doc.trailer.set("Root", catalog);
    doc
}

/// Area link yang bisa diklik di penampil PDF.
fn link_annotation(x: f32, y: f32, w: f32, h: f32, uri: &str) -> Dictionary {
    let action = Dictionary::from_iter([
        ("S", Object::Name(b"URI".to_vec())),
        ("URI", Object::String(uri.as_bytes().to_vec(), StringFormat::Literal)),
    ]);
    Dictionary::from_iter([
        ("Type", Object::Name(b"Annot".to_vec())),
        ("Subtype", Object::Name(b"Link".to_vec())),
        ("Rect", vec![Object::Real(x), Object::Real(y), Object::Real(x + w), Object::Real(y + h)].into()),
        ("Border", vec![0.into(), 0.into(), 0.into()].into()),
        ("A", action.into()),
    ])
}
