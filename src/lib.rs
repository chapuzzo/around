use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse, parse_macro_input};

/// Enables running other function before the function call.
///
/// The function must exist and be in scope.
/// ```no_run
///     #[around(database_clean)]
///     fn fancy_func(...){
///         ...
///     }
/// ```
/// This will run the `database_clean` function once before the function code itself.
#[proc_macro_attribute]
pub fn before(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut original: syn::ItemFn = parse_macro_input!(input);
    let hooked_fn: syn::Ident = parse_macro_input!(metadata);

    let element = parse(
        quote!(
            #hooked_fn();
        )
        .into(),
    )
    .expect("could not parse before statement");

    original.block.stmts.insert(0, element);

    original.into_token_stream().into()
}

/// Enables running other function after the function call.
///
/// The function must exist and be in scope.
/// ```no_run
///     #[around(database_clean)]
///     fn fancy_func(...){
///         ...
///     }
/// ```
/// This will run the `database_clean` function once after the function code itself.
#[proc_macro_attribute]
pub fn after(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let mut original: syn::ItemFn = parse_macro_input!(input);
    let hooked_fn: syn::Ident = parse_macro_input!(metadata);

    let element = parse(
        quote!(
            #hooked_fn();
        )
        .into(),
    )
    .expect("could not parse after statement");

    original.block.stmts.push(element);

    original.into_token_stream().into()
}

/// Enables running other function around the function call.
///
/// The function must exist and be in scope.
/// ```no_run
///     #[around(database_clean)]
///     fn fancy_func(...){
///         ...
///     }
/// ```
/// This will run the `database_clean` function twice: _both_ before and after the function code itself.
#[proc_macro_attribute]
pub fn both(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let hooked_fn: syn::Ident = parse_macro_input!(metadata);
    let ofiginal = parse_macro_input!(input as syn::ItemFn);

    quote!(
        #[before(#hooked_fn)]
        #[after(#hooked_fn)]
        #ofiginal
    )
    .to_token_stream()
    .into()
}
