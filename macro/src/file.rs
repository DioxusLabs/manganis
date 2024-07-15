use manganis_common::{AssetType, FileAsset, ManganisSupportError};
use quote::{quote, ToTokens};
use syn::{parenthesized, parse::Parse};

use crate::generate_link_section;

pub struct FileAssetParser {
    file_name: Result<String, ManganisSupportError>,
    asset: AssetType,
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
        let asset = manganis_common::AssetType::File(this_file.clone());

        let file_name = this_file.served_location();

        Ok(FileAssetParser { file_name, asset })
    }
}

impl ToTokens for FileAssetParser {
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
