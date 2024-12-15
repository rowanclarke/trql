use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(FromNodes)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    let (field_name, field_type): (Vec<_>, Vec<_>) = if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields_named) = data_struct.fields {
            fields_named
                .named
                .into_iter()
                .map(|field| {
                    let field_name = field.ident.unwrap();
                    let field_type = field.ty;
                    (field_name, field_type)
                })
                .unzip()
        } else {
            panic!("Only named fields are supported.");
        }
    } else {
        panic!("FromNodes macro only supports structs.");
    };

    let expanded = quote! {
        impl<T: Tree + 'static> FromNodes<T> for #struct_name {
            fn from_nodes(queries: Vec<Query>, nodes: Box<dyn DynNodes<T>>) -> Self {
                #(let mut #field_name: Option<#field_type> = None;)*
                for query in queries {
                    match query.into_named() {
                        #(
                        Some((name, select, subqueries)) if name == stringify!(#field_name) => {
                            #field_name = Some(String::from_nodes(
                                subqueries,
                                select.execute::<T, _>(nodes.clone()),
                            ))
                        },
                        )*
                        _ => (),
                    }
                }
                Self {
                    #(#field_name: #field_name.unwrap(),)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
