use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(QueryResult)]
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
        impl<'a, T: Tree + 'a> QueryResult<'a, T> for #struct_name {
            fn from_nodes(queries: Queries, nodes: Box<dyn DynNodes<T> + 'a>) -> Self {
                #(let mut #field_name: Option<#field_type> = None;)*
                for (name, queries) in queries {
                    match name.as_ref().map(String::as_str) {
                        #(Some(stringify!(#field_name)) => {
                            #field_name = Some(<#field_type>::from_nodes(
                                once((None, queries)).collect(),
                                nodes.clone(),
                            ))
                        },)*
                        _ => (),
                    }
                }
                Self {
                    #(#field_name: #field_name.unwrap(),)*
                }
            }

            fn from_node(queries: Queries, node: T::Node) -> Self {
                Self::from_nodes(queries, Box::new(node.tree()) as Box<dyn DynNodes<T>>)
            }

            fn from_leaf(_node: T::Node) -> Self {
                panic!("Cannot create object from leaf.")
            }
        }
    };

    TokenStream::from(expanded)
}
