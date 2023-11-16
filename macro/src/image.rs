use base64::Engine;
use manganis_cli_support::process_file;
use manganis_common::{FileAsset, FileOptions, FileSource, ImageOptions};
use quote::{quote, ToTokens};
use syn::{braced, parenthesized, parse::Parse};

use crate::add_asset;

struct ParseImageOptions {
    options: Vec<ParseImageOption>,
}

impl ParseImageOptions {
    fn apply_to_options(self, file: &mut FileAsset, low_quality_preview: &mut bool) {
        for option in self.options {
            option.apply_to_options(file, low_quality_preview);
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
    Lqip(bool),
}

impl ParseImageOption {
    fn apply_to_options(self, file: &mut FileAsset, low_quality_preview: &mut bool) {
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
                ParseImageOption::Lqip(lqip) => {
                    *low_quality_preview = lqip;
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
            "low_quality_preview" => {
                let lqip = input.parse::<syn::LitBool>()?;
                Ok(ParseImageOption::Lqip(lqip.value))
            }
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Unknown image option: {}. Supported options are format, size, preload, url_encoded, low_quality_preview",
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

#[derive(Clone, Copy, Default)]
enum ImageType {
    Png,
    Jpeg,
    Webp,
    #[default]
    Avif,
}

pub struct ImageAssetParser {
    file_name: String,
    low_quality_preview: Option<String>,
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
        let mut this_file =
            FileAsset::new(path.clone()).with_options(manganis_common::FileOptions::Image(
                ImageOptions::new(manganis_common::ImageType::Avif, None),
            ));
        let mut low_quality_preview = false;
        if let Some(parsed_options) = parsed_options {
            parsed_options.apply_to_options(&mut this_file, &mut low_quality_preview);
        }

        let asset = add_asset(manganis_common::AssetType::File(this_file.clone()));
        let this_file = match asset {
            manganis_common::AssetType::File(this_file) => this_file,
            _ => unreachable!(),
        };
        let file_name = if this_file.url_encoded() {
            url_encoded_asset(&this_file).map_err(|e| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to encode file: {}", e),
                )
            })?
        } else {
            this_file.served_location()
        };

        let low_quality_preview = if low_quality_preview {
            let current_image_size = match this_file.options() {
                manganis_common::FileOptions::Image(options) => options.size(),
                _ => None,
            };
            let low_quality_preview_size = current_image_size
                .map(|(width, height)| {
                    let width = width / 10;
                    let height = height / 10;
                    (width, height)
                })
                .unwrap_or((32, 32));
            let lqip = FileAsset::new(path).with_options(manganis_common::FileOptions::Image(
                ImageOptions::new(
                    manganis_common::ImageType::Avif,
                    Some(low_quality_preview_size),
                ),
            ));
            Some(url_encoded_asset(&lqip).map_err(|e| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to encode file: {}", e),
                )
            })?)
        } else {
            None
        };

        Ok(ImageAssetParser {
            file_name,
            low_quality_preview,
        })
    }
}

fn url_encoded_asset(file_asset: &FileAsset) -> Result<String, syn::Error> {
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
    process_file(file_asset, &output_folder).map_err(|e| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to process file: {}", e),
        )
    })?;
    let file = output_folder.join(file_asset.location().unique_name());
    let data = std::fs::read(file).map_err(|e| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to read file: {}", e),
        )
    })?;
    let data = base64::engine::general_purpose::STANDARD_NO_PAD.encode(data);
    let mime = file_asset
        .location()
        .source()
        .mime_type()
        .ok_or(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Failed to get mime type",
        ))?;
    Ok(format!("data:{mime};base64,{data}"))
}

impl ToTokens for ImageAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let file_name = &self.file_name;
        let low_quality_preview = match &self.low_quality_preview {
            Some(lqip) => quote! { Some(#lqip) },
            None => quote! { None },
        };

        tokens.extend(quote! {
            manganis::ImageAsset::new(#file_name).with_preview(#low_quality_preview)
        })
    }
}
