use crate::gnu::command::{GnuCommand, GnuCommandFactory};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct YLabel(String);

impl YLabel {
    pub fn new(value: &str) -> Self {
        YLabel(value.into())
    }
}

impl GnuCommandFactory for YLabel {
    fn as_command(&self) -> GnuCommand {
        GnuCommand(format!("set ylabel \"{}\"", self.0))
    }
}

impl From<&str> for YLabel {
    fn from(value: &str) -> Self {
        YLabel(value.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::maybe::Maybe;
    use crate::prelude::Required;
    use super::*;

    #[test]
    fn test_a_y_label_can_be_created_from_a_string_slice_reference(){
        let y_label = YLabel::new("label");
        assert_eq!(y_label.0, "label");
    }

    #[test]
    fn test_a_maybe_y_label_can_be_created_from_a_string_slice_reference(){
        let maybe_y_label: Maybe<YLabel> = "label".into();
        assert_eq!(maybe_y_label, Maybe::value(YLabel::new("label")));
    }

    #[test]
    fn test_a_required_y_label_can_be_created_from_a_string_slice_reference(){
        let maybe_y_label: Required<YLabel> = "label".into();
        assert_eq!(maybe_y_label, Required::value(YLabel::new("label")));
    }
}