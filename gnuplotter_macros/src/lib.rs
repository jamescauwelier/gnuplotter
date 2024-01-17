// #![deny(warnings)]
#![allow(dead_code, unused_imports)]

pub(crate) mod axis;

#[macro_use]
extern crate derive_builder;
extern crate proc_macro;
use proc_macro::TokenStream;
use std::any::Any;
use quote::{format_ident, quote, TokenStreamExt};
use syn::{parse_macro_input, DeriveInput, Attribute};
use crate::axis::create_axis_expansions;
// use gnuplotter_core::derive::axis::create_axis_expansions;

#[proc_macro_derive(Axis)]
pub fn derive_axis(input:TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // dbg!(&input);

    create_axis_expansions(input)
}

#[proc_macro_derive(Plot)]
pub fn derive_plot(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut command_appenders = quote! {};
    let (info, definitions, implementations) = match input.data {
        syn::Data::Struct(s) => {

            let mut definitions = quote!{
                fn plot(&mut self);

                // a simple comment
            };
            let mut implementations = quote!{
                // a simple comment
            };
            for field in s.fields {

                let field_name = field.ident.unwrap();
                let field_type = field.ty;

                command_appenders = quote!{
                    #command_appenders
                    commands.append(&mut self.#field_name.as_commands());
                };

                definitions = quote!{
                    #definitions
                    fn #field_name<T>(&mut self, value: T) -> &mut Self
                        where #field_type: From<T>;
                };

                implementations = quote! {
                    #implementations

                    fn #field_name<T>(&mut self, value: T) -> &mut Self
                    where
                        #field_type: From<T>
                    {
                        self.#field_name = value.into();
                        self
                    }
                };
            }

            (format!("struct: {}", name), definitions, implementations)
        }
        syn::Data::Enum(_e) => {
            ("A plot can only be constructed on a struct, not an enum.".to_string(), quote!("// setters"), quote!("// setters"))
        },
        syn::Data::Union(_u) => {
            ("A plot can only be constructed on a struct, not a union.".to_string(), quote!("// setters"), quote!("// setters"))
        },
    };

    let builder = format_ident!("{}Builder", name);

    let expanded = quote! {

        use ::std::collections::VecDeque;
        use gnuplotter::prelude::*;

        trait #builder {
            #definitions
        }

        impl #builder for #name {

            #implementations

            fn plot(&mut self) {
                println!(#info);
                println!("Plotting...");
            }
        }

        impl GnuCommandFactory for #name {
            fn as_commands(&self) -> VecDeque<GnuCommand> {
                let mut commands = VecDeque::new();
                #command_appenders

                commands
            }
        }
    };

    TokenStream::from(expanded)
}