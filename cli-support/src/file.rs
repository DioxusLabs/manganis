use anyhow::Context;
use image::{DynamicImage, EncodableLayout};
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use manganis_common::{CssOptions, FileAsset, FileLocation, FileOptions, ImageOptions, ImageType};
use std::{
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

pub trait Process {
    fn process(&self, input_location: &FileLocation, output_folder: &Path) -> anyhow::Result<()>;
}

/// Process a specific file asset
pub fn process_file(file: &FileAsset, output_folder: &Path) -> anyhow::Result<()> {
    file.options().process(file.location(), output_folder)
}

impl Process for FileOptions {
    fn process(&self, input_location: &FileLocation, output_folder: &Path) -> anyhow::Result<()> {
        match self {
            Self::Other { .. } => {
                let mut output_location = output_folder.to_path_buf();
                output_location.push(input_location.unique_name());
                let bytes = input_location.read_to_bytes()?;
                std::fs::write(&output_location, bytes).with_context(|| {
                    format!(
                        "Failed to write file to output location: {}",
                        output_location.display()
                    )
                })?;
            }
            Self::Css(options) => {
                options.process(input_location, output_folder)?;
            }
            Self::Image(options) => {
                options.process(input_location, output_folder)?;
            }
            _ => todo!(),
        }

        Ok(())
    }
}

impl Process for ImageOptions {
    fn process(&self, input_location: &FileLocation, output_folder: &Path) -> anyhow::Result<()> {
        let mut image = image::io::Reader::new(std::io::Cursor::new(
            &*input_location.read_to_bytes().unwrap(),
        ))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

        if let Some(size) = self.size() {
            image = image.resize_exact(size.0, size.1, image::imageops::FilterType::Lanczos3);
        }

        let mut output_location = output_folder.to_path_buf();

        match self.ty() {
            ImageType::Png => {
                output_location.push(input_location.unique_name());
                compress_png(image, output_location);
            }
            ImageType::Jpg => {
                output_location.push(input_location.unique_name());
                compress_jpg(image, output_location);
            }
            ImageType::Avif => {
                output_location.push(input_location.unique_name());
                if let Err(error) = image.save(&output_location) {
                    tracing::error!("Failed to save avif image: {} with path {}. You must have the avif feature enabled to use avif assets", error, output_location.display());
                }
            }
            ImageType::Webp => {
                output_location.push(input_location.unique_name());
                if let Err(err) = image.save(output_location) {
                    tracing::error!("Failed to save webp image: {}. You must have the avif feature enabled to use webp assets", err);
                }
            }
        }

        Ok(())
    }
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

impl Process for CssOptions {
    fn process(&self, input_location: &FileLocation, output_folder: &Path) -> anyhow::Result<()> {
        let css = input_location.read_to_string()?;

        let css = if self.minify() { minify_css(&css) } else { css };

        let mut output_location = output_folder.to_path_buf();
        output_location.push(input_location.unique_name());
        std::fs::write(&output_location, css).with_context(|| {
            format!(
                "Failed to write css to output location: {}",
                output_location.display()
            )
        })?;

        Ok(())
    }
}

pub(crate) fn minify_css(css: &str) -> String {
    let mut stylesheet = StyleSheet::parse(css, ParserOptions::default()).unwrap();
    stylesheet.minify(MinifyOptions::default()).unwrap();
    let printer = PrinterOptions {
        minify: true,
        ..Default::default()
    };
    let res = stylesheet.to_css(printer).unwrap();
    res.code
}
