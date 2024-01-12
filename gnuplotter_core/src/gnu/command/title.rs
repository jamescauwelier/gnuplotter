use crate::gnu::command::*;
use crate::maybe::Maybe;
use crate::required::Required;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Title(String);

impl Title {
    pub fn new(value: &str) -> Self {
        Title(value.into())
    }
}

impl GnuCommandFactory for Title {
    fn as_commands(&self) -> VecDeque<GnuCommand> {
        vec![GnuCommand(format!("set title '{}'", self.0))].into()
    }
}

impl From<&str> for Title {
    fn from(value: &str) -> Self {
        Title(value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_title_can_be_created_from_a_string_slice_reference(){
        let title = Title::new("an experiment");
        assert_eq!(title.0, "an experiment");
    }

    #[test]
    fn test_a_maybe_title_can_be_created_from_a_string_slice_reference(){
        let maybe_title: Maybe<Title> = "an experiment".into();
        assert_eq!(maybe_title, Maybe::value(Title::new("an experiment")));
    }

    #[test]
    fn test_a_required_title_can_be_created_from_a_string_slice_reference(){
        let maybe_title: Required<Title> = "an experiment".into();
        assert_eq!(maybe_title, Required::value(Title::new("an experiment")));
    }
}