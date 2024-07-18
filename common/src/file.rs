use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

/// The options for a file asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash)]
pub enum FileOptions {
    /// An image asset
    Image(ImageOptions),
    /// A video asset
    Video(VideoOptions),
    /// A font asset
    Font(FontOptions),
    /// A css asset
    Css(CssOptions),
    /// Any other asset
    Other(UnknownFileOptions),
}

impl Display for FileOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Image(options) => write!(f, "{}", options),
            Self::Video(options) => write!(f, "{}", options),
            Self::Font(options) => write!(f, "{}", options),
            Self::Css(options) => write!(f, "{}", options),
            Self::Other(options) => write!(f, "{}", options),
        }
    }
}

impl FileOptions {
    /// Returns the default options for a given extension
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
            _ => Self::Other(UnknownFileOptions {
                extension: extension.map(String::from),
            }),
        }
    }

    /// Returns the extension for this file
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
        Self::Other(UnknownFileOptions { extension: None })
    }
}

/// The options for an image asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash)]
pub struct ImageOptions {
    compress: bool,
    size: Option<(u32, u32)>,
    preload: bool,
    ty: ImageType,
}

impl Display for ImageOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((x, y)) = self.size {
            write!(f, "{} ({}x{})", self.ty, x, y)?;
        } else {
            write!(f, "{}", self.ty)?;
        }
        if self.compress {
            write!(f, " (compressed)")?;
        }
        if self.preload {
            write!(f, " (preload)")?;
        }
        Ok(())
    }
}

impl ImageOptions {
    /// Creates a new image options struct
    pub fn new(ty: ImageType, size: Option<(u32, u32)>) -> Self {
        Self {
            compress: true,
            size,
            ty,
            preload: false,
        }
    }

    /// Returns whether the image should be preloaded
    pub fn preload(&self) -> bool {
        self.preload
    }

    /// Sets whether the image should be preloaded
    pub fn set_preload(&mut self, preload: bool) {
        self.preload = preload;
    }

    /// Returns the image type
    pub fn ty(&self) -> &ImageType {
        &self.ty
    }

    /// Sets the image type
    pub fn set_ty(&mut self, ty: ImageType) {
        self.ty = ty;
    }

    /// Returns the size of the image
    pub fn size(&self) -> Option<(u32, u32)> {
        self.size
    }

    /// Sets the size of the image
    pub fn set_size(&mut self, size: Option<(u32, u32)>) {
        self.size = size;
    }

    /// Returns whether the image should be compressed
    pub fn compress(&self) -> bool {
        self.compress
    }

    /// Sets whether the image should be compressed
    pub fn set_compress(&mut self, compress: bool) {
        self.compress = compress;
    }
}

/// The type of an image
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Copy, Hash)]
pub enum ImageType {
    /// A png image
    Png,
    /// A jpg image
    Jpg,
    /// An avif image
    Avif,
    /// A webp image
    Webp,
}

impl Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Png => write!(f, "png"),
            Self::Jpg => write!(f, "jpg"),
            Self::Avif => write!(f, "avif"),
            Self::Webp => write!(f, "webp"),
        }
    }
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

/// The options for a video asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash)]
pub struct VideoOptions {
    /// Whether the video should be compressed
    compress: bool,
    /// Whether the video should be preloaded
    preload: bool,
    /// The type of the video
    ty: VideoType,
}

impl Display for VideoOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ty)?;
        if self.compress {
            write!(f, " (compressed)")?;
        }
        if self.preload {
            write!(f, " (preload)")?;
        }
        Ok(())
    }
}

impl VideoOptions {
    /// Creates a new video options struct
    pub fn new(ty: VideoType) -> Self {
        Self {
            compress: true,
            ty,
            preload: false,
        }
    }

    /// Returns the type of the video
    pub fn ty(&self) -> &VideoType {
        &self.ty
    }

    /// Sets the type of the video
    pub fn set_ty(&mut self, ty: VideoType) {
        self.ty = ty;
    }

    /// Returns whether the video should be compressed
    pub fn compress(&self) -> bool {
        self.compress
    }

    /// Sets whether the video should be compressed
    pub fn set_compress(&mut self, compress: bool) {
        self.compress = compress;
    }

    /// Returns whether the video should be preloaded
    pub fn preload(&self) -> bool {
        self.preload
    }

    /// Sets whether the video should be preloaded
    pub fn set_preload(&mut self, preload: bool) {
        self.preload = preload;
    }
}

/// The type of a video
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash)]
pub enum VideoType {
    /// An mp4 video
    MP4,
    /// A webm video
    Webm,
    /// A gif video
    GIF,
}

impl Display for VideoType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MP4 => write!(f, "mp4"),
            Self::Webm => write!(f, "webm"),
            Self::GIF => write!(f, "gif"),
        }
    }
}

/// The options for a font asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash)]
pub struct FontOptions {
    ty: FontType,
}

impl Display for FontOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ty)
    }
}

impl FontOptions {
    /// Creates a new font options struct
    pub fn new(ty: FontType) -> Self {
        Self { ty }
    }

    /// Returns the type of the font
    pub fn ty(&self) -> &FontType {
        &self.ty
    }
}

/// The type of a font
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash)]
pub enum FontType {
    /// A ttf (TrueType) font
    TTF,
    /// A woff (Web Open Font Format) font
    WOFF,
    /// A woff2 (Web Open Font Format 2) font
    WOFF2,
}

impl Display for FontType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TTF => write!(f, "ttf"),
            Self::WOFF => write!(f, "woff"),
            Self::WOFF2 => write!(f, "woff2"),
        }
    }
}

/// The options for a css asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash)]
pub struct CssOptions {
    minify: bool,
    preload: bool,
}

impl Default for CssOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for CssOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.minify {
            write!(f, "minified")?;
        }
        if self.preload {
            write!(f, " (preload)")?;
        }
        Ok(())
    }
}

impl CssOptions {
    /// Creates a new css options struct
    pub const fn new() -> Self {
        Self {
            minify: true,
            preload: false,
        }
    }

    /// Returns whether the css should be minified
    pub fn minify(&self) -> bool {
        self.minify
    }

    /// Sets whether the css should be minified
    pub fn set_minify(&mut self, minify: bool) {
        self.minify = minify;
    }

    /// Returns whether the css should be preloaded
    pub fn preload(&self) -> bool {
        self.preload
    }

    /// Sets whether the css should be preloaded
    pub fn set_preload(&mut self, preload: bool) {
        self.preload = preload;
    }
}

/// The options for an unknown file asset
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone, Hash)]
pub struct UnknownFileOptions {
    extension: Option<String>,
}

impl Display for UnknownFileOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(extension) = &self.extension {
            write!(f, "{}", extension)?;
        }
        Ok(())
    }
}

impl UnknownFileOptions {
    /// Creates a new unknown file options struct
    pub fn new(extension: Option<String>) -> Self {
        Self { extension }
    }

    /// Returns the extension of the file
    pub fn extension(&self) -> Option<&str> {
        self.extension.as_deref()
    }
}
