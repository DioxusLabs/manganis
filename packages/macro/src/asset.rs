// use crate::folder::FolderAssetParser;
// use crate::font::FontAssetParser;
// use crate::image::ImageAssetParser;
// use crate::js::JsAssetParser;
// use crate::{file::FileAssetParser, generate_link_section};
// use crate::json::JsonAssetParser;
// use crate::{css::CssAssetParser, resource::ResourceAssetParser};

// use manganis_common::cache::macro_log_file;
use core::panic;
use manganis_common::{MetadataAsset, ResourceAsset, TailwindAsset};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use serde::Serialize;
use std::{fs::File, sync::atomic::AtomicBool};
use std::{path::PathBuf, sync::atomic::Ordering};
use syn::{parenthesized, parse::Parse, parse_macro_input, Expr, ExprLit, Lit, LitStr, PatLit};

pub struct AssetParser {
    resource: ResourceAsset,
    name: Option<syn::Ident>,
    options: Vec<MethodCallOption>,
    source: TokenStream2,
}

impl Parse for AssetParser {
    // we can take
    //
    // This gives you the Asset type - it's generic and basically unrefined
    // ```
    // asset!("myfile.png")
    // ```
    //
    // To narrow the type, use a call to get the refined type
    // ```
    // asset!(
    //     image("myfile.png")
    //      .format(ImageType::Jpg)
    //      .size(512, 512)
    // )
    // ```
    //
    // But we need to decide the hint first before parsing the options
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Save the source of the macro so it comes out with the const builder api
        let source = input.fork().parse::<TokenStream2>()?;

        // Parse as an expr
        let expr = input.parse::<syn::Expr>()?;

        // And then match - is it a literal or a method call?
        // the method call will be a recursive chain
        let resource;
        let mut name = None;
        let mut options = vec![];
        match expr {
            syn::Expr::Lit(ExprLit {
                lit: Lit::Str(lit), ..
            }) => {
                resource = ResourceAsset::parse_any(&lit.value())
                    .map_err(|e| syn::Error::new(lit.span(), e))?;
            }
            syn::Expr::MethodCall(call) => {
                let collected = Self::collect_options_from_method_call(&call)?;
                name = Some(collected.name);
                resource = collected.resource;
                options = collected.options;
            }
            syn::Expr::Call(call) => {
                let collected = Self::collect_resource_from_call(&call)?;
                name = Some(collected.name);
                resource = collected.resource;
                options = collected.options;
            }
            _ => todo!(),
        }

        Ok(Self {
            resource,
            name,
            options,
            source,
        })
    }
}

impl AssetParser {
    fn collect_resource_from_call(call: &syn::ExprCall) -> syn::Result<RefinedAssetCall> {
        match call.func.as_ref() {
            Expr::Path(path) => {
                let ident = path.path.require_ident()?;
                let arg = call.args.first().unwrap();
                let Expr::Lit(ExprLit { lit, .. }) = arg else {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "The first argument of the asset call must be a literal",
                    ));
                };

                let Lit::Str(lit) = lit else {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "The first argument of the asset call must be a literal",
                    ));
                };

                let resource = ResourceAsset::parse_any(&lit.value())
                    .map_err(|e| syn::Error::new(lit.span(), e))?;

                Ok(RefinedAssetCall {
                    resource,
                    name: ident.clone(),
                    options: vec![],
                })
            }
            _ => {
                panic!("{call:?}");
            }
        }
    }

    fn collect_options_from_method_call(
        call: &syn::ExprMethodCall,
    ) -> syn::Result<RefinedAssetCall> {
        let receiver = call.receiver.as_ref();

        let Expr::Lit(ExprLit { lit, .. }) = receiver else {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "The receiver of the method call must be a literal",
            ));
        };
        todo!()
    }
}

struct RefinedAssetCall {
    name: syn::Ident,
    resource: ResourceAsset,
    options: Vec<MethodCallOption>,
}

struct MethodCallOption {
    method: syn::Ident,
    args: Vec<syn::Lit>,
}

