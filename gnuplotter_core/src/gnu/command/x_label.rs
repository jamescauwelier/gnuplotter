use crate::gnu::command::{GnuCommand, GnuCommandFactory};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct XLabel(String);

impl XLabel {
    pub fn new(value: &str) -> Self {
        XLabel(value.into())
    }
}

impl GnuCommandFactory for XLabel {
    fn as_command(&self) -> GnuCommand {
        GnuCommand(format!("set xlabel \"{}\"", self.0))
    }
}

impl From<&str> for XLabel {
    fn from(value: &str) -> Self {
        XLabel(value.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::maybe::Maybe;
    use crate::prelude::Required;
    use super::*;

    #[test]
    fn test_an_x_label_can_be_created_from_a_string_slice_reference(){
        let x_label = XLabel::new("label");
        assert_eq!(x_label.0, "label");
    }

    #[test]
    fn test_a_maybe_x_label_can_be_created_from_a_string_slice_reference(){
        let maybe_x_label: Maybe<XLabel> = "label".into();
        assert_eq!(maybe_x_label, Maybe::value(XLabel::new("label")));
    }

    #[test]
    fn test_a_required_x_label_can_be_created_from_a_string_slice_reference(){
        let maybe_x_label: Required<XLabel> = "label".into();
        assert_eq!(maybe_x_label, Required::value(XLabel::new("label")));
    }
}