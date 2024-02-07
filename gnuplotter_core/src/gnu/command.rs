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

#[derive(Debug, Clone, PartialEq)]
pub struct GnuCommand(String);

impl From<GnuCommand> for String {
    fn from(command: GnuCommand) -> Self {
        command.0
    }
}

impl GnuCommand {
    pub(crate) fn new<T>(command: T) -> Self
    where
        String: From<T>
    {
        GnuCommand(command.into())
    }
}

pub trait GnuCommandFactory {
    fn as_commands(&self) -> Result<VecDeque<GnuCommand>>;
}

impl<T> GnuCommandFactory for Required<T>
where
    T: GnuCommandFactory
{
    fn as_commands(&self) -> Result<VecDeque<GnuCommand>> {
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
    fn as_commands(&self) -> Result<VecDeque<GnuCommand>> {
        match self {
            Maybe::Nothing => Ok(vec![].into()),
            Maybe::Value(value) => value.as_commands()
        }
    }
}