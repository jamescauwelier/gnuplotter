use std::collections::VecDeque;
use crate::prelude::*;

pub mod title;
pub mod axis;

pub mod prelude {
    pub use super::title::*;
    pub use super::axis::*;
    pub use super::axis::label::*;

    pub use super::*;
}

pub struct GnuCommand(String);

impl GnuCommand {
    pub(crate) fn new(command: String) -> Self {
        GnuCommand(command)
    }
}

pub trait GnuCommandFactory {
    fn as_commands(&self) -> VecDeque<GnuCommand>;
}

impl<T> GnuCommandFactory for Required<T>
where
    T: GnuCommandFactory
{
    fn as_commands(&self) -> VecDeque<GnuCommand> {
        match self {
            Required::Missing => panic!("A required value must be present before commands can be generated."),
            Required::Value(value) => value.as_commands()
        }
    }
}

impl<T> GnuCommandFactory for Maybe<T>
    where
        T: GnuCommandFactory
{
    fn as_commands(&self) -> VecDeque<GnuCommand> {
        match self {
            Maybe::Nothing => vec![].into(),
            Maybe::Value(value) => value.as_commands()
        }
    }
}