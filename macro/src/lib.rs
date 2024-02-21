#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use file::FileAssetParser;
use font::FontAssetParser;
use image::ImageAssetParser;
use manganis_common::{AssetType, MetadataAsset, TailwindAsset};
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

fn add_asset(asset: manganis_common::AssetType) -> std::io::Result<AssetType> {
    if !INITIALIZED.load(Ordering::Relaxed) {
        INITIALIZED.store(true, Ordering::Relaxed);
        manganis_common::clear_assets()?;
    }

    manganis_common::add_asset(asset)
}

/// Collects tailwind classes that will be included in the final binary and returns them unmodified
///
/// ```rust
/// // You can include tailwind classes that will be collected into the final binary
/// const TAILWIND_CLASSES: &str = manganis::classes!("flex flex-col p-5");
/// assert_eq!(TAILWIND_CLASSES, "flex flex-col p-5");
/// ```
#[proc_macro]
pub fn classes(input: TokenStream) -> TokenStream {
    let input_as_str = parse_macro_input!(input as LitStr);
    let input_as_str = input_as_str.value();

    let result = add_asset(manganis_common::AssetType::Tailwind(TailwindAsset::new(
        &input_as_str,
    )))
    .map_err(|e| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to add asset: {e}"),
        )
        .into_compile_error()
    });

    let result = match result {
        Ok(_) => quote! {
            #input_as_str
        },
        Err(e) => quote! {
            #e
        },
    };

    quote! {
        #result
    }
    .into_token_stream()
    .into()
}

/// The mg macro collects assets that will be included in the final binary
///
/// # Files
///
/// The file builder collects an arbitrary file. Relative paths are resolved relative to the package root
/// ```rust
/// const _: &str = manganis::mg!(file("./src/asset.txt"));
/// ```
/// Or you can use URLs to read the asset at build time from a remote location
/// ```rust
/// const _: &str = manganis::mg!(file("https://rustacean.net/assets/rustacean-flat-happy.png"));
/// ```
///
/// # Images
///
/// You can collect images which will be automatically optimized with the image builder:
/// ```rust
/// const _: &str = manganis::mg!(image("./rustacean-flat-gesture.png"));
/// ```
/// Resize the image at compile time to make the assets file size smaller:
/// ```rust
/// const _: &str = manganis::mg!(image("./rustacean-flat-gesture.png").size(52, 52));
/// ```
/// Or convert the image at compile time to a web friendly format:
/// ```rust
/// const _: &str = manganis::mg!(image("./rustacean-flat-gesture.png").format(ImageFormat::Avif).size(52, 52));
/// ```
/// You can mark images as preloaded to make them load faster in your app
/// ```rust
/// const _: &str = manganis::mg!(image("./rustacean-flat-gesture.png").preload());
/// ```
///
/// # Fonts
///
/// You can use the font builder to collect fonts that will be included in the final binary from google fonts
/// ```rust
/// const _: &str = manganis::mg!(font().families(["Roboto"]));
/// ```
/// You can specify weights for the fonts
/// ```rust
/// const _: &str = manganis::mg!(font().families(["Roboto"]).weights([200]));
/// ```
/// Or set the text to only include the characters you need
/// ```rust
/// const _: &str = manganis::mg!(font().families(["Roboto"]).weights([200]).text("Hello, world!"));
/// ```
#[proc_macro]
pub fn mg(input: TokenStream) -> TokenStream {
    use proc_macro2::TokenStream as TokenStream2;

    let builder_tokens = {
        let input = input.clone();
        parse_macro_input!(input as TokenStream2)
    };

    let builder_output = quote! {
        const _: &dyn manganis::ForMgMacro = {
            use manganis::*;
            &#builder_tokens
        };
    };

    let asset = syn::parse::<ImageAssetParser>(input.clone())
        .ok()
        .map(ToTokens::into_token_stream)
        .or_else(|| {
            syn::parse::<FontAssetParser>(input.clone())
                .ok()
                .map(ToTokens::into_token_stream)
        })
        .or_else(|| {
            syn::parse::<FileAssetParser>(input.clone())
                .ok()
                .map(ToTokens::into_token_stream)
        });

    match asset {
        Some(asset) => quote! {
            {
                #builder_output
                #asset
            }
        }
        .into_token_stream()
        .into(),
        None => quote! {
            {
                #builder_output
                compile_error!("Expected an image, font or file asset")
            }
        }
        .into_token_stream()
        .into(),
    }
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
/// const _: () = manganis::meta!("opt-level": "3");
/// ```
#[proc_macro]
pub fn meta(input: TokenStream) -> TokenStream {
    let md = parse_macro_input!(input as MetadataValue);

    let result = add_asset(manganis_common::AssetType::Metadata(MetadataAsset::new(
        md.key.as_str(),
        md.value.as_str(),
    )))
    .map_err(|e| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to add asset: {e}"),
        )
        .into_compile_error()
    })
    .err();

    quote! {#result}.into_token_stream().into()
}
