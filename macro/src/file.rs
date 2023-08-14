use assets_common::{FileAsset, FileOptions};
use quote::{quote, ToTokens};
use syn::parse::Parse;

use crate::add_asset;

enum AssetType {
    Image(ImageType),
    Other,
}

impl AssetType {
    fn extension(self) -> Option<&'static str> {
        match self {
            AssetType::Image(image_type) => Some(match image_type {
                ImageType::Png => "png",
                ImageType::Jpeg => "jpeg",
                ImageType::Webp => "webp",
                ImageType::Avif => "avif",
            }),
            AssetType::Other => None,
        }
    }
}

impl Parse for AssetType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        match ident.to_string().as_str() {
            "png" => Ok(AssetType::Image(ImageType::Png)),
            "jpeg" => Ok(AssetType::Image(ImageType::Jpeg)),
            "webp" => Ok(AssetType::Image(ImageType::Webp)),
            "avif" => Ok(AssetType::Image(ImageType::Avif)),
            _ => Ok(AssetType::Other),
        }
    }
}

enum ImageType {
    Png,
    Jpeg,
    Webp,
    Avif,
}

pub struct FileAssetParser {
    file_name: String,
}

impl Parse for FileAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<syn::LitStr>()?;

        let output_type = {
            if let Ok(_) = input.parse::<syn::Token![->]>() {
                input.parse::<AssetType>()?
            } else {
                AssetType::Other
            }
        };

        let path_as_str = path.value();
        let path = std::path::PathBuf::from(&path_as_str);
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_owned());
        let asset = FileAsset::new_with_options(
            path,
            FileOptions::default_for_extension(output_type.extension().or(extension.as_deref())),
        );
        match asset {
            Ok(file) => {
                let file_name: String = file.unique_name().to_string();
                println!("file_name: {}", file_name);
                add_asset(assets_common::AssetType::File(file.clone()));

                Ok(FileAssetParser {file_name})
            }
            Err(e) => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to canonicalize path: {path_as_str}\nAny relative paths are resolved relative to the manifest directory\n{e}"),
            ))
        }
    }
}

impl ToTokens for FileAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let file_name = &self.file_name;

        tokens.extend(quote! {
            #file_name
        })
    }
}
