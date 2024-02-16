use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter, Write};
use crate::prelude::*;

pub mod title;
pub mod axis;
pub mod config;

pub mod prelude {
    pub use super::title::*;
    pub use super::axis::*;
    pub use super::axis::label::*;
    pub use super::config::*;
    pub use super::config::terminal::*;
    pub use super::config::filename::*;

    pub use super::*;
}

#[derive(Debug, Clone, PartialEq)]
pub struct GnuCommand(String);

impl Display for GnuCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.clone())
    }
}

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

pub enum GnuCommandFactoryError {
    RequiredValueMissing(String),
    IOError(String),
    Message(String),
}

impl GnuCommandFactoryError {
    pub fn required_value_missing(msg: &str) -> GnuCommandFactoryError {
        GnuCommandFactoryError::RequiredValueMissing(msg.into())
    }

    pub fn io_error(msg: &str) -> GnuCommandFactoryError {
        GnuCommandFactoryError::IOError(msg.into())
    }

    pub fn message(msg: &str) -> GnuCommandFactoryError {
        GnuCommandFactoryError::Message(msg.into())
    }
}

impl Debug for GnuCommandFactoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GnuCommandFactoryError::RequiredValueMissing(msg) =>
                f.write_str(msg),
            GnuCommandFactoryError::IOError(msg) =>
                f.write_str(msg),
            GnuCommandFactoryError::Message(msg) =>
                f.write_str(msg),
        }
    }
}

pub type GnuCommandFactoryResult = std::result::Result<VecDeque<GnuCommand>, GnuCommandFactoryError>;

pub trait GnuCommandFactory {
    fn as_commands(&self) -> GnuCommandFactoryResult;
}

impl<T> GnuCommandFactory for Required<T>
where
    T: GnuCommandFactory
{
    fn as_commands(&self) -> GnuCommandFactoryResult {
        match self {
            Required::Missing => Err(
                GnuCommandFactoryError::required_value_missing("A required value is missing, but needs a value when producing GnuCommands")
            ),
            Required::Value(value) => value.as_commands()
        }
    }
}

impl<T> GnuCommandFactory for Maybe<T>
    where
        T: GnuCommandFactory
{
    fn as_commands(&self) -> GnuCommandFactoryResult {
        match self {
            Maybe::Nothing => Ok(vec![].into()),
            Maybe::Value(value) => value.as_commands()
        }
    }
}