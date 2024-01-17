use std::marker::PhantomData;
use gnuplotter::prelude::*;
use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq, Debug, Default, Axis)]
pub struct XAxisWithRequiredLabel
{
    label: Required<Label<X>>
}

#[derive(Clone, PartialEq, Eq, Debug, Default, Axis)]
pub struct YAxisWithOptionalLabel
{
    label: Maybe<Label<Y>>
}

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
    fn test_axis_creation_with_optional_label(){
        let mut axis = YAxisWithOptionalLabel::default();
        let commands = axis.as_commands();

        assert_eq!(commands.len(), 0);

        axis.label("label y");
        let commands = axis.as_commands();

        assert_eq!(commands.len(), 1);
    }

    #[test]
    #[should_panic(expected = "A required value must be present before commands can be generated.")]
    fn test_axis_creation_with_required_label_requires_label(){
        let axis = XAxisWithRequiredLabel::default();
        let _commands = axis.as_commands();
    }

    #[test]
    fn test_axis_creation_with_required_label(){
        let mut axis = XAxisWithRequiredLabel::default();
        axis.label("label x");
        let commands = axis.as_commands();

        assert_eq!(commands.len(), 1);
    }

    #[test]
    #[should_panic(expected = "A required value must be present before commands can be generated.")]
    fn test_an_axis_requires_a_label(){
        let plot = Plot2D::default();
        let commands = plot.as_commands();

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

        let commands = plot.as_commands();

        assert_eq!(commands.len(), 3);
    }
}