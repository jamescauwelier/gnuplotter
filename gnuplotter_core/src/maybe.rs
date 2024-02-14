use crate::maybe::Maybe::Nothing;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Maybe<T> {
    Value(T),
    Nothing
}

impl<T> Maybe<T> {
    pub fn value(value: T) -> Self {
        Maybe::Value(value)
    }

    pub fn update<S>(&mut self, value: S) where S: Into<T> {
        std::mem::swap(self, &mut Maybe::Value(value.into()));
    }

    pub fn unwrap(self) -> T {
        match self {
            Maybe::Value(value) => value,
            Maybe::Nothing => panic!("Tried to unwrap a Maybe::Nothing value.")
        }
    }
}

pub mod maybe_conversions {
    use crate::gnu::command::title::Title;
    use crate::maybe::Maybe;

    impl<'a,T> From<&'a str> for Maybe<T>
    where
        T: From<&'a str>
    {
        fn from(value: &'a str) -> Self {
            Maybe::Value(T::from(value))
        }
    }
}

impl<T> Default for Maybe<T> {
    fn default() -> Self {
        Maybe::Nothing
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Required;
    use super::*;

    #[test]
    fn test_a_maybe_can_be_constructed() {
        assert_eq!(Maybe::value(1), Maybe::Value(1));
    }

    #[test]
    fn test_an_empty_maybe_can_be_updated() {
        let mut maybe: Maybe<i32> = Maybe::Nothing;
        maybe.update(123);
        assert_eq!(maybe, Maybe::Value(123));
    }

    #[test]
    fn test_a_maybe_can_be_updated(){
        let mut maybe: Maybe<i32> = Maybe::value(123);
        maybe.update(456);
        assert_eq!(maybe, Maybe::Value(456));
    }

    #[test]
    fn test_updating_convertable_value(){
        let mut maybe = Maybe::<String>::Nothing;
        maybe.update("something that can be converted");

        assert_eq!(maybe, Maybe::Value(String::from("something that can be converted")));
    }

    mod conversions {
        use super::super::*;

        #[test]
        fn test_an_str_can_be_converted_into_a_maybe_string() {
            let s = "Hello, world!";
            let maybe_s: Maybe<String> = s.into();
            assert_eq!(maybe_s, Maybe::Value(String::from(s)));
        }
    }
}