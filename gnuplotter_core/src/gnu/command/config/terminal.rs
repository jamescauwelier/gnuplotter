use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use crate::prelude::*;

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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PngCairo {
    size: PngCairoSize,
    font: PngCairoFont
}

impl Terminal for PngCairo {}

impl GnuCommandFactory for PngCairo {
    fn as_commands(&self) -> Result<VecDeque<GnuCommand>> {
        Ok(vec![
            GnuCommand::new(format!("set term pngcairo enhanced {} {}", self.size, self.font))
        ].into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_png_cairo_terminal_creation() {
        let mut terminal = PngCairo::default();
        terminal.font.update("Helvetica", 14);
        terminal.size.update(1200, 800);
        let commands = terminal.as_commands().unwrap();

        assert_eq!(commands.len(), 1);
        assert_eq!(commands[0].to_string(), "set term pngcairo enhanced size 1200,800 font \"Helvetica,14\"");
    }
}