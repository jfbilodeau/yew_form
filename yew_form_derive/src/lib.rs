#![recursion_limit = "65536"]
#[macro_use]
extern crate quote;
extern crate syn;

use syn::export::{ToTokens, TokenStream2, TokenStream};

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

    let scalars = vec![ "String", "bool" ];

    let mut fn_fields_list: Vec<TokenStream2> = vec![];
    let mut fn_value_list: Vec<TokenStream2> = vec![];
    let mut fn_set_value_list: Vec<TokenStream2> = vec![];

    for field in &fields {
        let field_ident = field.ident.clone().unwrap();
        let field_name = field_ident.to_string();
        let field_type = match field.ty {
            syn::Type::Path(syn::TypePath { ref path, .. }) => {
                let mut tokens = proc_macro2::TokenStream::new();
                path.to_tokens(&mut tokens);
                tokens.to_string().replace(' ', "")
            }
            _ => panic!("Type `{:?}` of field `{}` is not supported", field.ty, field_ident),
        };
        let is_scalar = scalars.contains(&field_type.as_str());

        let fn_fields = if is_scalar {
            quote! {
                fields.push(
                    String::from(
                        #field_name
                    )
                );
            }
        } else {
            quote! {
                {
                    let mut child_fields: Vec<String> = vec![];
                    self.address.fields(&mut child_fields);
                    // self.#field_name.fields(&mut child_fields);
                    &child_fields.iter().map(|f| format!("{}.{}", #field_name, f)).for_each(|f| fields.push(f));
                }
            }
        };

        let fn_value = match field_type.as_str() {
            "String" => quote! {
                #field_name => self.#field_ident.clone()
            },
            "bool" => quote! {
                #field_name => self.#field_ident.to_string()
            },
            _ => quote! {
                #field_name => self.#field_ident.value(&suffix)
            }
        };

        let fn_set_value = match field_type.as_str() {
            "String" => quote! {
                #field_name => { self.#field_ident = String::from(value); Ok(()) }
            },
            "bool" => quote! {
                #field_name => { self.#field_ident = value == "true"; Ok(()) }
            },
            _ => quote! {
                #field_name => { self.#field_ident.set_value(suffix, value) }
            }
        };

        fn_fields_list.push(fn_fields);
        fn_value_list.push(fn_value);
        fn_set_value_list.push(fn_set_value);
    }

    let struct_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let impl_ast = quote! {
        impl #impl_generics ::yew_form::model::Model for #struct_name #ty_generics #where_clause {
            fn fields(&self, fields: &mut Vec<String>) {
                #(#fn_fields_list)*
            }

            fn value(&self, field_path: &str) -> String {
                let (field_name, suffix) = ::yew_form::split_field_path(field_path);

                match field_name {
                    #(#fn_value_list,)*
                    _ => panic!(format!("Field {} does not exist in {}", field_path, stringify!(#struct_name)))
                }
            }

            fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String> {
                let (field_name, suffix) = ::yew_form::split_field_path(field_path);

                match field_name {
                    #(#fn_set_value_list,)*
                    _ => panic!(format!("Field {} does not exist in {}", field_path, stringify!(#struct_name)))
                }
            }
        }
    };

    impl_ast.into()
}

