use proc_macro::TokenStream;
use proc_macro::TokenTree::Punct;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};

use std::any::Any;
use std::collections::HashSet;
use syn::{DeriveInput, Type, WherePredicate};
use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use gnuplotter_core::prelude::Required;

#[derive(PartialEq, Eq, Debug, Hash)]
enum Command {
    Label,
    Axis,
    Title
}

impl TryFrom<&str> for Command {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "label" => Ok(Command::Label),
            "axis" => Ok(Command::Axis),
            "title" => Ok(Command::Title),
            _ => Err("Unknown command")

        }
    }
}


fn extract_command(stream: TokenStream2) -> Option<Command> {
    for token in stream {
        if let TokenTree::Ident(ident) = token {
            if let Ok(command) = Command::try_from(ident.to_string().as_str()) {
                return Some(command);
            }
        }
    }

    None
}

fn chained_type_notation(stream: TokenStream2) -> TokenStream2 {
    let mut s = String::new();
    for token in stream {
        match token {
            TokenTree::Ident(ident) => {
                s += &ident.to_string();
            },
            TokenTree::Punct(punct) => {
                match punct.as_char() {
                    '<' => s += "::<",
                    '>' => s += ">",
                    _ => {}
                }
            },
            _ => { }
        };
    }

    s.parse().unwrap()
}

fn struct_fields(input: &DeriveInput) -> Vec<(syn::Ident, syn::Type, Command)> {
    match &input.data {
        syn::Data::Struct(s) => {
            let mut fields = vec![];
            for field in s.fields.clone().into_iter() {
                let field_name = field.ident.unwrap();
                if let Some(field_type_command) = extract_command(field.ty.clone().into_token_stream()) {
                    fields.push((field_name, field.ty.clone(), field_type_command));
                }

            }

            fields
        },
        _ => {
            panic!("Only structs can be inspected for their fields.");
        }
    }
}

pub fn create_axis_expansions(input: DeriveInput) -> TokenStream {

    let generics_info = &input.generics.params.clone().into_token_stream();
    let generics_info = match generics_info.is_empty() {
        true => quote! {},
        false => quote! {<#generics_info>},
    };
    let where_info = &input.generics.where_clause.clone().into_token_stream();

    let name = &input.ident;
    let comment = format!("Expansions for an Axis type: {}", name);

    // creates an implementation with setters for each field

    let mut trait_methods = quote! {};
    let mut trait_method_implementations = quote! {};
    let mut allowed_setter_types = HashSet::new();
    allowed_setter_types.insert(Command::Label);

    for (field_name, field_type, field_type_command) in struct_fields(&input) {

        if allowed_setter_types.contains(&field_type_command) {
            println!("Adding setter for field: {}", field_name);
            trait_methods = quote! {
                #trait_methods
                fn #field_name(&mut self) -> &mut #field_type;
            };
            trait_method_implementations = quote! {
                #trait_method_implementations
                fn #field_name(&mut self) -> &mut #field_type {
                    &mut self.#field_name
                }
            };
        } else {
            println!("Skipping setter for field: {}, of type: {:?}", field_name, field_type_command);
        }

    }

    let trait_name = format_ident!("{}DerivedAxis", name);
    let axis_trait = quote! {
        trait #trait_name {
            #trait_methods
        }
    };
    let axis_impl = quote! {
        impl #generics_info #trait_name for #name #generics_info #where_info {
            #trait_method_implementations
        }
    };

    // creates appenders combining gnu commands from each field

    let mut command_factory_combiner = quote! {
        // appenders
    };

    for (field_name, _field_type, _field_type_command) in struct_fields(&input) {
        println!("Adding appender for field: {}", field_name);
        command_factory_combiner = quote!{
            #command_factory_combiner
            commands.append(&mut self.#field_name.as_commands());
        };
    }

    println!("Implementing GnuCommandFactory for {}", name);
    let command_factory_impl = quote! {
        impl #generics_info  GnuCommandFactory for #name #generics_info #where_info {
            fn as_commands(&self) -> VecDeque<GnuCommand> {
                let mut commands = VecDeque::new();
                #command_factory_combiner

                commands
            }
        }
    };

    TokenStream::from(
        quote! {
            #[doc = #comment]
            #command_factory_impl
            #axis_trait
            #axis_impl
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chaining_types() {
        let something = quote!(Required<Label<X>>);
        let chained = chained_type_notation(something).to_string();

        assert_eq!(chained, "Required ::< Label ::< X >>");
    }
}