#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use file::FileAssetParser;
use font::FontAssetParser;
use image::ImageAssetParser;
use manganis_common::cache::macro_log_file;
use manganis_common::{AssetType, MetadataAsset, TailwindAsset};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use syn::{parse::Parse, parse_macro_input, LitStr};

mod file;
mod font;
mod image;

static LOG_FILE_FRESH: AtomicBool = AtomicBool::new(false);

fn trace_to_file() {
    // If this is the first time the macro is used in the crate, set the subscriber to write to a file
    if !LOG_FILE_FRESH.fetch_or(true, Ordering::Relaxed) {
        let path = macro_log_file();
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        tracing_subscriber::fmt::fmt()
            .with_writer(file)
            .init();
    }
}

// It appears rustc uses one instance of the dynamic library for each crate that uses it.
// We can reset the asset of the current crate the first time the macro is used in the crate.
static INITIALIZED: AtomicBool = AtomicBool::new(false);

fn add_asset(asset: manganis_common::AssetType) -> std::io::Result<AssetType> {
    if !INITIALIZED.fetch_or(true, Ordering::Relaxed) {
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
    trace_to_file();

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
    trace_to_file();

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

    let asset = parse_macro_input!(input as AnyAssetParser);

    quote! {
        {
            #builder_output
            #asset
        }
    }
    .into_token_stream()
    .into()
}

enum AnyAssetParser {
    File(FileAssetParser),
    Image(ImageAssetParser),
    Font(FontAssetParser),
}

impl Parse for AnyAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let as_string = ident.to_string();

        Ok(match &*as_string {
            "file" => Self::File(input.parse::<FileAssetParser>()?),
            "image" => Self::Image(input.parse::<ImageAssetParser>()?),
            "font" => Self::Font(input.parse::<FontAssetParser>()?),
            _ => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Unknown asset type: {as_string}. Supported types are file, image, font"
                    ),
                ))
            }
        })
    }
}

impl ToTokens for AnyAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::File(file) => {
                file.to_tokens(tokens);
            }
            Self::Image(image) => {
                image.to_tokens(tokens);
            }
            Self::Font(font) => {
                font.to_tokens(tokens);
            }
        }
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
    trace_to_file();

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
