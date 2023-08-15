use assets_common::{AssetType, MetadataAsset, TailwindAsset};
use file::FileAssetParser;
use image::ImageAssetParser;
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, LitStr};

mod file;
mod image;

// It appears rustc uses one instance of the dynamic library for each crate that uses it.
// We can reset the asset of the current crate the first time the macro is used in the crate.
static RESET_ASSETS: Lazy<()> = Lazy::new(|| assets_common::clear_assets());

fn add_asset(asset: assets_common::AssetType) -> AssetType {
    let _: () = *RESET_ASSETS;

    assets_common::add_asset(asset)
}

#[proc_macro]
pub fn classes(input: TokenStream) -> TokenStream {
    let input_as_str = parse_macro_input!(input as LitStr);
    let input_as_str = input_as_str.value();

    add_asset(assets_common::AssetType::Tailwind(TailwindAsset::new(
        &input_as_str,
    )));

    quote! {
        #input_as_str
    }
    .into_token_stream()
    .into()
}

#[proc_macro]
pub fn image(input: TokenStream) -> TokenStream {
    let asset = parse_macro_input!(input as ImageAssetParser);

    quote! {
        #asset
    }
    .into_token_stream()
    .into()
}

#[proc_macro]
pub fn file(input: TokenStream) -> TokenStream {
    let asset = parse_macro_input!(input as FileAssetParser);

    quote! {
        #asset
    }
    .into_token_stream()
    .into()
}

struct MetadataValue {
    key: String,
    value: String,
}

impl Parse for MetadataValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse::<Ident>()?.to_string();
        input.parse::<syn::Token![:]>()?;
        let value = input.parse::<LitStr>()?.value();
        Ok(Self { key, value })
    }
}

#[proc_macro]
pub fn meta(input: TokenStream) -> TokenStream {
    let md = parse_macro_input!(input as MetadataValue);

    add_asset(assets_common::AssetType::Metadata(MetadataAsset::new(
        md.key.as_str(),
        md.value.as_str(),
    )));

    quote! {}.into_token_stream().into()
}