impl ToTokens for AssetParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let source = &self.source;
        let asset = &self.resource;
        let link_section = crate::generate_link_section(&asset);
        let input = asset.input.to_string();
        let bundled = asset.bundled.to_string();

        let local = match asset.local.as_ref() {
            Some(local) => {
                let local = local.to_string();
                quote! { #local }
            }
            None => {
                quote! {
                    {
                        // ensure it exists by throwing away the include_bytes
                        static _BLAH: &[u8] = include_bytes!(#input);

                        // But then pass along the path
                        concat!(env!("CARGO_MANIFEST_DIR"), "/", file!(), "/<split>/", #input)
                    }
                }
            }
        };

        let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
        let displayed_manifest_dir = manifest_dir.display().to_string();

        tokens.extend(quote! {
            {
                const _: &dyn manganis::ForMgMacro = {
                    use manganis::*;
                    &#source
                };

                #link_section
                manganis::Asset {
                    input: #input,
                    source_file: concat!(#displayed_manifest_dir, "/", file!()),
                    local: #local,
                    bundled: #bundled,
                }
            }
        })
    }
}

enum RefinedAsset {
    Asset,
    File,
    Folder,
    Image,
    Font,
    Css,
    Js,
    Json,
}

// impl Parse for AnyAssetParserType {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let ident = input.parse::<syn::Ident>()?;
//         let as_string = ident.to_string();

//         Ok(match &*as_string {
//             "file" => Self::File(input.parse::<FileAssetParser>()?),
//             "folder" => Self::Folder(input.parse::<FolderAssetParser>()?),
//             "image" => Self::Image(input.parse::<ImageAssetParser>()?),
//             "font" => Self::Font(input.parse::<FontAssetParser>()?),
//             "css" => Self::Css(input.parse::<CssAssetParser>()?),
//             "js" => Self::Js(input.parse::<JsAssetParser>()?),
//             "json" => Self::Json(input.parse::<JsonAssetParser>()?),
//             _ => {
//                 return Err(syn::Error::new(
//                     proc_macro2::Span::call_site(),
//                     format!(
//                         "Unknown asset type: {as_string}. Supported types are file, image, font, and css"
//                     ),
//                 ))
//             }
//         })
//     }
// }

// Default to the file asset type
// let mut asset_type = AnyAssetParserType::File;

// // If we see an ident, we know we're dealing with a resource
// let resource = if input.peek(syn::Ident) {
//     let ident = input.parse::<syn::Ident>()?;
//     asset_type = match ident.to_string().as_str() {
//         "file" => AnyAssetParserType::File,
//         "folder" => AnyAssetParserType::Folder,
//         "image" => AnyAssetParserType::Image,
//         "font" => AnyAssetParserType::Font,
//         "css" => AnyAssetParserType::Css,
//         "js" => AnyAssetParserType::Js,
//         "json" => AnyAssetParserType::Json,
//         _ => panic!("Unknown asset type: {}", ident),
//     };
//     let parantheted;
//     parenthesized!(parantheted in input);
//     parantheted.parse::<ResourceAssetParser>()?
// } else {
//     input.parse::<ResourceAssetParser>()?
// };

// Read the file from the FS if we need to and refine it if necessary?

// // First try to parse `"myfile".option1().option2()`. We parse that like asset_type("myfile.png").option1().option2()
// if input.peek(syn::LitStr) {
//     // let asset = input.parse::<ResourceAssetParser>()?;
//     let builder_tokens = { input.fork().parse::<TokenStream2>()? };
//     let asset = input.parse::<ResourceAssetParser>()?;
//     let asset = FileAssetParser { asset };

//     return Ok(AssetParser {
//         return_type: ReturnType::StaticStr,
//         asset_type: Ok(AnyAssetParserType::File(asset)),
//         source: builder_tokens,
//         // asset_type: Ok(AnyAssetParserType::File(asset)),
//         // source: input.parse()?,
//     });

//     // let path_str = input.parse::<syn::LitStr>()?;
//     // // Try to parse an extension
//     // let asset = ResourceAsset::parse_any(&path_str.value())
//     //     .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), e))?;
//     // let input: proc_macro2::TokenStream = input.parse()?;
//     // let parse_asset = || -> syn::Result<Self> {
//     //     if let Some(extension) = asset.extension() {
//     //         if extension.parse::<manganis_common::ImageType>().is_ok() {
//     //             return syn::parse2(
//     //                 quote_spanned! { path_str.span() => image(#path_str) #input },
//     //             );
//     //         } else if extension.parse::<manganis_common::VideoType>().is_ok() {
//     //             return syn::parse2(
//     //                 quote_spanned! { path_str.span() => video(#path_str) #input },
//     //             );
//     //         }
//     //     }
//     //     // if let ResourceAsset::Local(path) = &asset {
//     //     //     if path.canonicalized.is_dir() {
//     //     //         return syn::parse2(
//     //     //             quote_spanned! { path_str.span() => folder(#path_str) #input },
//     //     //         );
//     //     //     }
//     //     // }
//     //     syn::parse2(quote_spanned! { path_str.span() => file(#path_str) #input })
//     // };

//     // let mut asset = parse_asset()?;
//     // // We always return a static string if the asset was not parsed with an explicit type
//     // asset.return_type = ReturnType::StaticStr;
//     // return Ok(asset);
// }

// let builder_tokens = { input.fork().parse::<TokenStream2>()? };
// let asset = input.parse::<AnyAssetParserType>();

// Ok(AssetParser {
//     return_type: ReturnType::AssetSpecific,
//     asset_type: asset,
//     source: builder_tokens,
// })

// let asset = match &self.asset_type {
//     Ok(AnyAssetParserType::File(file)) => file.into_token_stream(),
//     Ok(AnyAssetParserType::Folder(folder)) => folder.into_token_stream(),
//     Ok(AnyAssetParserType::Image(image)) => {
//         let tokens = image.into_token_stream();
//         if self.return_type == ReturnType::StaticStr {
//             quote! {
//                 #tokens.path()
//             }
//         } else {
//             tokens
//         }
//     }
//     Ok(AnyAssetParserType::Font(font)) => font.into_token_stream(),
//     Ok(AnyAssetParserType::Css(css)) => css.into_token_stream(),
//     Ok(AnyAssetParserType::Js(js)) => js.into_token_stream(),
//     Ok(AnyAssetParserType::Json(js)) => js.into_token_stream(),
//     Err(e) => e.to_compile_error(),
// };
