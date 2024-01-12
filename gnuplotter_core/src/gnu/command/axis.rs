use std::collections::VecDeque;
use either::Either;
use crate::gnu::dimension::Dimension;
use crate::prelude::*;
use crate::prelude::label::Label;

pub mod label;

pub trait AxisTrait<D>
where
    D: Dimension
{
    fn label(&mut self, text: &str);
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Axis<D>
where
    D: Dimension,
    Label<D>: GnuCommandFactory
{
    dimension: Required<D>,
    label: Required<Label<D>>,
}

impl<D> Axis<D>
where
    D: Dimension,
    Label<D>: GnuCommandFactory
{
    pub fn new() -> Self {
        Axis {
            dimension: Required::value(D::default()),
            label: Required::Missing,
        }
    }
}

impl<D> AxisTrait<D> for Axis<D>
where
    D: Dimension,
    Label<D>: GnuCommandFactory
{
    fn label(&mut self, text: &str) {
        self.label = Required::value(Label::new(text));
    }
}

impl<D> AxisTrait<D> for Required<Axis<D>>
    where
        D: Dimension,
        Label<D>: GnuCommandFactory
{
    fn label(&mut self, text: &str) {
        match self {
            Required::Missing => {
                *self = Required::value(Axis::default());
                self.label(text);
            },
            Required::Value(value) => value.label(text)
        }
    }
}

impl<D> GnuCommandFactory for Axis<D>
where
    D: Dimension,
    Label<D>: GnuCommandFactory
{
    fn as_commands(&self) -> VecDeque<GnuCommand> {
        let mut commands = VecDeque::new();
        commands.append(&mut self.label.as_commands());
        commands
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_an_x_axis_can_be_created(){
        let axis: Axis<X> = Axis::new();
        assert_eq!(axis.dimension, Required::value(X::default()));
    }

    #[test]
    fn test_a_y_axis_can_be_created(){
        let axis: Axis<Y> = Axis::new();
        assert_eq!(axis.dimension, Required::value(Y::default()));
    }

    #[test]
    fn test_an_axis_can_be_converted_to_commands() {
        let mut axis: Axis<X> = Axis::new();
        axis.label("x");
        let mut commands = axis.as_commands();

        assert_eq!(commands.len(), 1);
        assert_eq!(commands.pop_front().unwrap().0, "set xlabel \"x\"");
    }
}