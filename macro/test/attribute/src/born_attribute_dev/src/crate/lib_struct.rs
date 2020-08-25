use proc_macro::TokenStream;
use proc_macro_roids::{
    FieldsNamedAppend,
    // FieldsUnnamedAppend
};
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, FieldsNamed};

macro_rules! struct_fields {
    (pub $name:ident, { $( $commonfieldpub:vis $commonfield:ident: $commonty:ty ),+ $(,)* }) => {
        #[proc_macro_attribute]
        pub fn $name(_args: TokenStream, item: TokenStream) -> TokenStream {
            let mut ast = parse_macro_input!(item as DeriveInput);
            let common_fields: FieldsNamed = parse_quote!({ $( $commonfieldpub $commonfield: $commonty, )+ });
            ast.append_named(common_fields);
            TokenStream::from(quote! { #ast })
        }
    };
    ($name:ident, { $( $commonfield:ident: $commonty:ty ),+ $(,)* }) => {
        #[proc_macro_attribute]
        pub fn $name(_args: TokenStream, item: TokenStream) -> TokenStream {
            let mut ast = parse_macro_input!(item as DeriveInput);
            let common_fields: FieldsNamed = parse_quote!({ $( $commonfield: $commonty, )+ });
            ast.append_named(common_fields);
            TokenStream::from(quote! { #ast })
        }
    };
}

struct_fields!(pub message_base, {
    text: String,
});
struct_fields!(pub user_base, { pub active: bool, });
struct_fields!(no_field, { useful: bool, });
