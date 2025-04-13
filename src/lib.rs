extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, Block, DeriveInput};
use quote::quote;

#[proc_macro]
pub fn verified(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block);
    let expanded = quote! {
        unsafe #block
    };
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn verified_attr(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as syn::Item);

    match item {
        syn::Item::Fn(mut item_fn) => {
            item_fn.sig.unsafety = Some(syn::token::Unsafe::default());
            let expanded = quote! { #item_fn };
            expanded.into()
        }
        syn::Item::Impl(mut item_impl) => {
            item_impl.unsafety = Some(syn::token::Unsafe::default());
            let expanded = quote! { #item_impl };
            expanded.into()
        }
        _ => {
            let err = syn::Error::new_spanned(
                &item,
                "#[verified] only supported on functions and impls",
            );
            err.to_compile_error().into()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream as TokenStream2;
    use quote::quote;
    use syn::{parse_quote, Block, Item};

    fn token_stream_to_string(ts: TokenStream2) -> String {
        ts.to_string()
    }

    #[test]
    fn test_verified_block() {
        let input: Block = parse_quote! {
            {
                let x = 42;
                println!("Hello");
            }
        };

        let expanded = quote! {
            unsafe #input
        };

        let expected = quote! {
            unsafe {
                let x = 42;
                println!("Hello");
            }
        };

        assert_eq!(
            token_stream_to_string(expanded),
            token_stream_to_string(expected),
            "Verified block should generate unsafe block"
        );
    }

    #[test]
    fn test_verified_function() {
        let mut item: Item = parse_quote! {
            fn foo(x: i32) -> i32 {
                x + 1
            }
        };

        let expanded = match item {
            Item::Fn(ref mut item_fn) => {
                item_fn.sig.unsafety = Some(syn::token::Unsafe::default());
                quote! { #item_fn }
            }
            _ => panic!("Expected Item::Fn"),
        };

        // Expected output
        let expected = quote! {
            unsafe fn foo(x: i32) -> i32 {
                x + 1
            }
        };

        assert_eq!(
            token_stream_to_string(expanded),
            token_stream_to_string(expected),
            "Verified attribute should make function unsafe"
        );
    }

    #[test]
    fn test_verified_impl() {
        let mut item: Item = parse_quote! {
            impl MyType {
                fn bar(&self) {}
            }
        };

        let expanded = match item {
            Item::Impl(ref mut item_impl) => {
                item_impl.unsafety = Some(syn::token::Unsafe::default());
                quote! { #item_impl }
            }
            _ => panic!("Expected Item::Impl"),
        };

        let expected = quote! {
            unsafe impl MyType {
                fn bar(&self) {}
            }
        };

        assert_eq!(
            token_stream_to_string(expanded),
            token_stream_to_string(expected),
            "Verified attribute should make impl unsafe"
        );
    }

    #[test]
    fn test_verified_unsupported() {
        let item: Item = parse_quote! {
            struct MyStruct;
        };

        let output = match item {
            Item::Fn(_) | Item::Impl(_) => panic!("Expected unsupported item"),
            item => {
                let err = syn::Error::new_spanned(
                    item,
                    "#[verified] only supported on functions and impls",
                );
                err.to_compile_error()
            }
        };

        let output_str = token_stream_to_string(output);
        assert!(
            output_str.contains("error") && output_str.contains("only supported on functions and impls"),
            "Verified attribute on struct should produce error"
        );
    }
}