use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum FileOptions {
    Image(ImageOptions),
    Video(VideoOptions),
    Font(FontOptions),
    Css(CssOptions),
    Other,
}

impl Default for FileOptions {
    fn default() -> Self {
        Self::Other
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct ImageOptions {
    compress: bool,
    ty: ImageType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum ImageType {
    PNG,
    JPG,
    Avif,
    Webp,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct VideoOptions {
    compress: bool,
    ty: VideoType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum VideoType {
    MP4,
    Webm,
    GIF,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct FontOptions {
    ty: FontType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum FontType {
    TTF,
    WOFF,
    WOFF2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct CssOptions {
    compress: bool,
}
