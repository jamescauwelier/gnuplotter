use proc_macro::TokenStream;

use syn::{ItemStruct};
use quote::{ quote};
use crate::implement::getters::getters;
use crate::implement::gnu_command_factory::gnu_command_factory;

pub fn derive_axis(item: ItemStruct) -> TokenStream {

    let getters = getters(&item);
    let command_factory = gnu_command_factory(&item);

    TokenStream::from(
        quote! {
            #[doc = "Expansions for an Axis type: #name"]
            #getters
            #command_factory
        }
    )
}