use std::collections::VecDeque;
use std::marker::PhantomData;
use crate::prelude::*;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Label<D>
where
    D: Dimension
{
    text: String,
    dimension: PhantomData<D>
}

impl<D> Label<D>
where
    D: Dimension
{
    pub(in super) fn new(text: &str) -> Self {
        Label {
            text: text.into(),
            dimension: PhantomData
        }
    }

    pub(in super) fn from(text: &str) -> Self {
        Label::new(text)
    }
}

impl<D> From<Label<D>> for Maybe<Label<D>>
where
    D: Dimension
{
    fn from(label: Label<D>) -> Self {
        Maybe::Value(label)
    }
}

impl<D> From<Label<D>> for Required<Label<D>>
    where
        D: Dimension
{
    fn from(label: Label<D>) -> Self {
        Required::Value(label)
    }
}

impl GnuCommandFactory for Label<X> {
    fn as_commands(&self) -> VecDeque<GnuCommand> {
        vec![
            GnuCommand::new(format!("set xlabel \"{}\"", self.text))
        ].into()
    }
}

impl GnuCommandFactory for Label<Y> {
    fn as_commands(&self) -> VecDeque<GnuCommand> {
        vec![
            GnuCommand::new(format!("set ylabel \"{}\"", self.text))
        ].into()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use super::*;


    #[test]
    fn test_an_x_label_can_be_created_from_a_string_slice_reference(){
        let label: Label<X> = Label::new("x");
        assert_eq!(label.text, "x");
    }

    #[test]
    fn test_a_y_label_can_be_created_from_a_string_slice_reference(){
        let label: Label<Y> = Label::new("y");
        assert_eq!(label.text, "y");
    }

    #[test]
    fn test_a_maybe_x_label_can_be_created_from_a_string_slice_reference(){
        let maybe_x_label: Maybe<Label<X>> = Label::from("label").into();
        assert_eq!(maybe_x_label, Maybe::value(Label::<X>::new("label")));
    }

    #[test]
    fn test_a_required_x_label_can_be_created_from_a_string_slice_reference(){
        let maybe_x_label: Required<Label<X>> = Label::from("label").into();
        assert_eq!(maybe_x_label, Required::value(Label::<X>::new("label")));
    }

    #[test]
    fn test_a_label_has_label_information() {
        let label: Label<X> = Label::new("x");
        let mut commands = label.as_commands();

        assert_eq!(commands.len(), 1);
        assert_eq!(commands.pop_front().unwrap().0, "set xlabel \"x\"");
    }

    #[test]
    fn test_a_maybe_label_has_label_information() {
        let label: Maybe<Label<X>> = Maybe::value(Label::from("x"));
        let mut commands = label.as_commands();

        assert_eq!(commands.len(), 1);
        assert_eq!(commands.pop_front().unwrap().0, "set xlabel \"x\"");
    }

    #[test]
    fn test_an_empty_maybe_label_has_no_label_information() {
        let label: Maybe<Label<X>> = Maybe::Nothing;
        let mut commands = label.as_commands();

        assert_eq!(commands.len(), 0);
    }

    #[test]
    fn test_a_required_label_has_label_information() {
        let label: Required<Label<Y>> = Required::value(Label::from("y"));
        let mut commands = label.as_commands();

        assert_eq!(commands.len(), 1);
        assert_eq!(commands.pop_front().unwrap().0, "set ylabel \"y\"");
    }

    #[test]
    #[should_panic(expected = "A required value must be present before commands can be generated.")]
    fn test_a_missing_required_label_has_no_label_information() {
        let label: Required<Label<X>> = Required::Missing;
        let commands = label.as_commands();

        assert_eq!(commands.len(), 0);
    }
}