use proc_macro::TokenStream;
// use std::any::Any;
use syn::{DeriveInput, WherePredicate};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
// use proc_macro2::TokenStream;

fn struct_fields(input: &DeriveInput) -> Vec<(syn::Ident, syn::Type)> {
    println!("Inspecting struct fields");
    match &input.data {
        syn::Data::Struct(s) => {
            let mut fields = vec![];
            for field in s.fields.clone().into_iter() {
                let field_name = field.ident.unwrap();
                let field_type = field.ty;
                fields.push((field_name, field_type));
            }

            fields
        },
        _ => {
            panic!("Only structs can be inspected for their fields.");
        }
    }
}

pub fn create_axis_expansions(input: DeriveInput) -> TokenStream {

    if &input.attrs.len() == &0usize {
        println!("No attributes found");
    } else {
        for a in &input.attrs {
            println!("Attribute: {}", a.to_token_stream().to_string());
        }
    }

    let generics_info = &input.generics.params.clone().into_token_stream();
    let where_info = &input.generics.where_clause.clone().into_token_stream();

    let name = &input.ident;
    let comment = format!("Expansions for an Axis type: {}", name);

    let mut command_appenders = quote! {
        // appenders
    };

    for (field_name, _field_type) in struct_fields(&input) {
        command_appenders = quote!{
            #command_appenders
            commands.append(&mut self.#field_name.as_commands());
        };
    }

    println!("Implementing GnuCommandFactory for {}", name);
    let _command_impl = quote! {
        impl<#generics_info> GnuCommandFactory for #name<#generics_info> #where_info {
            fn as_commands(&self) -> VecDeque<GnuCommand> {
                let mut commands = VecDeque::new();
                #command_appenders

                commands
            }
        }
    };

    TokenStream::from(
        quote! {
            #[doc = #comment]

            // use ::std::collections::VecDeque;
            use gnuplotter::prelude::*;

            trait Something {
                fn something(&self);
            }

            impl <D> Something for CustomAxis <D> where D: Dimension  {
                fn something(&self) {
                    println!("Something");
                }
            }
        }
    )
}