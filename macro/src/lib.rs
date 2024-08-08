#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use css::CssAssetParser;
use file::FileAssetParser;
use folder::FolderAssetParser;
use font::FontAssetParser;
use image::ImageAssetParser;
use js::JsAssetParser;
use json::JsonAssetParser;
use manganis_common::cache::macro_log_file;
use manganis_common::{AssetSource, MetadataAsset, TailwindAsset};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use syn::{parse::Parse, parse_macro_input, LitStr};

mod css;
mod file;
mod folder;
mod font;
mod image;
mod js;
mod json;

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
        tracing_subscriber::fmt::fmt().with_writer(file).init();
    }
}

/// this new approach will store the assets descriptions *inside the executable*.
/// The trick is to use the `link_section` attribute.
/// We force rust to store a json representation of the asset description
/// inside a particular region of the binary, with the label "manganis".
/// After linking, the "manganis" sections of the different executables will be merged.
fn generate_link_section(asset: manganis_common::AssetType) -> TokenStream2 {
    let position = proc_macro2::Span::call_site();

    let asset_description = serde_json::to_string(&asset).unwrap();

    let len = asset_description.as_bytes().len();

    let asset_bytes = syn::LitByteStr::new(asset_description.as_bytes(), position);

    let section_name = syn::LitStr::new(
        manganis_common::linker::LinkSection::CURRENT.link_section,
        position,
    );

    quote! {
        #[link_section = #section_name]
        #[used]
        static ASSET: [u8; #len] = * #asset_bytes;
    }
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

    let asset = manganis_common::AssetType::Tailwind(TailwindAsset::new(&input_as_str));

    let link_section = generate_link_section(asset);

    quote! {
        {
        #link_section
        #input_as_str
        }
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
/// const _: &str = manganis::mg!("src/asset.txt");
/// ```
/// Or you can use URLs to read the asset at build time from a remote location
/// ```rust
/// const _: &str = manganis::mg!("https://rustacean.net/assets/rustacean-flat-happy.png");
/// ```
///
/// # Images
///
/// You can collect images which will be automatically optimized with the image builder:
/// ```rust
/// const _: manganis::ImageAsset = manganis::mg!(image("rustacean-flat-gesture.png"));
/// ```
/// Resize the image at compile time to make the assets file size smaller:
/// ```rust
/// const _: manganis::ImageAsset = manganis::mg!(image("rustacean-flat-gesture.png").size(52, 52));
/// ```
/// Or convert the image at compile time to a web friendly format:
/// ```rust
/// const _: manganis::ImageAsset = manganis::mg!(image("rustacean-flat-gesture.png").format(ImageFormat::Avif).size(52, 52));
/// ```
/// You can mark images as preloaded to make them load faster in your app
/// ```rust
/// const _: manganis::ImageAsset = manganis::mg!(image("rustacean-flat-gesture.png").preload());
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

    let asset = parse_macro_input!(input as AnyAssetParser);

    quote! {
        #asset
    }
    .into_token_stream()
    .into()
}

#[derive(Copy, Clone, Default, PartialEq)]
enum ReturnType {
    #[default]
    AssetSpecific,
    StaticStr,
}

struct AnyAssetParser {
    return_type: ReturnType,
    asset_type: syn::Result<AnyAssetParserType>,
    source: TokenStream2,
}

impl ToTokens for AnyAssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let asset = match &self.asset_type {
            Ok(AnyAssetParserType::File(file)) => file.into_token_stream(),
            Ok(AnyAssetParserType::Folder(folder)) => folder.into_token_stream(),
            Ok(AnyAssetParserType::Image(image)) => {
                let tokens = image.into_token_stream();
                if self.return_type == ReturnType::StaticStr {
                    quote! {
                        #tokens.path()
                    }
                } else {
                    tokens
                }
            }
            Ok(AnyAssetParserType::Font(font)) => font.into_token_stream(),
            Ok(AnyAssetParserType::Css(css)) => css.into_token_stream(),
            Ok(AnyAssetParserType::Js(js)) => js.into_token_stream(),
            Ok(AnyAssetParserType::Json(js)) => js.into_token_stream(),
            Err(e) => e.to_compile_error(),
        };
        let source = &self.source;
        let source = quote! {
            const _: &dyn manganis::ForMgMacro = {
                use manganis::*;
                &#source
            };
        };

        tokens.extend(quote! {
            {
                #source
                #asset
            }
        })
    }
}

