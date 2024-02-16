//! # Basic example
//!
//! See [the basic_plot() function source](../src/demo_basic/demo_basic.rs.html#49-77) for a demo
//! on how to use this library after declaring the components of a plot.
//!
//! A basic plot may be configured by providing a struct detailing its components and deriving it
//! using the `Plot` macro:
//!
//! ```
//! #[derive(Clone, PartialEq, Debug, Default, Plot)]
//! pub struct BasicPlot {
//!     config: Config,
//!     title: Maybe<Title>,
//!     x: XAxis,
//!     y: YAxis,
//!     series: Series<f64>,
//! }
//! ```
//!
//! Axis can be similarly derived with an `Axis` macro:
//!
//! ```
//! #[derive(Clone, PartialEq, Eq, Debug, Default, Axis)]
//! pub struct XAxis
//! {
//!     /// declares a label on the X axis as required (without it, the gnuplot commands can't be
//!     /// accessed)
//!     label: Required<Label<X>>
//! }
//! ```
//!
//! Individual properties must implement the `GnuCommandFactory` trait.

use gnuplotter::maybe::Maybe;
use gnuplotter::prelude::{Axis, Config, Label, Plot, Required, Series, Title, X, Y};

/// Derives an axis using the `Axis` macro. This produces all necessary getters and gnuplot
/// command producing functionalities.
#[derive(Clone, PartialEq, Eq, Debug, Default, Axis)]
pub struct XAxis
{
    /// declares a label on the X axis as required (without it, the gnuplot commands can't be
    /// accessed)
    label: Required<Label<X>>
}

/// Derives an axis using the `Axis` macro. This produces all necessary getters and gnuplot
/// command producing functionalities.
#[derive(Clone, PartialEq, Eq, Debug, Default, Axis)]
pub struct YAxis
{
    /// Declares an optional label
    label: Maybe<Label<Y>>
}

/// Produces a plot by declaring its components, and then deriving both getters and rendering
/// functionality (usage demonstrated in basic_plot() function)
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

pub fn main() {
    basic_plot().unwrap();
}

/// Produces a basic plot by instantiating and configuring the structure, before adding data to it.
pub fn basic_plot() -> RenderResult<()> {

    // initialize the plot
    let mut plot = BasicPlot::new();

    // configure where to save the output file (must be PNG at the moment)
    plot.config().terminal().output().update("./.tmp/output.png");
    plot.config().terminal().font().update("Helvetica", 9);
    plot.config().terminal().size().update(1200, 800);

    // update various fields
    plot.x().label().update("X");
    plot.y().label().update("Y");
    plot.title().update("A basic plot");

    // we need to generate some data into a `Serie` struct. At the moment, only f64 falues are allowed.
    let mut data_1 = Serie::with_title("Data series 1");
    let mut data_2 = Serie::with_title("Data series 2");
    for i in 1..=20 {
        data_1.add((i*i) as f64);
        data_2.add(i as f64);
    }

    // data series are added individually at this time
    plot.series().add(data_1);
    plot.series().add(data_2);

    plot.render()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_plot(){
        assert!(basic_plot().is_ok());
    }
}
