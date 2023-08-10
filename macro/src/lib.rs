use assets_common::TailwindAsset;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn asset(input: TokenStream) -> TokenStream {
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
