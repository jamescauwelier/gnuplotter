use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct};
use crate::implement::getters::getters;
use crate::implement::gnu_command_factory::gnu_command_factory;


pub fn derive_plot(item: ItemStruct) -> TokenStream {

    let struct_name = &item.ident;
    let command_factory_implementation = gnu_command_factory(&item);
    let mutable_getters = getters(&item);

    TokenStream::from(
        quote! {

            use ::std::collections::VecDeque;
            use gnuplotter::prelude::*;

            #command_factory_implementation

            impl CanRender for #struct_name {}

            #mutable_getters
        }
    )
}