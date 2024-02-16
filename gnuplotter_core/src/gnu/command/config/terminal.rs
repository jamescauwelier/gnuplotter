use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::prelude::*;
use crate::prelude::prelude::GnuCommandFactoryResult;

trait Terminal : GnuCommandFactory {}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum PngCairoFont {
    #[default]
    Default,
    Custom {
        name: String,
        size: u32
    }
}

impl PngCairoFont {
    pub fn custom(name: &str, size: usize) -> Self {
        PngCairoFont::Custom {
            name: name.into(),
            size: size as u32
        }
    }

    pub fn update(&mut self, name: &str, size: usize) {
        std::mem::swap(self, &mut PngCairoFont::custom(name, size));
    }
}

impl Display for PngCairoFont {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PngCairoFont::Default => f.write_str(""),
            PngCairoFont::Custom { name, size} => {
                f.write_fmt(
                    format_args!("font \"{},{}\"", name, size)
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum PngCairoSize {
    #[default]
    Missing,
    Value {
        width: usize,
        height: usize
    }
}

impl PngCairoSize {
    pub fn update(&mut self, width: usize, height: usize) {
        std::mem::swap(self, &mut PngCairoSize::Value { width, height });
    }
}

impl Display for PngCairoSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PngCairoSize::Missing => f.write_str(""),
            PngCairoSize::Value { width, height} => {
                f.write_fmt(
                    format_args!("size {},{}", width, height)
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum PngCairoOutput {
    #[default]
    Missing,
    Filename(String)
}

impl PngCairoOutput {
    pub fn update(&mut self, filename: &str) {
        std::mem::swap(self, &mut PngCairoOutput::Filename(filename.into()))
    }
}

impl GnuCommandFactory for PngCairoOutput {
    fn as_commands(&self) -> GnuCommandFactoryResult {
        match self {
            PngCairoOutput::Missing =>
                panic!("PNGCairo requires an output file: TERMINAL.output().update(\"./filename.png\")"),
            PngCairoOutput::Filename(filename) => {
                Ok(vec![GnuCommand::new(format!("set output '{}'", filename))].into())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PngCairo {
    size: PngCairoSize,
    font: PngCairoFont,
    output: PngCairoOutput
}

impl PngCairo {
    pub fn font(&mut self) -> &mut PngCairoFont {
        &mut self.font
    }

    pub fn size(&mut self) -> &mut PngCairoSize {
        &mut self.size
    }

    pub fn output(&mut self) -> &mut PngCairoOutput {
        &mut self.output
    }
}

impl Terminal for PngCairo {}

impl GnuCommandFactory for PngCairo {
    fn as_commands(&self) -> GnuCommandFactoryResult {
        let mut commands = VecDeque::new();
        commands.append(&mut self.output.as_commands()?);
        commands.push_back(
            GnuCommand::new(format!("set term pngcairo enhanced {} {}", self.size, self.font).trim())
        );


        Ok(commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_cairo_terminal_creation() {
        let mut terminal = PngCairo::default();
        terminal.font().update("Helvetica", 14);
        terminal.size().update(1200, 800);
        terminal.output().update("./result.png");
        let commands = terminal.as_commands().unwrap();

        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].to_string(), "set output './result.png'");
        assert_eq!(commands[1].to_string(), "set term pngcairo enhanced size 1200,800 font \"Helvetica,14\"");
    }

    #[test]
    fn test_png_cairo_terminal_empty_creation() {
        let mut terminal = PngCairo::default();
        terminal.output().update("./result.png");
        let commands = terminal.as_commands().unwrap();

        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].to_string(), "set output './result.png'");
        assert_eq!(commands[1].to_string(), "set term pngcairo enhanced");
    }

    #[test]
    #[should_panic(expected = "PNGCairo requires an output file: TERMINAL.output().update(\"./filename.png\")")]
    fn test_png_cairo_terminal_creation_without_filename_panics() {
        let terminal = PngCairo::default();
        let commands = terminal.as_commands().unwrap();

        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].to_string(), "set output './result.png'");
        assert_eq!(commands[1].to_string(), "set term pngcairo enhanced");
    }
}