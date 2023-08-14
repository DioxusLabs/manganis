use assets_common::TailwindAsset;
use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, LitStr};

// It appears rustc uses one instance of the dynamic library for each crate that uses it.
// We can reset the asset of the current crate the first time the macro is used in the crate.
static RESET_ASSETS: Lazy<()> = Lazy::new(|| assets_common::clear_assets());

#[proc_macro]
pub fn asset(input: TokenStream) -> TokenStream {
    let _: () = *RESET_ASSETS;
    
    let input_as_str = parse_macro_input!(input as LitStr);
    let input_as_str = input_as_str.value();

    assets_common::add_asset(assets_common::AssetType::Tailwind(TailwindAsset::new(
        &input_as_str,
    )));

    quote! {
        #input_as_str
    }
    .into_token_stream()
    .into()
}
