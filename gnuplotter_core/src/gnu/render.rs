use std::collections::VecDeque;
use std::io::Write;
use std::process::{Command, Stdio, Child, ChildStdin};
use crate::prelude::*;

pub struct Render;

pub enum RenderError {
    WritingCommandFailed,
    GnuSpawnFailed,
    GnuSTDINNotAccessible,
    CannotFlushSTDIN,
    CouldNotKillGnuPlot,
    WaitingForGnuPlotFailed
}

impl From<RenderError> for String {
    fn from(value: RenderError) -> Self {
        match value {
            RenderError::WritingCommandFailed => "Failed to write command to GnuPlot",
            RenderError::GnuSpawnFailed => "Failed to spawn GnuPlot. Verify it is correctly installed and available",
            RenderError::GnuSTDINNotAccessible => "GnuPlot STDIN cannot be accessed",
            RenderError::CannotFlushSTDIN => "Flushing STDIN failed",
            RenderError::CouldNotKillGnuPlot => "Could not kill Gnu Plot process",
            RenderError::WaitingForGnuPlotFailed => "Waiting for Gnu Plot failed"
        }.into()
    }
}

impl Render {
    pub fn render(commands: VecDeque<GnuCommand>) -> Result<()> {

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
        gnu.kill().or(Err(RenderError::CouldNotKillGnuPlot))?;

        Ok(())
    }
}

