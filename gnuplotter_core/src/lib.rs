#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub mod required;
pub mod maybe;
pub mod gnu;
mod result;

pub mod prelude {
    pub use super::required::*;
    pub use super::required::required_conversions::*;
    pub use super::maybe::*;
    pub use super::maybe::maybe_conversions::*;
    pub use super::gnu::command::prelude::*;
    pub use super::gnu::dimension::*;
    pub use super::gnu::data::*;
    pub use super::gnu::data::source::*;
    pub use super::gnu::data::source::vector_data_source::*;
    pub use super::gnu::data::serie::*;
    pub use super::gnu::data::series::*;
    pub use super::result::*;
}