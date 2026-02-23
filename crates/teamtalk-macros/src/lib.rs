//! Proc-macro helpers for TeamTalk bot handlers.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, ItemFn, LitStr, parse::Parse, parse_macro_input};

struct EventAttr {
    expr: Expr,
}

impl Parse for EventAttr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            expr: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn teamtalk_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let command_name = parse_macro_input!(attr as LitStr);
    let function = parse_macro_input!(item as ItemFn);
    let fn_name = function.sig.ident.clone();
    let register_name = format_ident!("register_{}", fn_name);

    let expanded = quote! {
        #function

        /// Auto-generated registration helper for this command handler.
        pub fn #register_name(router: ::teamtalk::Router) -> ::teamtalk::Router {
            router.on_command(#command_name, #fn_name)
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn teamtalk_event(attr: TokenStream, item: TokenStream) -> TokenStream {
    let event_attr = parse_macro_input!(attr as EventAttr);
    let function = parse_macro_input!(item as ItemFn);
    let fn_name = function.sig.ident.clone();
    let register_name = format_ident!("register_{}", fn_name);
    let event_expr = event_attr.expr;

    let expanded = quote! {
        #function

        /// Auto-generated registration helper for this event handler.
        pub fn #register_name(router: ::teamtalk::Router) -> ::teamtalk::Router {
            router.on_event(#event_expr, #fn_name)
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn teamtalk_middleware(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);
    let fn_name = function.sig.ident.clone();
    let register_name = format_ident!("register_{}", fn_name);

    let expanded = quote! {
        #function

        /// Auto-generated registration helper for this middleware hook.
        pub fn #register_name(router: ::teamtalk::Router) -> ::teamtalk::Router {
            router.use_middleware_fn(#fn_name)
        }
    };

    expanded.into()
}
