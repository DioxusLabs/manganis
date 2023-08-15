use assets_common::FileAsset;
use quote::{quote, ToTokens};
use syn::parse::Parse;

use crate::add_asset;

pub struct FileAssetParser {
    file_name: String,
}

impl Parse for FileAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path = input.parse::<syn::LitStr>()?;

        let path_as_str = path.value();
        let path = std::path::PathBuf::from(&path_as_str);
        let asset = FileAsset::new(path);
        match asset {
            Ok( this_file) => {
                let asset = add_asset(assets_common::AssetType::File(this_file.clone()));
                let this_file = match asset {
                    assets_common::AssetType::File(this_file) => this_file,
                    _ => unreachable!(),
                };
                let file_name: String = this_file.unique_name().to_string();

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
