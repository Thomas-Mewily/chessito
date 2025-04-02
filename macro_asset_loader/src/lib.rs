
mod generate_asset;
use generate_asset::*;

extern crate proc_macro;
use proc_macro2::TokenStream;
//use quote::quote;
//use quote::quote;

/* 
#[proc_macro]
pub fn make_answer(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}*/

// Todo : refactor it for later
#[proc_macro]
pub fn import_asset(item: proc_macro::TokenStream) -> proc_macro::TokenStream 
{
    let input = item.to_string();
    let parts: Vec<&str> = input.split(',').collect();

    if parts.len() != 2 {
        panic!("Expected two string parameters separated by a comma");
    }

    // not really robust but anyway
    let asset_path = parts[0].trim_matches(|c| c == '"' || c == '\'' || c == ' ');
    let code_path  = parts[1].trim_matches(|c| c == '"' || c == '\'' || c == ' ');

    //println!("Generating assets at `{}` to code at `{}`", asset_path, code_path);

    generate_game_asset(asset_path, code_path);
    TokenStream::default().into()
}