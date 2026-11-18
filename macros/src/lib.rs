// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::ItemFn;

/// Defines the entry point for the program.
///
/// This attribute must be used exactly once within a program and be used on an `fn` or `async fn`
/// with the return type [`()`](unit) or [`!`](never).
#[proc_macro_attribute]
pub fn main(attrs: TokenStream, item: TokenStream) -> TokenStream {
    main2(attrs.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn main2(attrs: TokenStream2, item: TokenStream2) -> syn::Result<TokenStream2> {
    if !attrs.is_empty() {
        return Err(syn::Error::new_spanned(
            attrs,
            "vex_hal::main takes no arguments",
        ));
    }
    let func: ItemFn = syn::parse2(item)?;
    let name = &func.sig.ident;
    let body = if func.sig.asyncness.is_some() {
        quote! {
            let f: fn(Peripherals) -> _ = #name;
            ::vex_hal::runtime::task::block_on(f(peripherals))
        }
    } else {
        quote! {
            let f: fn(Peripherals) = #name;
            f(peripherals)
        }
    };
    Ok(quote! {
        #func

        #[doc(hidden)]
        #[unsafe(export_name = "__vex_hal_main")]
        extern "Rust" fn __vex_hal_main() {
            type Peripherals = ::vex_hal::peripherals::Peripherals;
            let peripherals = unsafe { Peripherals::new() };
            #body
        }
    })
}
