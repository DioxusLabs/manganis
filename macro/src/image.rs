use base64::Engine;
use manganis_cli_support::process_file;
use manganis_common::{Config, FileAsset, FileOptions, FileSource, ImageOptions};
use quote::{quote, ToTokens};
use syn::{braced, parenthesized, parse::Parse};

use crate::add_asset;

struct ParseImageOptions {
    options: Vec<ParseImageOption>,
}

impl ParseImageOptions {
    fn apply_to_options(self, file: &mut FileAsset) {
        for option in self.options {
            option.apply_to_options(file);
        }
    }
}

impl Parse for ParseImageOptions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        braced!(inside in input);
        let mut options = Vec::new();
        while !inside.is_empty() {
            options.push(inside.parse::<ParseImageOption>()?);
            if !inside.is_empty() {
                let _ = inside.parse::<syn::Token![,]>()?;
            }
        }
        Ok(ParseImageOptions { options })
    }
}

enum ParseImageOption {
    Format(manganis_common::ImageType),
    Size((u32, u32)),
    Preload(bool),
    UrlEncoded(bool),
}

impl ParseImageOption {
    fn apply_to_options(self, file: &mut FileAsset) {
        if let FileOptions::Image(options) = file.options_mut() {
            match self {
                ParseImageOption::Format(format) => {
                    options.set_ty(format);
                }
                ParseImageOption::Size(size) => {
                    options.set_size(Some(size));
                }
                ParseImageOption::Preload(preload) => {
                    options.set_preload(preload);
                }
                ParseImageOption::UrlEncoded(url_encoded) => {
                    file.set_url_encoded(url_encoded);
                }
            }
        }
    }
}

impl Parse for ParseImageOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let _ = input.parse::<syn::Token![:]>()?;
        match ident.to_string().to_lowercase().as_str() {
            "format" => {
                let format = input.parse::<ImageType>()?;
                Ok(ParseImageOption::Format(format.into()))
            }
            "size" => {
                let size = input.parse::<ImageSize>()?;
                Ok(ParseImageOption::Size((size.width, size.height)))
            }
            "preload" => {
                let preload = input.parse::<syn::LitBool>()?;
                Ok(ParseImageOption::Preload(preload.value))
            }
            "url_encoded" => {
                let url_encoded = input.parse::<syn::LitBool>()?;
                Ok(ParseImageOption::UrlEncoded(url_encoded.value))
            }
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Unknown image option: {}. Supported options are format, size, preload, url_encoded",
                    ident
                ),
            )),
        }
    }
}

struct ImageSize {
    width: u32,
    height: u32,
}

impl Parse for ImageSize {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        parenthesized!(inside  in input);
        let width = inside.parse::<syn::LitInt>()?;
        let _ = inside.parse::<syn::Token![,]>()?;
        let height = inside.parse::<syn::LitInt>()?;
        Ok(ImageSize {
            width: width.base10_parse()?,
            height: height.base10_parse()?,
        })
    }
}

impl From<ImageType> for manganis_common::ImageType {
    fn from(val: ImageType) -> Self {
        match val {
            ImageType::Png => manganis_common::ImageType::Png,
            ImageType::Jpeg => manganis_common::ImageType::Jpg,
            ImageType::Webp => manganis_common::ImageType::Webp,
            ImageType::Avif => manganis_common::ImageType::Avif,
        }
    }
}

impl Parse for ImageType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        match ident.to_string().to_lowercase().as_str() {
            "png" => Ok(ImageType::Png),
            "jpeg" => Ok(ImageType::Jpeg),
            "webp" => Ok(ImageType::Webp),
            "avif" => Ok(ImageType::Avif),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Unknown image type: {}. Supported types are png, jpeg, webp, avif",
                    ident
                ),
            )),
        }
    }
}

enum ImageType {
    Png,
    Jpeg,
    Webp,
    Avif,
}

pub struct ImageAssetParser {
    file_name: String,
}

impl Parse for ImageAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<syn::LitStr>()?;

        let parsed_options = {
            let _ = input.parse::<syn::Token![,]>();
            if input.is_empty() {
                None
            } else {
                Some(input.parse::<ParseImageOptions>()?)
            }
        };

        let path_as_str = path.value();
        let path: FileSource = match path_as_str.parse() {
            Ok(path) => path,
            Err(_) => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to parse path: {}", path_as_str),
                ))
            }
        };
        let Some(extension) = path.extension() else {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to get extension from path: {}", path_as_str),
            ));
        };
        let Ok(extension) = extension.parse() else {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Failed to parse extension: {}, supported types are png, jpeg, webp, avif",
                    extension
                ),
            ));
        };
        let mut this_file = FileAsset::new(path).with_options(manganis_common::FileOptions::Image(
            ImageOptions::new(extension, None),
        ));
        if let Some(parsed_options) = parsed_options {
            parsed_options.apply_to_options(&mut this_file);
        }

        let asset = add_asset(manganis_common::AssetType::File(this_file.clone()));
        let this_file = match asset {
            manganis_common::AssetType::File(this_file) => this_file,
            _ => unreachable!(),
        };
        let file_name = if this_file.url_encoded() {
            let target_directory =
                std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
            let output_folder = std::path::Path::new(&target_directory)
                .join("manganis")
                .join("assets");
            std::fs::create_dir_all(&output_folder).map_err(|e| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to create output folder: {}", e),
                )
            })?;
            process_file(&this_file, &output_folder).map_err(|e| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to process file: {}", e),
                )
            })?;
            let file = output_folder.join(this_file.location().unique_name());
            let data = std::fs::read(file).map_err(|e| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to read file: {}", e),
                )
            })?;
            let data = base64::engine::general_purpose::STANDARD_NO_PAD.encode(data);
            let mime = this_file.location().source().mime_type().unwrap();
            format!("data:{mime};base64,{data}")
        } else {
            this_file.served_location()
        };

        Ok(ImageAssetParser { file_name })
    }
}

impl ToTokens for ImageAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let file_name = &self.file_name;

        tokens.extend(quote! {
            #file_name
        })
    }
}
