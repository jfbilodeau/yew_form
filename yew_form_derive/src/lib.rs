#![recursion_limit = "65536"]
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let fields: Vec<syn::Field> = match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                panic!("#[derive(Model)] struct cannot have unnamed field");
            }
            fields.iter().cloned().collect()
        }
        _ => panic!("#[derive(Model)] can only be used with structs"),
    };

    let mut field_idents: Vec<syn::Ident> = vec![];
    let mut field_names: Vec<String> = vec![];
    let mut field_types: Vec<String> = vec![];

    for field in &fields {
        let field_ident = field.ident.clone().unwrap();
        let field_name = field_ident.to_string();
        let field_type = match field.ty {
            syn::Type::Path(syn::TypePath { ref path, .. }) => {
                let mut tokens = proc_macro2::TokenStream::new();
                path.to_tokens(&mut tokens);
                tokens.to_string().replace(' ', "")
            }
            _ => panic!(
                "Type `{:?}` of field `{}` is not supported",
                field.ty, field_ident
            ),
        };

        field_idents.push(field_ident);
        field_names.push(field_name);
        field_types.push(field_type);
    }

    let struct_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let impl_ast = quote! {
        impl #impl_generics ::yew_form::model::Model for #struct_name #ty_generics #where_clause {
        }

        impl #impl_generics ::yew_form::model::FormValue for #struct_name #ty_generics #where_clause {
            fn fields(&self, prefix: &str, fields: &mut Vec<String>) {
                let field_prefix = if prefix == "" {
                    String::new()
                } else {
                    format!("{}.", prefix)
                };

                #(
                let field_path = format!("{}{}", field_prefix, #field_names);
                self.#field_idents.fields(&field_path, fields);
                )*
            }

            fn value(&self, field_path: &str) -> String {
                let (field_name, suffix) = ::yew_form::split_field_path(field_path);

                match field_name {
                    #(
                    #field_names => self.#field_idents.value(suffix),
                    )*
                    _ => panic!(format!("Field {} does not exist in {}", field_path, stringify!(#struct_name)))
                }
            }

            fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String> {
                let (field_name, suffix) = ::yew_form::split_field_path(field_path);

                match field_name {
                    #(
                    #field_names => self.#field_idents.set_value(suffix, value),
                    )*
                    _ => panic!(format!("Field {} does not exist in {}", field_path, stringify!(#struct_name)))
                }
            }
        }
    };

    impl_ast.into()
}
