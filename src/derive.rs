use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Validate)]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_names: Vec<_> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    let field_types: Vec<_> = fields.iter()
        .map(|f| &f.ty)
        .collect();

    let gen = quote! {
        impl schema::clone::CloneAny for #name {
            fn clone_any(&self) -> Box<dyn std::any::Any> {
                Box::new(self.clone())
            }
        }

        impl schema::mapping::FromFields for #name {
            fn from_fields(fields: &std::collections::HashMap<String, Box<dyn std::any::Any>>) -> Option<Self> {
                Some(Self {
                    #(
                        #field_names: fields.get(stringify!(#field_names))?
                            .downcast_ref::<#field_types>()?.clone(),
                    )*
                })
            }
        }
    };

    TokenStream::from(gen)
}