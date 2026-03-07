//! Proc-macro helpers for TeamTalk bot handlers.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, ItemFn, LitStr, Token, parse::Parse, parse_macro_input, punctuated::Punctuated};

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

struct CommandAttr {
    name: LitStr,
    aliases: Vec<LitStr>,
}

impl Parse for CommandAttr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let items = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;
        let mut iter = items.into_iter();
        let name = iter
            .next()
            .ok_or_else(|| syn::Error::new(input.span(), "expected command name"))?;
        let aliases = iter.collect::<Vec<_>>();
        Ok(Self { name, aliases })
    }
}

struct CommandHelpAttr {
    name: LitStr,
    summary: LitStr,
    aliases: Vec<LitStr>,
}

impl Parse for CommandHelpAttr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let items = Punctuated::<LitStr, Token![,]>::parse_terminated(input)?;
        let mut iter = items.into_iter();
        let name = iter
            .next()
            .ok_or_else(|| syn::Error::new(input.span(), "expected command name"))?;
        let summary = iter
            .next()
            .ok_or_else(|| syn::Error::new(input.span(), "expected command summary"))?;
        let aliases = iter.collect::<Vec<_>>();
        Ok(Self {
            name,
            summary,
            aliases,
        })
    }
}

#[proc_macro_attribute]
pub fn teamtalk_command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let command_attr = parse_macro_input!(attr as CommandAttr);
    let command_name = command_attr.name;
    let aliases = command_attr.aliases;
    let function = parse_macro_input!(item as ItemFn);
    let fn_name = function.sig.ident.clone();
    let register_name = format_ident!("register_{}", fn_name);

    let alias_calls = aliases.iter().map(|alias| {
        quote! { router = router.alias_command(#alias, #command_name); }
    });

    let expanded = quote! {
        #function

        /// Auto-generated registration helper for this command handler.
        pub fn #register_name(router: ::teamtalk::Router) -> ::teamtalk::Router {
            let router = router.on_command(#command_name, #fn_name);
            let mut router = router;
            #(#alias_calls)*
            router
        }
    };

    expanded.into()
}

#[proc_macro_attribute]
pub fn teamtalk_command_help(attr: TokenStream, item: TokenStream) -> TokenStream {
    let command_attr = parse_macro_input!(attr as CommandHelpAttr);
    let command_name = command_attr.name;
    let summary = command_attr.summary;
    let aliases = command_attr.aliases;
    let function = parse_macro_input!(item as ItemFn);
    let fn_name = function.sig.ident.clone();
    let register_name = format_ident!("register_{}", fn_name);

    let alias_calls = aliases.iter().map(|alias| {
        quote! { router = router.alias_command(#alias, #command_name); }
    });

    let expanded = quote! {
        #function

        /// Auto-generated registration helper for this command handler.
        pub fn #register_name(router: ::teamtalk::Router) -> ::teamtalk::Router {
            let router = router.on_command_with_help(#command_name, #summary, #fn_name);
            let mut router = router;
            #(#alias_calls)*
            router
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
