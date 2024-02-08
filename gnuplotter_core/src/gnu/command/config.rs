use std::collections::VecDeque;
use crate::gnu::command::config::terminal::PngCairo;
use crate::prelude::{GnuCommand, GnuCommandFactory};

pub mod terminal;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Config {
    terminal: PngCairo
}

impl Config {
    pub fn terminal(&mut self) -> &mut PngCairo {
        &mut self.terminal
    }
}

impl GnuCommandFactory for Config {
    fn as_commands(&self) -> crate::prelude::Result<VecDeque<GnuCommand>> {
        let mut commands: VecDeque<GnuCommand> = vec![].into();
        commands.append(&mut self.terminal.as_commands()?);

        Ok(commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::default();
        let commands = config.as_commands();

        assert_eq!(commands.unwrap().pop_front().unwrap().to_string(), "set term pngcairo enhanced");
    }
}