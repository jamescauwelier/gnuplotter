#![deny(warnings)]
// #![allow(dead_code, unused_imports)]

pub(crate) mod axis;
mod plot;
pub(crate) mod implement;

extern crate derive_builder;
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input,  ItemStruct};
use crate::axis::derive_axis;
use crate::plot::derive_plot;

#[proc_macro_derive(Axis)]
pub fn axis_macro(input:TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    derive_axis(input)
}



#[proc_macro_derive(Plot)]
pub fn plot_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    derive_plot(input)
}