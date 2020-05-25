use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_roids::{DeriveInputStructExt, FieldExt};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

// Failed.
// custom attribute panicked
// help: message: This macro must be used on a struct.
macro_rules! public_enum_fields {
    ($name:ident, { $( $commonfield:tt )+ }) => {
        #[proc_macro_attribute]
        pub fn $name(_args: TokenStream, item: TokenStream) -> TokenStream {

            let ast = parse_macro_input!(item as DeriveInput);
            let fields = ast.fields();

            let variants = fields.iter()
                .map(|field| {
                    let type_name = field.type_name();
                    let variant_name = type_name.to_string().to_uppercase();
                    let variant_name = Ident::new(&variant_name, Span::call_site());
                    quote! {
                        #variant_name(#type_name)
                    }
                })
                .collect::<Vec<_>>();

            let token_stream = quote! {
                pub enum $name {
                    #(#variants,)*
                    $( $commonfield:tt )+
                }
            };

            token_stream.into()
        }
    };
}

public_enum_fields!(web_event_base, {
    PageLoad,
    PageUnload
});