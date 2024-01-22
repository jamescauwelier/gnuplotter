use std::collections::VecDeque;
use crate::prelude::{GnuCommand, GnuCommandFactory};

pub mod vector_data_source;

pub trait DataSource {
    fn data(&self) -> & Vec<(i32, f64)>;
}

impl GnuCommandFactory for dyn DataSource {
    fn as_commands(&self) -> VecDeque<GnuCommand> {
        vec![
            GnuCommand::new("some command")
        ].into()
    }
}