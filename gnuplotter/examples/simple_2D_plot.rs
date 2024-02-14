use gnuplotter_core::maybe::Maybe;
use gnuplotter_core::prelude::{Config, Label, Required, Series, Title, X, Y};
use gnuplotter_macros::{Axis, Plot};

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
pub struct Plot2D {
    config: Config,
    title: Maybe<Title>,
    x: XAxis,
    y: YAxis,
    series: Series<f64>,
}

impl Plot2D {
    pub fn new() -> Self {
        Plot2D::default()
    }
}