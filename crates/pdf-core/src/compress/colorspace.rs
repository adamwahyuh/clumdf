use lopdf::{Document, Object};

/// Jumlah channel bila ruang warna aman diproses (Gray = 1, RGB = 3).
/// CMYK, Indexed, Lab, dll. → `None` supaya gambar dilewati.
pub fn safe_channels(doc: &Document, color_space: &Object) -> Option<u8> {
    match color_space {
        Object::Name(name) => match name.as_slice() {
            b"DeviceGray" | b"G" => Some(1),
            b"DeviceRGB" | b"RGB" => Some(3),
            _ => None,
        },
        Object::Reference(id) => safe_channels(doc, doc.get_object(*id).ok()?),
        // [/ICCBased <stream>] dengan N = 1 atau 3
        Object::Array(items) if items.len() == 2 && items[0].as_name().ok() == Some(b"ICCBased") => {
            let id = items[1].as_reference().ok()?;
            let profile = doc.get_object(id).ok()?.as_stream().ok()?;
            match profile.dict.get(b"N").and_then(Object::as_i64).ok()? {
                1 => Some(1),
                3 => Some(3),
                _ => None,
            }
        }
        _ => None,
    }
}
