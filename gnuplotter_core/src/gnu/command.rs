pub mod title;
pub mod x_label;
pub mod y_label;

pub mod prelude {
    pub use super::title::*;
    pub use super::x_label::*;
    pub use super::y_label::*;
}

pub struct GnuCommand(String);

pub trait GnuCommandFactory {
    fn as_command(&self) -> GnuCommand;
}