use crate::maybe::Maybe::Nothing;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Maybe<T> {
    Value(T),
    Nothing
}

impl<T> Default for Maybe<T> {
    fn default() -> Self {
        Nothing
    }
}

impl<T> Maybe<T> {
    pub fn value(value: T) -> Self {
        Maybe::Value(value)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maybe_default_is_nothing() {
        assert_eq!(Maybe::default(), Maybe::<i32>::Nothing);
    }

    #[test]
    fn test_a_maybe_can_be_constructed() {
        assert_eq!(Maybe::value(1), Maybe::Value(1));
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