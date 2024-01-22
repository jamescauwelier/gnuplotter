use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::process::{Child, Command, Stdio};
use std::ptr::write;
use tempfile::tempfile;

pub struct PlotError(String);

impl From<std::io::Error> for PlotError {
    fn from(value: Error) -> Self {
        PlotError(value.to_string())
    }
}

pub struct Plot {
    gnu: Option<Child>
}

impl Plot {
    pub fn new() -> Self {
        Plot {
            gnu: None
        }
    }

    pub fn draw(&mut self) -> Result<(), PlotError> {

        // what data is available?

        let data_1 = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
        let _data_2: Vec<i32> = data_1
            .iter()
            .map(|v| v * 2)
            .collect();

        // creates a file with the data to be plotted

        let f = tempfile().expect("Unable to create temporary file");
        // let f = File::create("./tmp/data.1.txt").expect("Unable to create file");
        let mut f = BufWriter::new(f);
        // f.write_fmt(format_args!("{}\n", "Hello, world!"))?;

        let mut i = 0;
        for v in data_1 {
            f.write_fmt(format_args!("{} {} {}\n", i, v, v*2))?;
            i += 1;
        }
        f.flush()?;

        let plot = Some(
            Command::new("gnuplot")
                .arg("-p")
                .stdin(Stdio::piped())
                .spawn()
                .expect(
                    "Couldn't spawn gnuplot. Make sure it is installed and available in PATH.",
                )
        ).take();

        self.gnu = plot;
        let stdin = self.gnu.as_mut().unwrap().stdin.as_mut().expect("Failed to access STDIN for GnuPlot");

        writeln!(stdin, "set title '{}'", "an experiment")?;
        writeln!(stdin, "clear")?;
        writeln!(stdin, "set term pngcairo font \"Helvetica,14\" size 600,400")?;
        // writeln!(stdin, "set term pngcairo")?;
        writeln!(stdin, "set tics font \"Helvetica,8\"")?;
        writeln!(stdin, "set output './tmp/output.png'")?;
        writeln!(stdin, "set multiplot")?;
        writeln!(stdin, "set xr [1:10]")?;
        writeln!(stdin, "set yr [1:100]")?;
        writeln!(stdin, "plot \"./tmp/data.1.txt\" using 1:2 title 'A' with linespoint")?;
        writeln!(stdin, "plot \"./tmp/data.1.txt\" using 1:3 title 'B' with linespoint")?;

        // writeln!(stdin, "save 'figure.png'")?;



        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut plot = Plot::new();
        assert!(plot.draw().is_ok());
    }
}