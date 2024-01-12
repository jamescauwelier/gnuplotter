#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Required<T> {
    Missing,
    Value(T)
}

impl<T> Default for Required<T> {
    fn default() -> Self {
        Required::Missing
    }
}

impl<T> Required<T> {
    pub fn value(value: T) -> Self {
        Required::Value(value)
    }

    pub fn unwrap(self) -> T {
        match self {
            Required::Missing => panic!("Required value is missing."),
            Required::Value(value) => value
        }
    }

    pub fn get_mut_ref(&mut self) -> &mut T {
        match self {
            Required::Missing => panic!("Required value is missing."),
            Required::Value(value) => value
        }
    }
}

pub mod required_conversions {
    use crate::gnu::command::title::Title;
    use super::*;

    impl<'a, T> From<&'a str> for Required<T>
    where
        T: From<&'a str>
    {
        fn from(value: &'a str) -> Self {
            Required::Value(T::from(value))
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_required_is_default_missing() {
        use super::*;

        let required: Required<i32> = Required::default();
        assert_eq!(required, Required::Missing);
    }

    mod conversions {
        use super::super::*;

        #[test]
        fn test_an_str_can_be_converted_into_a_required_string() {
            let s = "Hello, world!";
            let required_s: Required<String> = s.into();
            assert_eq!(required_s, Required::Value(String::from(s)));
        }
    }
}