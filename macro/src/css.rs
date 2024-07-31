use manganis_common::{
    AssetType, CssOptions, FileAsset, FileOptions, FileSource, ManganisSupportError,
};
use quote::{quote, ToTokens};
use syn::{parenthesized, parse::Parse, LitBool};

use crate::generate_link_section;

struct ParseCssOptions {
    options: Vec<ParseCssOption>,
}

impl ParseCssOptions {
    fn apply_to_options(self, file: &mut FileAsset) {
        for option in self.options {
            option.apply_to_options(file);
        }
    }
}

impl Parse for ParseCssOptions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut options = Vec::new();
        while !input.is_empty() {
            options.push(input.parse::<ParseCssOption>()?);
        }
        Ok(ParseCssOptions { options })
    }
}

enum ParseCssOption {
    UrlEncoded(bool),
    Preload(bool),
    Minify(bool),
}

impl ParseCssOption {
    fn apply_to_options(self, file: &mut FileAsset) {
        match self {
            ParseCssOption::Preload(_) | ParseCssOption::Minify(_) => {
                file.with_options_mut(|options| {
                    if let FileOptions::Css(options) = options {
                        match self {
                            ParseCssOption::Minify(format) => {
                                options.set_minify(format);
                            }
                            ParseCssOption::Preload(preload) => {
                                options.set_preload(preload);
                            }
                            _ => {}
                        }
                    }
                })
            }
            ParseCssOption::UrlEncoded(url_encoded) => {
                file.set_url_encoded(url_encoded);
            }
        }
    }
}

impl Parse for ParseCssOption {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _ = input.parse::<syn::Token![.]>()?;
        let ident = input.parse::<syn::Ident>()?;
        let content;
        parenthesized!(content in input);
        match ident.to_string().as_str() {
            "preload" => {
                crate::verify_preload_valid(&ident)?;
                Ok(ParseCssOption::Preload(true))
            }
            "url_encoded" => {
                Ok(ParseCssOption::UrlEncoded(true))
            }
            "minify" => {
                Ok(ParseCssOption::Minify(content.parse::<LitBool>()?.value()))
            }
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!(
                    "Unknown Css option: {}. Supported options are preload, url_encoded, and minify",
                    ident
                ),
            )),
        }
    }
}

pub struct CssAssetParser {
    file_name: Result<String, ManganisSupportError>,
    asset: AssetType,
}

impl Parse for CssAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        parenthesized!(inside in input);
        let path = inside.parse::<syn::LitStr>()?;

        let parsed_options = {
            if input.is_empty() {
                None
            } else {
                Some(input.parse::<ParseCssOptions>()?)
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
            .with_options(manganis_common::FileOptions::Css(CssOptions::new()));
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

        Ok(CssAssetParser { file_name, asset })
    }
}

impl ToTokens for CssAssetParser {
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
