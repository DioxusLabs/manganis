use assets_common::{FileAsset, FileOptions, FileSource, ImageOptions};
use quote::{quote, ToTokens};
use syn::{braced, parenthesized, parse::Parse};

use crate::add_asset;

struct ParseImageOptions {
    options: Vec<ParseImageOption>,
}

impl ParseImageOptions {
    fn apply_to_options(self, options: &mut ImageOptions) {
        for option in self.options {
            option.apply_to_options(options);
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
    Format(assets_common::ImageType),
    Size((u32, u32)),
}

impl ParseImageOption {
    fn apply_to_options(self, options: &mut ImageOptions) {
        match self {
            ParseImageOption::Format(format) => {
                options.set_ty(format);
            }
            ParseImageOption::Size(size) => {
                options.set_size(Some(size));
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
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Unknown image option: {}. Supported options are format, size",
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

impl Into<assets_common::ImageType> for ImageType {
    fn into(self) -> assets_common::ImageType {
        match self {
            ImageType::Png => assets_common::ImageType::Png,
            ImageType::Jpeg => assets_common::ImageType::Jpg,
            ImageType::Webp => assets_common::ImageType::Webp,
            ImageType::Avif => assets_common::ImageType::Avif,
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
            ))
        };
        let Ok(extension) = extension.parse() else {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to parse extension: {}, supported types are png, jpeg, webp, avif", extension),
            ))
        };
        let mut options = ImageOptions::new(extension, None);
        if let Some(parsed_options) = parsed_options {
            parsed_options.apply_to_options(&mut options);
        }
        let asset = FileAsset::new_with_options(path, FileOptions::Image(options));
        match asset {
            Ok( this_file) => {
                let asset = add_asset(assets_common::AssetType::File(this_file.clone()));
                let this_file = match asset {
                    assets_common::AssetType::File(this_file) => this_file,
                    _ => unreachable!(),
                };
                let file_name = this_file.served_location();

                Ok(ImageAssetParser {file_name})
            }
            Err(e) => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to canonicalize path: {path_as_str}\nAny relative paths are resolved relative to the manifest directory\n{e}"),
            ))
        }
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
