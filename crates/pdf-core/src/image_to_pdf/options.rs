use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PageSize {
    /// Ukuran halaman mengikuti gambar
    Fit,
    A4,
    /// F4/Folio, ukuran kertas umum di Indonesia
    F4,
    Letter,
    Legal,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PageOrientation {
    /// Ikuti bentuk gambar (lebar → landscape)
    Auto,
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Margin {
    None,
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImageQuality {
    /// Tanpa resize; JPEG dimasukkan apa adanya, format lain lossless
    Original,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageToPdfOptions {
    pub page_size: PageSize,
    pub orientation: PageOrientation,
    pub margin: Margin,
    pub quality: ImageQuality,
}

impl PageSize {
    /// Ukuran portrait dalam mm; `None` untuk mode Fit.
    pub fn portrait_mm(self) -> Option<(f32, f32)> {
        match self {
            PageSize::Fit => None,
            PageSize::A4 => Some((210.0, 297.0)),
            PageSize::F4 => Some((215.0, 330.0)),
            PageSize::Letter => Some((215.9, 279.4)),
            PageSize::Legal => Some((215.9, 355.6)),
        }
    }
}

impl Margin {
    pub fn mm(self) -> f32 {
        match self {
            Margin::None => 0.0,
            Margin::Small => 10.0,
            Margin::Medium => 20.0,
            Margin::Large => 30.0,
        }
    }
}

impl ImageQuality {
    /// (sisi terpanjang px, kualitas JPEG); `None` = kualitas asli.
    pub fn limits(self) -> Option<(u32, u8)> {
        match self {
            ImageQuality::Original => None,
            ImageQuality::High => Some((3000, 90)),
            ImageQuality::Medium => Some((2000, 75)),
            ImageQuality::Low => Some((1400, 60)),
        }
    }
}
