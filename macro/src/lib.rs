#![doc = include_str!("../../README.md")]
#![deny(missing_docs)]

use assets_common::{AssetType, MetadataAsset, TailwindAsset};
use file::FileAssetParser;
use font::FontAssetParser;
use image::ImageAssetParser;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use syn::{parse::Parse, parse_macro_input, LitStr};

mod file;
mod font;
mod image;

// It appears rustc uses one instance of the dynamic library for each crate that uses it.
// We can reset the asset of the current crate the first time the macro is used in the crate.
static INITIALIZED: AtomicBool = AtomicBool::new(false);

fn add_asset(asset: assets_common::AssetType) -> AssetType {
    if !INITIALIZED.load(Ordering::Relaxed) {
        INITIALIZED.store(true, Ordering::Relaxed);
        assets_common::clear_assets();
    }

    assets_common::add_asset(asset)
}

/// Collects tailwind classes that will be included in the final binary and returns them unmodified
///
/// ```rust
/// // You can include tailwind classes that will be collected into the final binary
/// const TAILWIND_CLASSES: &str = collect_assets::classes!("flex flex-col p-5");
/// assert_eq!(TAILWIND_CLASSES, "flex flex-col p-5");
/// ```
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

/// You can use the font macro to collect fonts that will be included in the final binary from google fonts
/// ```rust
/// const _: &str = collect_assets::font!({ families: ["Roboto"] });
/// ```
/// You can specify weights for the fonts
/// ```rust
/// const _: &str = collect_assets::font!({ families: ["Comfortaa"], weights: [300] });
/// ```
/// Or set the text to only include the characters you need
/// ```rust
/// const _: &str = collect_assets::font!({ families: ["Roboto"], weights: [200], text: "light font" });
/// ```
#[proc_macro]
pub fn font(input: TokenStream) -> TokenStream {
    let asset = parse_macro_input!(input as FontAssetParser);

    quote! {
        #asset
    }
    .into_token_stream()
    .into()
}

/// You can collect images which will be automatically optimized with the image macro:
/// ```rust
/// const _: &str = collect_assets::image!("./rustacean-flat-gesture.png");
/// ```
/// Resize the image at compile time to make the assets file size smaller:
/// ```rust
/// const _: &str = collect_assets::image!("./rustacean-flat-gesture.png", { size: (52, 52) });
/// ```
/// Or convert the image at compile time to a web friendly format:
/// ```rust
/// const _: &str = collect_assets::image!("./rustacean-flat-gesture.png", { format: avif, size: (52, 52) });
/// ```
/// You can mark images as preloaded to make them load faster in your app
/// ```rust
/// const _: &str = collect_assets::image!("./rustacean-flat-gesture.png", { preload: true });
/// ```
#[proc_macro]
pub fn image(input: TokenStream) -> TokenStream {
    let asset = parse_macro_input!(input as ImageAssetParser);

    quote! {
        #asset
    }
    .into_token_stream()
    .into()
}

/// The file macro collects an arbitrary file. Relative paths are resolved relative to the package root
/// ```rust
/// const _: &str = collect_assets::file!("./src/asset.txt");
/// ```
/// You can use URLs to read the asset at build time from a remote location
/// ```rust
/// const _: &str = collect_assets::file!("https://rustacean.net/assets/rustacean-flat-happy.png");
/// ```
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

/// // You can also collect arbitrary key-value pairs. The meaning of these pairs is determined by the CLI that processes your assets
/// ```rust
/// const _: () = collect_assets::meta!("opt-level": "3");
/// ```
#[proc_macro]
pub fn meta(input: TokenStream) -> TokenStream {
    let md = parse_macro_input!(input as MetadataValue);

    add_asset(assets_common::AssetType::Metadata(MetadataAsset::new(
        md.key.as_str(),
        md.value.as_str(),
    )));

    quote! {}.into_token_stream().into()
}
