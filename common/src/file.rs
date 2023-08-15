use image::{DynamicImage, EncodableLayout};
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::FileLocation;

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

    pub fn process_file(
        &self,
        input_location: &FileLocation,
        output_folder: &Path,
    ) -> std::io::Result<()> {
        match self {
            Self::Other { .. } => {
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

    pub fn set_ty(&mut self, ty: ImageType) {
        self.ty = ty;
    }

    pub fn set_size(&mut self, size: Option<(u32, u32)>) {
        self.size = size;
    }

    fn process_file(
        &self,
        input_location: &FileLocation,
        output_folder: &Path,
    ) -> std::io::Result<()> {
        let mut image = image::open(input_location.path()).unwrap();

        if let Some(size) = self.size {
            image = image.resize_exact(size.0, size.1, image::imageops::FilterType::Lanczos3);
        }

        let mut output_location = output_folder.to_path_buf();

        match self.ty {
            ImageType::Png => {
                output_location.push(input_location.unique_name());
                Self::compress_png(image, output_location);
            }
            ImageType::Jpg => {
                output_location.push(input_location.unique_name());
                Self::compress_jpg(image, output_location);
            }
            ImageType::Avif => {
                output_location.push(input_location.unique_name());
                image.save(output_location).unwrap();
            }
            ImageType::Webp => {
                output_location.push(input_location.unique_name());
                image.save(output_location).unwrap();
            }
        }

        Ok(())
    }

    fn compress_jpg(image: DynamicImage, output_location: PathBuf) {
        let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_EXT_RGBX);
        let width = image.width() as usize;
        let height = image.height() as usize;

        comp.set_size(width, height);
        comp.set_optimize_scans(true);
        comp.set_mem_dest();
        comp.start_compress();

        comp.write_scanlines(image.to_rgba8().as_bytes());

        comp.finish_compress();
        let jpeg_bytes = comp.data_to_vec().unwrap();

        let file = std::fs::File::create(output_location).unwrap();
        let w = &mut BufWriter::new(file);
        w.write_all(&jpeg_bytes).unwrap();
    }

    fn compress_png(image: DynamicImage, output_location: PathBuf) {
        // Image loading/saving is outside scope of this library
        let width = image.width() as usize;
        let height = image.height() as usize;
        let bitmap: Vec<_> = image
            .into_rgba8()
            .pixels()
            .map(|px| imagequant::RGBA::new(px[0], px[1], px[2], px[3]))
            .collect();

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

        let file = std::fs::File::create(output_location).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        let mut flattened_palette = Vec::new();
        let mut alpha_palette = Vec::new();
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

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct FileExtension {
    extension: Option<String>,
}
