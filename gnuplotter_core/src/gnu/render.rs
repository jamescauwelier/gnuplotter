use std::collections::VecDeque;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::process::{Command, Stdio, Child, ChildStdin};
use crate::prelude::*;

pub struct Render;

/// Documents in which ways the rendering of a plot can fail. Rendering would most likely fail if
/// gnuplot is not installed or reachable for some reason. There is no error associated with errors
/// in gnuplot syntax. The only thing these errors indicate is some kind of success or error in the
/// process of communicating with gnuplot.
pub enum RenderError {
    /// When calling `.as_commands()` fails with an error message
    CommandGenerationFailed(String),
    /// Sending the command by writing to the STDIN of the command failed. This does not indicate an error in your command.
    WritingCommandFailed,
    /// Spawning the child process for gnuplot failed. Verify you have Gnuplot installed and available.
    GnuSpawnFailed,
    /// Unable to access the STDIN for the gnuplot child process
    GnuSTDINNotAccessible,
    /// We need to wait for gnuplot to finish rendering before killing it. This indicates that this
    /// waiting failed. This might indicate an error in gnuplot command syntax that gnuplot is
    /// escalating.
    WaitingForGnuPlotFailed
}

impl Display for RenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            RenderError::CommandGenerationFailed(msg) => format!("Command generation failed: {}", msg),
            RenderError::WritingCommandFailed => "Failed to `write` command to GnuPlot".into(),
            RenderError::GnuSpawnFailed => "Failed to spawn GnuPlot. Verify it is correctly installed and available".into(),
            RenderError::GnuSTDINNotAccessible => "GnuPlot STDIN cannot be accessed".into(),
            RenderError::WaitingForGnuPlotFailed => "Waiting for Gnu Plot failed. Check your command syntax for errors.".into()
        };
        f.write_fmt(format_args!("{}", msg))
    }
}

impl Debug for RenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl From<GnuCommandFactoryError> for RenderError {
    fn from(value: GnuCommandFactoryError) -> Self {
        match value {
            GnuCommandFactoryError::Message(msg) =>
                RenderError::CommandGenerationFailed(msg),
            GnuCommandFactoryError::RequiredValueMissing(msg) =>
                RenderError::CommandGenerationFailed(msg),
            GnuCommandFactoryError::IOError(msg) =>
                RenderError::CommandGenerationFailed(msg),
        }
    }
}

pub type RenderResult<T> = std::result::Result<T, RenderError>;

impl Render {
    pub fn render(commands: VecDeque<GnuCommand>) -> RenderResult<()> {

        let commands: Vec<GnuCommand> = commands.into();

        let mut gnu = Command::new("gnuplot")
            .arg("-p")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|_| RenderError::GnuSpawnFailed)?;
        let stdin = gnu.stdin.as_mut().ok_or(RenderError::GnuSTDINNotAccessible)?;

        writeln!(stdin, "clear").or(Err(RenderError::WritingCommandFailed))?;
        for command in commands {
            writeln!(stdin, "{}", command).or(Err(RenderError::WritingCommandFailed))?;
        }
        writeln!(stdin, "exit").or(Err(RenderError::WritingCommandFailed))?;

        gnu.wait().or(Err(RenderError::WaitingForGnuPlotFailed))?;

        Ok(())
    }
}

pub trait CanRender: GnuCommandFactory {
    fn render(&self) -> RenderResult<()> {
        let commands = self.as_commands()?;
        Render::render(commands)
    }
}