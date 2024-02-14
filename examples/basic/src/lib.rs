//! # Basic example
//!
//! A basic example on how to construct a 2D plot using this library.
//!
//! ## Declare a plot
//!
//! We declare a `BasicPlot` by providing private properties describing underlying gnuplot features.
//! We only need to declare what we need. Other features of gnuplot can be safely ignored.
//!
//! ```
//! use gnuplotter::prelude::*;
//!
//! #[derive(Clone, PartialEq, Debug, Default, Plot)]
//! pub struct BasicPlot {
//!     config: Config,
//!     title: Maybe<Title>,
//!     // x: XAxis, --> uncomment in later step
//!     // y: YAxis, --> uncomment in later step
//!     series: Series<f64>,
//! }
//!
//! impl BasicPlot {
//!     pub fn new() -> Self {
//!         BasicPlot::default()
//!     }
//! }
//! ```
//!
//! It is essential to mark a new plot with a `Plot` derive macro. This will generate getter methods
//! allowing to mutate the underlying values:
//!
//! ```
//! # use crate::basic::*;
//! # let mut plot = BasicPlot::new();
//! plot.title().update("New title");
//! ```
//!
//! In addition, the same macro implements the `GnuCommandFactory` trait, and can be used like so:
//!
//! ```
//! # use gnuplotter::prelude::*;
//! # use crate::basic::*;
//! # let mut plot = BasicPlot::new();
//! # plot.config().terminal().output().update("./.tmp/output.png");
//! # plot.x().label().update("X");
//! let commands = plot.as_commands();
//! ```
//!
//! Normally, you shouldn't need to manipulate the commands, although they can be useful for testing
//! when adding your own custom features. Any custom field added to an axis or plot needs to
//! implement the `GnuCommandFactory` or the generated code will fail.
//!
//! ## Declare axis elements
//!
//! Sometimes, a hierarchy of elements makes sense. In our basic plot, two axis were used: X and Y.
//! But since these are more complex, compositions of elements, they need to be defined first.
//!
//! ```
//! use std::collections::VecDeque;
//! use gnuplotter::prelude::*;
//!
//! #[derive(Clone, PartialEq, Eq, Debug, Default, Axis)]
//! pub struct XAxis
//! {
//!     label: Required<Label<X>>
//! }
//!
//! #[derive(Clone, PartialEq, Eq, Debug, Default, Axis)]
//! pub struct YAxis
//! {
//!     label: Maybe<Label<Y>>
//! }
//! ```
//!
//! In the definitions, an `Axis` macro is used that generates getter methods for the fields that
//! return mutable references. In the case of the above examples, it allows us to update the label
//! text.
//!
//! ## Instantiate your basic plot
//!
//! Now, the basic plot can be used.
//!
//! ```
//! use std::collections::VecDeque;
//! use gnuplotter::prelude::*;
//! use crate::basic::*;
//!
//! let mut plot = BasicPlot::new();
//! plot.config().terminal().output().update("./.tmp/output.png");
//! plot.x().label().update("X");
//! plot.y().label().update("Y");
//! plot.title().update("A basic plot");
//!
//! // we expect 5 gnuplot commands in total:
//! // - 2 for the terminal and output configuration
//! // - 1 to set the x label
//! // - 1 to set the y label
//! // - 1 to set the title
//! assert_eq!(5, plot.as_commands().unwrap().len());
//! ```
//!
//! ## Rendering
//!
//! One other functionality that is implemented by the `Plot` derive macro, is rendering. The
//! implementation uses the `plot.as_commands()` method to extract all commands and send them to
//! a gnuplot process.
//!
//! ```
//! use std::collections::VecDeque;
//! use gnuplotter::prelude::*;
//! use crate::basic::*;
//!
//! // initialize the plot
//! let mut plot = BasicPlot::new();
//!
//! // configure where to save the output file (must be PNG at the moment)
//! plot.config().terminal().output().update("./.tmp/output.png");
//! plot.config().terminal().font().update("Helvetica", 9);
//! plot.config().terminal().size().update(1200, 800);
//!
//! // update various fields
//! plot.x().label().update("X");
//! plot.y().label().update("Y");
//! plot.title().update("A basic plot");
//!
//! // we need to generate some data into a `Serie` struct. At the moment, only f64 falues are allowed.
//! let mut data_1 = Serie::with_title("Data series 1");
//! let mut data_2 = Serie::with_title("Data series 2");
//! for i in 1..=20 {
//!     data_1.add((i*i) as f64);
//!     data_2.add(i as f64);
//! }
//!
//! // data series are added individually at this time
//! plot.series().add(data_1);
//! plot.series().add(data_2);
//!
//! assert!(plot.render().is_ok());
//! ```

use gnuplotter::maybe::Maybe;
use gnuplotter::prelude::{Axis, Config, Label, Plot, Required, Series, Title, X, Y};

#[derive(Clone, PartialEq, Eq, Debug, Default, Axis)]
pub struct XAxis
{
    label: Required<Label<X>>
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Axis)]
pub struct YAxis
{
    label: Maybe<Label<Y>>
}

#[derive(Clone, PartialEq, Debug, Default, Plot)]
pub struct BasicPlot {
    config: Config,
    title: Maybe<Title>,
    x: XAxis,
    y: YAxis,
    series: Series<f64>,
}

impl BasicPlot {
    pub fn new() -> Self {
        BasicPlot::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_plot(){
        let mut plot = BasicPlot::new();
        plot.config().terminal().output().update("output.png");
        plot.x().label().update("X");
        plot.y().label().update("Y");
        plot.title().update("A basic plot");

        let result = plot.render();

        assert!(result.is_ok());

    }
}
