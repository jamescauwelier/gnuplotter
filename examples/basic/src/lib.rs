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
//! ## Declare axis elements
//!
//! Sometimes, a hierarchy of elements makes sense. Here, there is a hierarchy of two axes, X and Y.
//! To use the axes, we need to first define them
//!
//! ```
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
//! ## Instantiate, manipulate, and render your plot
//!
//! Now, the basic plot can be used:
//!
//! ```
//! use crate::basic::*;
//!
//! let mut plot = BasicPlot::new();
//! plot.config().terminal().output().update("output.png");
//! plot.x().label().update("X".into());
//! plot.y().label().update("Y".into());
//! plot.title().update("A basic plot".into());
//!
//! assert_eq!(0, plot.as_commands().unwrap().len());
//! ```
//!

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
        plot.x().label().update("X".into());
        plot.y().label().update("Y".into());
        plot.title().update("A basic plot".into());

        let command_count = plot.as_commands().unwrap().len();

        todo!("Add data to the plot");
        todo!("Assert everything is working ok");

    }
}
