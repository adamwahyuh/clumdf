use image::codecs::jpeg::JpegEncoder;
use image::{ColorType, DynamicImage, ImageResult};
use lopdf::{Dictionary, Object, Stream};

/// Gambar yang siap dijadikan image XObject PDF.
pub struct Encoded {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub gray: bool,
    /// true: `bytes` berisi JPEG (DCTDecode); false: sampel mentah 8-bit
    pub jpeg: bool,
}

fn is_gray(img: &DynamicImage) -> bool {
    matches!(img.color(), ColorType::L8 | ColorType::L16 | ColorType::La8 | ColorType::La16)
}

pub fn encode_jpeg(img: &DynamicImage, quality: u8) -> ImageResult<Encoded> {
    let gray = is_gray(img);
    let mut bytes = Vec::new();
    let encoder = JpegEncoder::new_with_quality(&mut bytes, quality.clamp(1, 100));
    if gray {
        img.to_luma8().write_with_encoder(encoder)?;
    } else {
        img.to_rgb8().write_with_encoder(encoder)?;
    }
    Ok(Encoded { bytes, width: img.width(), height: img.height(), gray, jpeg: true })
}

/// Lossless: sampel mentah, nanti dikompres Flate oleh `into_stream`.
pub fn encode_lossless(img: &DynamicImage) -> Encoded {
    let gray = is_gray(img);
    let bytes = if gray { img.to_luma8().into_raw() } else { img.to_rgb8().into_raw() };
    Encoded { bytes, width: img.width(), height: img.height(), gray, jpeg: false }
}

impl Encoded {
    pub fn into_stream(self) -> Stream {
        let color_space = if self.gray { "DeviceGray" } else { "DeviceRGB" };
        let mut dict = Dictionary::from_iter([
            ("Type", Object::Name(b"XObject".to_vec())),
            ("Subtype", Object::Name(b"Image".to_vec())),
            ("Width", Object::Integer(self.width.into())),
            ("Height", Object::Integer(self.height.into())),
            ("ColorSpace", Object::Name(color_space.as_bytes().to_vec())),
            ("BitsPerComponent", Object::Integer(8)),
        ]);
        if self.jpeg {
            dict.set("Filter", Object::Name(b"DCTDecode".to_vec()));
            // JPEG sudah terkompresi; jangan dibungkus Flate lagi oleh doc.compress()
            return Stream::new(dict, self.bytes).with_compression(false);
        }
        let mut stream = Stream::new(dict, self.bytes);
        // Jika gagal, stream tetap valid (tanpa kompresi), hanya lebih besar
        let _ = stream.compress();
        stream
    }
}
