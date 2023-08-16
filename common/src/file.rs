use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum FileOptions {
    Image(ImageOptions),
    Video(VideoOptions),
    Font(FontOptions),
    Css(CssOptions),
    Other(FileExtension),
}

impl FileOptions {
    pub fn default_for_extension(extension: Option<&str>) -> Self {
        match extension {
            Some("png") => Self::Image(ImageOptions::new(ImageType::Png, None)),
            Some("jpg") | Some("jpeg") => Self::Image(ImageOptions::new(ImageType::Jpg, None)),
            Some("avif") => Self::Image(ImageOptions::new(ImageType::Avif, None)),
            Some("webp") => Self::Image(ImageOptions::new(ImageType::Webp, None)),
            Some("mp4") => Self::Video(VideoOptions::new(VideoType::MP4)),
            Some("webm") => Self::Video(VideoOptions::new(VideoType::Webm)),
            Some("gif") => Self::Video(VideoOptions::new(VideoType::GIF)),
            Some("ttf") => Self::Font(FontOptions::new(FontType::TTF)),
            Some("woff") => Self::Font(FontOptions::new(FontType::WOFF)),
            Some("woff2") => Self::Font(FontOptions::new(FontType::WOFF2)),
            Some("css") => Self::Css(CssOptions::default()),
            _ => Self::Other(FileExtension {
                extension: extension.map(String::from),
            }),
        }
    }

    pub fn extension(&self) -> Option<&str> {
        match self {
            Self::Image(options) => match options.ty {
                ImageType::Png => Some("png"),
                ImageType::Jpg => Some("jpg"),
                ImageType::Avif => Some("avif"),
                ImageType::Webp => Some("webp"),
            },
            Self::Video(options) => match options.ty {
                VideoType::MP4 => Some("mp4"),
                VideoType::Webm => Some("webm"),
                VideoType::GIF => Some("gif"),
            },
            Self::Font(options) => match options.ty {
                FontType::TTF => Some("ttf"),
                FontType::WOFF => Some("woff"),
                FontType::WOFF2 => Some("woff2"),
            },
            Self::Css(_) => Some("css"),
            Self::Other(extension) => extension.extension.as_deref(),
        }
    }
}

impl Default for FileOptions {
    fn default() -> Self {
        Self::Other(FileExtension { extension: None })
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct ImageOptions {
    compress: bool,
    size: Option<(u32, u32)>,
    ty: ImageType,
}

impl ImageOptions {
    pub fn new(ty: ImageType, size: Option<(u32, u32)>) -> Self {
        Self {
            compress: true,
            size,
            ty,
        }
    }

    pub fn ty(&self) -> &ImageType {
        &self.ty
    }

    pub fn set_ty(&mut self, ty: ImageType) {
        self.ty = ty;
    }

    pub fn size(&self) -> Option<(u32, u32)> {
        self.size
    }

    pub fn set_size(&mut self, size: Option<(u32, u32)>) {
        self.size = size;
    }

    pub fn compress(&self) -> bool {
        self.compress
    }

    pub fn set_compress(&mut self, compress: bool) {
        self.compress = compress;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum ImageType {
    Png,
    Jpg,
    Avif,
    Webp,
}

impl FromStr for ImageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "png" => Ok(Self::Png),
            "jpg" | "jpeg" => Ok(Self::Jpg),
            "avif" => Ok(Self::Avif),
            "webp" => Ok(Self::Webp),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct VideoOptions {
    compress: bool,
    ty: VideoType,
}

impl VideoOptions {
    fn new(ty: VideoType) -> Self {
        Self { compress: true, ty }
    }

    pub fn ty(&self) -> &VideoType {
        &self.ty
    }

    pub fn set_ty(&mut self, ty: VideoType) {
        self.ty = ty;
    }

    pub fn compress(&self) -> bool {
        self.compress
    }

    pub fn set_compress(&mut self, compress: bool) {
        self.compress = compress;
    }
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

impl FontOptions {
    fn new(ty: FontType) -> Self {
        Self { ty }
    }

    pub fn ty(&self) -> &FontType {
        &self.ty
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum FontType {
    TTF,
    WOFF,
    WOFF2,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct CssOptions {
    minify: bool,
}

impl CssOptions {
    pub fn minify(&self) -> bool {
        self.minify
    }
}

impl Default for CssOptions {
    fn default() -> Self {
        Self { minify: true }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct FileExtension {
    extension: Option<String>,
}

impl FileExtension {
    pub fn extension(&self) -> Option<&str> {
        self.extension.as_deref()
    }
}
