//! proc-macro crate intended to be used together with `l0g` crate.

#![warn(missing_docs)]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[cfg(all(feature = "defmt", feature = "log"))]
compile_error!("defmt and log are mutually exclusive");

/// Derive the appropriate "formatting" trait, depending on the feature chosen
/// on the `l0g` crate.
/// - `defmt` will derive `defmt::Format` trait
/// - `log` will derive `core::fmt::Debug` trait
/// - no feature is a noop
#[cfg(feature = "defmt")]
#[proc_macro_attribute]
#[proc_macro_error]
pub fn format(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    quote::quote!(
        #[allow(unused)]
        use l0g::__internal::*;
        #[derive(defmt::Format)]
        #input
    )
    .into()
}

/// Derive the appropriate "formatting" trait, depending on the feature chosen
/// on the `l0g` crate.
/// - `defmt` will derive `defmt::Format` trait
/// - `log` will derive `core::fmt::Debug` trait
/// - no feature is a noop
#[cfg(feature = "log")]
#[proc_macro_attribute]
#[proc_macro_error]
pub fn format(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    quote::quote!(
        #[derive(core::fmt::Debug)]
        #input
    )
    .into()
}

/// Derive the appropriate "formatting" trait, depending on the feature chosen
/// on the `l0g` crate.
/// - `defmt` will derive `defmt::Format` trait
/// - `log` will derive `core::fmt::Debug` trait
/// - no feature is a noop
#[cfg(all(not(feature = "defmt"), not(feature = "log")))]
#[proc_macro_attribute]
#[proc_macro_error]
pub fn format(_: TokenStream, input: TokenStream) -> TokenStream {
    input
}
