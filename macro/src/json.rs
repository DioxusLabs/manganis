use manganis_common::{AssetType, FileAsset, FileOptions, FileSource, ManganisSupportError};
use quote::{quote, ToTokens};
use syn::{parenthesized, parse::Parse};

use crate::generate_link_section;

struct ParseJsonOptions {
    options: Vec<ParseJsonOption>,
}

impl ParseJsonOptions {
    fn apply_to_options(self, file: &mut FileAsset) {
        for option in self.options {
            option.apply_to_options(file);
        }
    }
}

impl Parse for ParseJsonOptions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut options = Vec::new();
        while !input.is_empty() {
            options.push(input.parse::<ParseJsonOption>()?);
        }
        Ok(ParseJsonOptions { options })
    }
}

enum ParseJsonOption {
    UrlEncoded(bool),
    Preload(bool),
}

impl ParseJsonOption {
    fn apply_to_options(self, file: &mut FileAsset) {
        match self {
            ParseJsonOption::Preload(preload) => file.with_options_mut(|options| {
                if let FileOptions::Json(options) = options {
                    options.set_preload(preload);
                }
            }),
            ParseJsonOption::UrlEncoded(url_encoded) => {
                file.set_url_encoded(url_encoded);
            }
        }
    }
}

impl Parse for ParseJsonOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<syn::Token![.]>()?;
        let ident = input.parse::<syn::Ident>()?;
        let _content;
        parenthesized!(_content in input);
        match ident.to_string().as_str() {
            "preload" => {
                crate::verify_preload_valid(&ident)?;
                Ok(ParseJsonOption::Preload(true))
            },
            "url_encoded" => Ok(ParseJsonOption::UrlEncoded(true)),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Unknown Json option: {}. Supported options are preload, url_encoded, and minify",
                    ident
                ),
            )),
        }
    }
}

pub struct JsonAssetParser {
    file_name: Result<String, ManganisSupportError>,
    asset: AssetType,
}

impl Parse for JsonAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        parenthesized!(inside in input);
        let path = inside.parse::<syn::LitStr>()?;

        let parsed_options = {
            if input.is_empty() {
                None
            } else {
                Some(input.parse::<ParseJsonOptions>()?)
            }
        };

        let path_as_str = path.value();
        let path: FileSource = match path_as_str.parse() {
            Ok(path) => path,
            Err(e) => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("{e}"),
                ))
            }
        };
        let mut this_file = FileAsset::new(path.clone())
            .with_options(manganis_common::FileOptions::Json(Default::default()));
        if let Some(parsed_options) = parsed_options {
            parsed_options.apply_to_options(&mut this_file);
        }

        let asset = manganis_common::AssetType::File(this_file.clone());

        let file_name = if this_file.url_encoded() {
            #[cfg(not(feature = "url-encoding"))]
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "URL encoding is not enabled. Enable the url-encoding feature to use this feature",
            ));
            #[cfg(feature = "url-encoding")]
            Ok(crate::url_encoded_asset(&this_file).map_err(|e| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to encode file: {}", e),
                )
            })?)
        } else {
            this_file.served_location()
        };

        Ok(JsonAssetParser { file_name, asset })
    }
}

impl ToTokens for JsonAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let file_name = crate::quote_path(&self.file_name);

        let link_section = generate_link_section(self.asset.clone());

        tokens.extend(quote! {
            {
                #link_section
                #file_name
            }
        })
    }
}
