#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

use proc_macro_helpers::global_variables::hardcode::ERROR_ENUM_NAME;
use proc_macro_helpers::global_variables::hardcode::ORIGIN_NAME;
use proc_macro_helpers::global_variables::hardcode::WRAPPER_NAME;

#[proc_macro_derive(ImplGetWhereWasOriginOrWrapperFromTufaCommon)]
pub fn derive_impl_get_where_was_origin_or_wrapper_from_tufa_common(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::TufaCommon)
}

#[proc_macro_derive(ImplGetWhereWasOriginOrWrapperFromCrate)]
pub fn derive_impl_get_where_was_origin_or_wrapper_from_crate(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    generate(input, proc_macro_helpers::path::Path::Crate)
}

fn generate(
    input: proc_macro::TokenStream,
    path: proc_macro_helpers::path::Path,
) -> proc_macro::TokenStream {
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ImplGetWhereWasOriginOrWrapper syn::parse(input) failed");
    let ident = &ast.ident;
    let get_where_was_one_or_many_token_stream =
        format!("{path}::traits::get_where_was_one_or_many::GetWhereWasOriginOrWrapper")
            .parse::<proc_macro2::TokenStream>()
            .expect("path parse failed");
    let where_was_one_or_many_token_stream =
        format!("{path}::common::where_was::WhereWasOriginOrWrapper")
            .parse::<proc_macro2::TokenStream>()
            .expect("path parse failed");
    let where_was_with_addition_token_stream =
        format!("{path}::common::where_was::WhereWasWithAddition")
            .parse::<proc_macro2::TokenStream>()
            .expect("path parse failed");
    match ast.data {
        syn::Data::Union(_) => {
            panic!("ImplGetWhereWasOriginOrWrapper only work on structs!")
        }
        syn::Data::Enum(data_enum) => {
            let variants = data_enum.variants.into_iter().map(|v| {
                let variant_ident = v.ident;
                let ident_as_string = variant_ident.to_string();
                let is_wrapper = if ident_as_string.contains(WRAPPER_NAME)
                    && ident_as_string.contains(ORIGIN_NAME)
                {
                    panic!(
                        "ImplGetSource - ident name {} contains {} and {}",
                        ident_as_string, WRAPPER_NAME, ORIGIN_NAME
                    );
                } else if ident_as_string.contains(WRAPPER_NAME) {
                    true
                } else if ident_as_string.contains(ORIGIN_NAME) {
                    false
                } else {
                    panic!(
                        "ImplGetSource - ident name {} does not contain {} or {}",
                        ident_as_string, WRAPPER_NAME, ORIGIN_NAME
                    );
                };
                match is_wrapper {
                    true => match v.fields {
                        syn::Fields::Unit => {
                            panic!("ImplGetWhereWasOriginOrWrapper still not work with syn::Fields::Unit")
                        }
                        syn::Fields::Named(fields_named) => {
                            let fields_idents = fields_named.named.iter().map(|field| {
                                let field_ident = field
                                    .ident
                                    .clone()
                                    .expect("some of named fields doesnt have ident");
                                quote::quote! { #field_ident }
                            });
                            let where_was_one_or_many_vec =
                                fields_named.named.iter().map(|field| {
                                    let field_ident = field
                                        .ident
                                        .clone()
                                        .expect("some of named fields doesnt have ident");
                                    quote::quote! {
                                        #field_ident
                                        .get_where_was_one_or_many()
                                        .into_vec()
                                        .into_iter()
                                        .for_each(|w| {
                                            vec.push(w);
                                        });
                                    }
                                });
                            quote::quote! {
                                #ident::#variant_ident{
                                    #(#fields_idents,)*
                                } => {
                                    let mut vec = Vec::new();
                                    #(#where_was_one_or_many_vec)*
                                    WhereWasOriginOrWrapper::Many(vec)
                                }
                            }
                        }
                        syn::Fields::Unnamed(_) => quote::quote! {
                            #ident::#variant_ident(e) => e.get_where_was_one_or_many()
                        },
                    },
                    false => match v.fields {
                        syn::Fields::Unit => {
                            panic!("ImplGetWhereWasOriginOrWrapper still not work with syn::Fields::Unit")
                        }
                        syn::Fields::Named(fields_named) => {
                            let fields_idents = fields_named.named.iter().map(|_| {
                                quote::quote! { _ }
                            });
                            quote::quote! {
                                #ident::#variant_ident{
                                    #(#fields_idents,)*
                                } => #where_was_one_or_many_token_stream::None
                            }
                        }
                        syn::Fields::Unnamed(_) => quote::quote! {
                            #ident::#variant_ident(_) => #where_was_one_or_many_token_stream::None
                        },
                    },
                }
            });
            let gen = quote::quote! {
                impl #get_where_was_one_or_many_token_stream for #ident {
                    fn get_where_was_one_or_many(&self) -> #where_was_one_or_many_token_stream {
                        use #where_was_one_or_many_token_stream;//todo maybe dont need it
                        use #where_was_with_addition_token_stream;//todo maybe dont need it
                        match self {
                            #(#variants,)*
                        }
                    }
                }
            };
            gen.into()
        }
        syn::Data::Struct(data_struct) => {
            let ident_as_string = ident.to_string();
            let is_wrapper = if ident_as_string.contains(WRAPPER_NAME)
                && ident_as_string.contains(ORIGIN_NAME)
            {
                panic!(
                    "ImplGetSource - ident name {} contains {} and {}",
                    ident_as_string, WRAPPER_NAME, ORIGIN_NAME
                );
            } else if ident_as_string.contains(WRAPPER_NAME) {
                true
            } else if ident_as_string.contains(ORIGIN_NAME) {
                false
            } else {
                panic!(
                    "ImplGetSource - ident name {} does not contain {} or {}",
                    ident_as_string, WRAPPER_NAME, ORIGIN_NAME
                );
            };
            let get_where_was_one_or_many_token_stream =
                format!("{path}::traits::get_where_was_one_or_many::GetWhereWasOriginOrWrapper")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("path parse failed");
            let where_was_one_or_many_token_stream =
                format!("{path}::common::where_was::WhereWasOriginOrWrapper")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("path parse failed");
            let where_was_with_addition_token_stream =
                format!("{path}::common::where_was::WhereWasWithAddition")
                    .parse::<proc_macro2::TokenStream>()
                    .expect("path parse failed");
            match is_wrapper {
                true => {
                    match data_struct.fields {
                        syn::Fields::Named(fields_named) => {
                            match fields_named.named.len() {
                                2 => {
                                    let source_field_ident = fields_named.named[0].ident.clone().expect("ImplGetWhereWasOriginOrWrapper - there is no first field ident!");
                                    if format!("{}", source_field_ident) != *"source" {
                                        panic!("ImplGetWhereWasOriginOrWrapper - no 'source'-named field found!");
                                    }
                                    match fields_named.named[0].ty.clone() {
                                        syn::Type::Path(type_path) => {
                                            match type_path.path.segments.len() {
                                                1 => {
                                                    let possible_vec_or_hashmap_ident_as_string = format!("{}", type_path.path.segments[0].ident);
                                                    match possible_vec_or_hashmap_ident_as_string.contains(ERROR_ENUM_NAME) {
                                                        true => {
                                                            if possible_vec_or_hashmap_ident_as_string == *"Vec" {
                                                                let gen = quote::quote! {
                                                                    impl #get_where_was_one_or_many_token_stream for #ident
                                                                    {
                                                                        fn get_where_was_one_or_many(&self) -> #where_was_one_or_many_token_stream {
                                                                            use #where_was_one_or_many_token_stream;
                                                                            use #where_was_with_addition_token_stream;
                                                                            let mut vec = Vec::new();
                                                                            self.source.iter().for_each(|e| {
                                                                                e.get_where_was_one_or_many()
                                                                                    .into_vec()
                                                                                    .into_iter()
                                                                                    .for_each(|w| {
                                                                                        vec.push(w);
                                                                                    });
                                                                            });
                                                                            vec.push(WhereWasWithAddition {
                                                                                additional_info: None,
                                                                                where_was: self.where_was.clone(),
                                                                            });
                                                                            WhereWasOriginOrWrapper::Many(vec)
                                                                        }
                                                                    }
                                                                };
                                                                gen.into()
                                                            }
                                                            else if possible_vec_or_hashmap_ident_as_string == *"HashMap" {
                                                                let gen = quote::quote! {
                                                                    impl #get_where_was_one_or_many_token_stream for #ident
                                                                    {
                                                                        fn get_where_was_one_or_many(&self) -> #where_was_one_or_many_token_stream {
                                                                            use #where_was_one_or_many_token_stream;
                                                                            use #where_was_with_addition_token_stream;
                                                                            let mut vec = Vec::new();
                                                                            self.source.iter().for_each(|(key, error)| {
                                                                                error
                                                                               .get_where_was_one_or_many()
                                                                               .into_vec()
                                                                               .into_iter()
                                                                               .for_each(|mut w| {
                                                                                    w.additional_info = Some(format!("{}", key)); //todo
                                                                                    vec.push(w);
                                                                                });
                                                                            });
                                                                            vec.push(WhereWasWithAddition {
                                                                                additional_info: None,
                                                                                where_was: self.where_was.clone(),
                                                                            });
                                                                            WhereWasOriginOrWrapper::Many(vec)
                                                                        }
                                                                    }
                                                                };
                                                                gen.into()
                                                            }
                                                            else {
                                                                let gen = quote::quote! {
                                                                    impl #get_where_was_one_or_many_token_stream for #ident
                                                                    {
                                                                        fn get_where_was_one_or_many(&self) -> #where_was_one_or_many_token_stream {
                                                                            use #where_was_one_or_many_token_stream;
                                                                            use #where_was_with_addition_token_stream;
                                                                            let mut vec = Vec::new();
                                                                            self.source
                                                                            .get_where_was_one_or_many()
                                                                            .into_vec()
                                                                            .into_iter()
                                                                            .for_each(|w| {
                                                                              vec.push(w);
                                                                            });
                                                                            vec.push(WhereWasWithAddition {
                                                                                additional_info: None,
                                                                                where_was: self.where_was.clone(),
                                                                            });
                                                                            WhereWasOriginOrWrapper::Many(vec)
                                                                        }
                                                                    }
                                                                };
                                                                gen.into()
                                                            }
                                                        },
                                                        false => {
                                                            let gen = quote::quote! {
                                                                impl #get_where_was_one_or_many_token_stream for #ident {
                                                                    fn get_where_was_one_or_many(&self) -> #where_was_one_or_many_token_stream {
                                                                        use #where_was_one_or_many_token_stream;
                                                                        use #where_was_with_addition_token_stream;
                                                                        WhereWasOriginOrWrapper::One(
                                                                            #where_was_with_addition_token_stream {
                                                                                additional_info: None,
                                                                                where_was: self.where_was.clone(),
                                                                            },
                                                                        )
                                                                    }
                                                                }
                                                            };
                                                            gen.into()
                                                        },
                                                    }
                                                }
                                                _ => panic!("ImplGetWhereWasOriginOrWrapper only work with type_path.path.segments.len() == 1!"),
                                            }
                                        },
                                        _ => panic!("ImplGetWhereWasOriginOrWrapper only work on Type::Path!")
                                    }
                                }
                                _ => panic!("ImplGetWhereWasOriginOrWrapper only work on structs with 2 named fields!")
                            }
                        }
                        _ => {
                            panic!(
                                "ImplGetWhereWasOriginOrWrapper only work with syn::Fields::Named!"
                            )
                        }
                    }
                }
                false => {
                    let gen = quote::quote! {
                        impl #get_where_was_one_or_many_token_stream for #ident {
                            fn get_where_was_one_or_many(&self) -> #where_was_one_or_many_token_stream {
                                use #where_was_one_or_many_token_stream;
                                WhereWasOriginOrWrapper::One(
                                    #where_was_with_addition_token_stream {
                                        additional_info: None,
                                        where_was: self.where_was.clone(),
                                    },
                                )
                            }
                        }
                    };
                    gen.into()
                }
            }
        }
    }
}
