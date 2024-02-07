![Tests](https://github.com/jamescauwelier/gnuplotter/actions/workflows/build_and_test.yml/badge.svg)

# GNUPlotter

## Library status

This library is in alpha stage. It is not yet ready for production use. The main features are not yet implemented. It is 
not even possible to generate plots yet.

The priority is to explore the design goals and to implement:

- [ ] A declarative interface to GnuPlot (restricted to simple 2D line plots).
- [ ] A way to generate 2D line plots from the declarative interface, by sending commands to a terminal.
- [ ] A way to save plots on disk.

## Design goals

The main design goal of GNUPlotter is to access GnuPlot through a declarative interface. Such an interface does not
impose the shape and details of a graph on an end-user, but allows the user to specify what a graph can and cannot
contain. This is demonstrated by this example:

```rust
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
```

The user declares that an X axis always needs a label, but that the Y axis label may be absent. A title is optional
as well. And data is provided in a collection of Series containing `f64` values.

The axis and plot types are derived using the `Axis` and `Plot` macros. This provides an interface to interact with the
plot like so:

```rust 
#[test]
fn test_plotting_linear_and_exponential_series() {
    
    let mut plot = Plot2D::default();
    
    // only the x label is updated here
    plot.x.label().update("label x".into());

    // then some data is added to the series
    let mut linear_series = Serie::<f64>::with_title(Some("Linear data".into()));
    let mut exponential_series = Serie::<f64>::new();
    for i in 0..10 {
        linear_series.add(i as f64);
        exponential_series.add((i*i) as f64);
    }
    plot.series.add(linear_series);
    plot.series.add(exponential_series);

    // finally, gnuplot commands are generated (note that they are not yet sent out to gnuplot in this alpha version)
    let mut commands = plot.as_commands().unwrap();
    let _first = commands.pop_front();
    let second = commands.pop_front().unwrap().to_string();

    assert_eq!(second, "plot \"./.tmp/series_data.txt\" using 1:2 title 'Linear data' with linespoint, \"./.tmp/series_data.txt\" using 1:3 with linespoint");
}
```
