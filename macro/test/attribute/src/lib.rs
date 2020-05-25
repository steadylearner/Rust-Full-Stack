use proc_macro::TokenStream;
use proc_macro_roids::FieldsNamedAppend;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, FieldsNamed};

macro_rules! append_struct {
    ($ast:tt, { $( $commonfieldpub:vis $commonfield:ident: $commonty:ty ),+ $(,)* }) => {
        let fields_additional: FieldsNamed = parse_quote!({ $( $commonfieldpub $commonfield: $commonty, )+ });
        $ast.append_named(fields_additional);
    };
}

#[proc_macro_attribute]
pub fn message_base(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);
    append_struct!(ast, { text: String, });
    TokenStream::from(quote! { #ast })
}

#[proc_macro_attribute]
pub fn user_base(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);
    append_struct!(ast, { pub active: bool, });
    TokenStream::from(quote! { #ast })
}

#[proc_macro_attribute]
pub fn no_field(_args: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as DeriveInput);
    append_struct!(ast, { pub useful: bool, });
    TokenStream::from(quote! { #ast })
}