impl Parse for AnyAssetParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // First try to parse `"myfile".option1().option2()`. We parse that like asset_type("myfile.png").option1().option2()
        if input.peek(syn::LitStr) {
            let path_str = input.parse::<syn::LitStr>()?;
            // Try to parse an extension
            let asset = AssetSource::parse_any(&path_str.value())
            .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;
            let input: proc_macro2::TokenStream = input.parse()?;
            let parse_asset = || -> syn::Result<Self> {
                if let Some(extension) = asset.extension() {
                    if extension.parse::<manganis_common::ImageType>().is_ok() {
                        return syn::parse2(
                            quote_spanned! { path_str.span() => image(#path_str) #input },
                        );
                    } else if extension.parse::<manganis_common::VideoType>().is_ok() {
                        return syn::parse2(
                            quote_spanned! { path_str.span() => video(#path_str) #input },
                        );
                    }
                }
                if let AssetSource::Local(path) = &asset {
                    if path.is_dir() {
                        return syn::parse2(
                            quote_spanned! { path_str.span() => folder(#path_str) #input },
                        );
                    }
                }
                syn::parse2(quote_spanned! { path_str.span() => file(#path_str) #input })
            };

            let mut asset = parse_asset()?;
            // We always return a static string if the asset was not parsed with an explicit type
            asset.return_type = ReturnType::StaticStr;
            return Ok(asset);
        }

        let builder_tokens = { input.fork().parse::<TokenStream2>()? };

        let asset = input.parse::<AnyAssetParserType>();
        Ok(AnyAssetParser {
            return_type: ReturnType::AssetSpecific,
            asset_type: asset,
            source: builder_tokens,
        })
    }
}

enum AnyAssetParserType {
    File(FileAssetParser),
    Folder(FolderAssetParser),
    Image(ImageAssetParser),
    Font(FontAssetParser),
    Css(CssAssetParser),
    Js(JsAssetParser),
    Json(JsonAssetParser),
}

impl Parse for AnyAssetParserType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<syn::Ident>()?;
        let as_string = ident.to_string();

        Ok(match &*as_string {
            // videos and files don't have any special settings yet, we just parse them as files
            "video" | "file" => Self::File(input.parse::<FileAssetParser>()?),
            "folder" => Self::Folder(input.parse::<FolderAssetParser>()?),
            "image" => Self::Image(input.parse::<ImageAssetParser>()?),
            "font" => Self::Font(input.parse::<FontAssetParser>()?),
            "css" => Self::Css(input.parse::<CssAssetParser>()?),
            "js" => Self::Js(input.parse::<JsAssetParser>()?),
            "json" => Self::Json(input.parse::<JsonAssetParser>()?),
            _ => {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                        "Unknown asset type: {as_string}. Supported types are file, image, font, and css"
                    ),
                ))
            }
        })
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

    let asset = manganis_common::AssetType::Metadata(MetadataAsset::new(
        md.key.as_str(),
        md.value.as_str(),
    ));

    let link_section = generate_link_section(asset);

    quote! {
        {
            #link_section
        }
    }
    .into_token_stream()
    .into()
}

fn quote_path(path: &Result<String, manganis_common::ManganisSupportError>) -> TokenStream2 {
    match path {
        Ok(path) => quote! { #path },
        Err(err) => {
            // Expand the error into a warning and return an empty path. Manganis should try not fail to compile the application because it may be checked in CI where manganis CLI support is not available.
            let err = err.to_string();
            quote! {
                {
                    #[deprecated(note = #err)]
                    struct ManganisSupportError;
                    _ = ManganisSupportError;
                    ""
                }
            }
        }
    }
}

#[cfg(feature = "url-encoding")]
pub(crate) fn url_encoded_asset(
    file_asset: &manganis_common::FileAsset,
) -> Result<String, syn::Error> {
    use base64::Engine;

    let target_directory =
        std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let output_folder = std::path::Path::new(&target_directory)
        .join("manganis")
        .join("assets");
    std::fs::create_dir_all(&output_folder).map_err(|e| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to create output folder: {}", e),
        )
    })?;
    manganis_cli_support::process_file(file_asset, &output_folder).map_err(|e| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to process file: {}", e),
        )
    })?;
    let file = output_folder.join(file_asset.location().unique_name());
    let data = std::fs::read(file).map_err(|e| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to read file: {}", e),
        )
    })?;
    let data = base64::engine::general_purpose::STANDARD_NO_PAD.encode(data);
    let mime = manganis_common::get_mime_from_ext(file_asset.options().extension());
    Ok(format!("data:{mime};base64,{data}"))
}

pub(crate) fn verify_preload_valid(ident: &Ident) -> Result<(), syn::Error> {
    // Compile time preload is only supported for the primary package
    if std::env::var("CARGO_PRIMARY_PACKAGE").is_err() {
        return Err(syn::Error::new(
            ident.span(),
            "The `preload` option is only supported for the primary package. Libraries should not preload assets or should preload assets\
            at runtime with utilities your framework provides",
        ));
    }

    Ok(())
}
