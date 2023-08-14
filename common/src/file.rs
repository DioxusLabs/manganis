use image::DynamicImage;
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use serde::{Deserialize, Serialize};
use std::{path::{Path, PathBuf}, io::BufWriter};

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
            Self::Image(options) => {
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

    fn process_file(
        &self,
        input_location: &FileLocation,
        output_folder: &Path,
    ) -> std::io::Result<()> {
        let image = image::open(input_location.path()).unwrap();

        let mut output_location = output_folder.to_path_buf();

        match self.ty {
            ImageType::PNG => {
                output_location.push(input_location.unique_name());
                Self::compress_png(image, output_location);
            },
            ImageType::JPG => {
                output_location.push(input_location.unique_name());
                image.save(output_location).unwrap();
            },
            ImageType::Avif => {
                output_location.push(input_location.unique_name());
                image.save(output_location).unwrap();
            },
            ImageType::Webp => {
                output_location.push(input_location.unique_name());
                image.save(output_location).unwrap();
            },
        }

        Ok(())
    }

    fn compress_png(image: DynamicImage, output_location: PathBuf){
        // Image loading/saving is outside scope of this library
        let width = image.width() as usize;
        let height = image.height() as usize;
        let bitmap:Vec<_> = image.into_rgba8().pixels().map(|px|{
            imagequant::RGBA::new(px[0],px[1],px[2],px[3])
        }).collect();

        // Configure the library
        let mut liq = imagequant::new();
        liq.set_speed(5).unwrap();
        liq.set_quality(0, 99).unwrap();

        // Describe the bitmap
        let mut img = liq.new_image(&bitmap[..], width, height, 0.0).unwrap();

        // The magic happens in quantize()
        let mut res = match liq.quantize(&mut img) {
            Ok(res) => res,
            Err(err) => panic!("Quantization failed, because: {err:?}"),
        };

        let (palette, pixels) = res.remapped(&mut img).unwrap();

        let file  = std::fs::File::create(output_location).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        let mut flattened_palette =Vec::new();
        let mut alpha_palette =Vec::new();
        for px in palette {
            flattened_palette.push(px.r);
            flattened_palette.push(px.g);
            flattened_palette.push(px.b);
            alpha_palette.push(px.a);
        }
        encoder.set_palette(flattened_palette);
        encoder.set_trns(alpha_palette);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_compression(png::Compression::Best);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&pixels).unwrap();
        writer.finish().unwrap();
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

        let css = if self.minify { minify_css(&css) } else { css };

        let mut output_location = output_folder.to_path_buf();
        output_location.push(input_location.unique_name());
        std::fs::write(output_location, css)?;

        Ok(())
    }
}

pub(crate) fn minify_css(css: &str) -> String {
    let mut stylesheet = StyleSheet::parse(&css, ParserOptions::default()).unwrap();
    stylesheet.minify(MinifyOptions::default()).unwrap();
    let mut printer = PrinterOptions::default();
    printer.minify = true;
    let res = stylesheet.to_css(printer).unwrap();
    res.code
}
