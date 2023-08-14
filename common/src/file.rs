use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::FileLocation;

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum FileOptions {
    Image(ImageOptions),
    Video(VideoOptions),
    Font(FontOptions),
    Css(CssOptions),
    Other,
}

impl FileOptions {
    pub fn default_for_extension(extension: &str) -> Self {
        match extension {
            "png" => Self::Image(ImageOptions::new(ImageType::PNG)),
            "jpg" => Self::Image(ImageOptions::new(ImageType::JPG)),
            "avif" => Self::Image(ImageOptions::new(ImageType::Avif)),
            "webp" => Self::Image(ImageOptions::new(ImageType::Webp)),
            "mp4" => Self::Video(VideoOptions::new(VideoType::MP4)),
            "webm" => Self::Video(VideoOptions::new(VideoType::Webm)),
            "gif" => Self::Video(VideoOptions::new(VideoType::GIF)),
            "ttf" => Self::Font(FontOptions::new(FontType::TTF)),
            "woff" => Self::Font(FontOptions::new(FontType::WOFF)),
            "woff2" => Self::Font(FontOptions::new(FontType::WOFF2)),
            "css" => Self::Css(CssOptions::default()),
            _ => Self::Other,
        }
    }

    pub fn process_file(
        &self,
        input_location: &FileLocation,
        output_folder: &Path,
    ) -> std::io::Result<()> {
        match self {
            Self::Other => {
                let mut output_location = output_folder.to_path_buf();
                output_location.push(input_location.unique_name());
                std::fs::copy(input_location.path(), output_location)?;
            }
            Self::Css(options) => {
                options.process_file(input_location, output_folder)?;
            }
            _ => todo!(),
        }

        Ok(())
    }
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

impl ImageOptions {
    fn new(ty: ImageType) -> Self {
        Self { compress: true, ty }
    }
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

impl VideoOptions {
    fn new(ty: VideoType) -> Self {
        Self { compress: true, ty }
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

impl Default for CssOptions {
    fn default() -> Self {
        Self { minify: true }
    }
}

impl CssOptions {
    fn process_file(
        &self,
        input_location: &FileLocation,
        output_folder: &Path,
    ) -> std::io::Result<()> {
        let path = input_location.path();
        let css = std::fs::read_to_string(path)?;

        let css = if self.minify {
            minify_css(&css)
        }
        else {
            css
        };

        let mut output_location = output_folder.to_path_buf();
        output_location.push(input_location.unique_name());
        std::fs::write(output_location, css)?;

        Ok(())
    }
}

pub(crate)fn minify_css(css: &str) -> String {
    let mut stylesheet = StyleSheet::parse(&css, ParserOptions::default()).unwrap();
    stylesheet.minify(MinifyOptions::default()).unwrap();
    let mut printer = PrinterOptions::default();
    printer.minify = true;
    let res = stylesheet.to_css(printer).unwrap();
    res.code
}