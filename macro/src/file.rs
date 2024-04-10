use manganis_common::FileAsset;
use quote::{quote, ToTokens};
use syn::{parenthesized, parse::Parse};

use crate::add_asset;

pub struct FileAssetParser {
    file_name: String,
}

impl Parse for FileAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inside;
        parenthesized!(inside in input);
        let path = inside.parse::<syn::LitStr>()?;

        let path_as_str = path.value();
        let path = match path_as_str.parse() {
            Ok(path) => path,
            Err(e) => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("{e}"),
                ))
            }
        };
        let this_file = FileAsset::new(path);
        let asset =
            add_asset(manganis_common::AssetType::File(this_file.clone())).map_err(|e| {
                syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!("Failed to add asset: {e}"),
                )
            })?;
        let this_file = match asset {
            manganis_common::AssetType::File(this_file) => this_file,
            _ => unreachable!(),
        };
        let file_name = this_file.served_location();

        Ok(FileAssetParser { file_name })
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
