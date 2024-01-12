use gnuplotter::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug, Default, Plot)]
pub struct Plot2D {
    title: Maybe<Title>,
    x: Required<Axis<X>>,
    y: Required<Axis<Y>>,
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
    #[should_panic(expected = "A required value must be present before commands can be generated.")]
    fn test_an_axis_requires_a_label(){
        let mut plot = Plot2D::default();
        let mut commands = plot.as_commands();

        assert_eq!(commands.len(), 3);
    }

    #[test]
    fn test_plot_creation() {
        let mut plot = Plot2D::default();

        assert_eq!(plot.title, Maybe::Nothing);
        assert_eq!(plot.x, Required::Missing);
        assert_eq!(plot.y, Required::Missing);

        plot.title("an experiment");
        plot.x.label("label x");
        plot.y.label("label y");

        let mut commands = plot.as_commands();

        assert_eq!(commands.len(), 3);
    }
}