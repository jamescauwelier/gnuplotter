use std::collections::VecDeque;
use crate::gnu::command::config::terminal::PngCairo;
use crate::prelude::{GnuCommand, GnuCommandFactory};
use crate::prelude::prelude::GnuCommandFactoryResult;

pub mod terminal;
pub mod filename;

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
    fn as_commands(&self) -> GnuCommandFactoryResult {
        let mut commands: VecDeque<GnuCommand> = vec![].into();
        commands.append(&mut self.terminal.as_commands()?);

        Ok(commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_config_creation_panics() {
        let config = Config::default();
        let commands = config.as_commands();

        assert_eq!(commands.unwrap().pop_front().unwrap().to_string(), "set term pngcairo enhanced");
    }

    #[test]
    fn test_config_creation_with_output() {
        let mut config = Config::default();
        config.terminal.output().update("./some-file.png");
        let commands = config.as_commands().unwrap();

        assert_eq!(commands[1].to_string(), "set term pngcairo enhanced");
    }
}