use std::marker::PhantomData;
use gnuplotter::prelude::*;
use std::fmt::Debug;

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
    title: Maybe<Title>,
    x: XAxis,
    y: YAxis,
    series: Series<f64>
}

impl Plot2D {
    pub fn new() -> Self {
        Plot2D::default()
    }
}


#[cfg(test)]
mod tests {
    use gnuplotter::prelude::Required::Missing;
    use super::*;

    #[test]
    fn test_axis_creation_with_optional_label(){
        let mut axis = YAxis::default();
        let commands = axis.as_commands().unwrap();

        assert_eq!(commands.len(), 0);

        axis.label().update("label y".into());
        let commands = axis.as_commands().unwrap();

        assert_eq!(commands.len(), 1);
    }

    #[test]
    #[should_panic(expected = "A required value must be present before commands can be generated.")]
    fn test_axis_creation_with_required_label_requires_label(){
        let axis = XAxis::default();
        let _commands = axis.as_commands().unwrap();
    }

    #[test]
    fn test_axis_creation_with_required_label(){
        let mut axis = XAxis::default();
        axis.label().update("label x".into());
        let commands = axis.as_commands().unwrap();

        assert_eq!(commands.len(), 1);

        axis.label().update("label x 2nd".into());
        let mut commands = axis.as_commands().unwrap();
        let c: String = commands.pop_front().unwrap().into();
        assert_eq!(c, "set xlabel \"label x 2nd\"");
    }

    #[test]
    #[should_panic(expected = "A required value must be present before commands can be generated.")]
    fn test_an_axis_requires_a_label(){
        let plot = Plot2D::default();
        let commands = plot.as_commands().unwrap();

        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn test_plot_creation() {
        let mut plot = Plot2D::default();

        assert_eq!(plot.title(), &mut Maybe::Nothing);
        assert_eq!(plot.x(), &mut XAxis::default());
        assert_eq!(plot.y(), &mut YAxis::default());

        plot.title().update("an experiment".into());
        plot.x.label().update("label x".into());
        plot.y.label().update("label y".into());

        let commands = plot.as_commands().unwrap();

        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn test_plotting_linear_and_exponential_series() {
        let mut plot = Plot2D::default();
        plot.x.label().update("label x".into());

        let mut linear_series = Serie::<f64>::with_title(Some("Linear data".into()));
        let mut exponential_series = Serie::<f64>::new();
        for i in 0..10 {
            linear_series.add(i as f64);
            exponential_series.add((i*i) as f64);
        }
        plot.series.add(linear_series);
        plot.series.add(exponential_series);

        let mut commands = plot.as_commands().unwrap();
        let _first = commands.pop_front();
        let second = commands.pop_front().unwrap().to_string();

        assert_eq!(second, "plot \"./.tmp/series_data.txt\" using 1:2 title 'Linear data' with linespoint, \"./.tmp/series_data.txt\" using 1:3 with linespoint");
    }
}