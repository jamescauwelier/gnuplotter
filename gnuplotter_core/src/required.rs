use crate::prelude::{Dimension, Label};

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Required<T> {
    Missing,
    Value(T)
}

impl<T> Required<T> {
    pub fn missing() -> Self {
        Required::Missing
    }

    pub fn value(value: T) -> Self {
        Required::Value(value)
    }

    pub fn update<S>(&mut self, value: S) where S: Into<T> {
        std::mem::swap(self, &mut Required::value(value.into()));
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
            Required::value(T::from(value))
        }
    }
}

impl<T> Default for Required<T> {
    fn default() -> Self {
        Required::missing()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_is_default_missing() {
        let required: Required<i32> = Required::missing();
        assert_eq!(required, Required::Missing);
    }

    #[test]
    fn test_updating_values(){
        let mut required: Required<i32> = Required::missing();
        required.update(123);
        assert_eq!(required, Required::Value(123));

        let mut required: Required<i32> = Required::value(123);
        required.update(456);
        assert_eq!(required, Required::Value(456));
    }

    #[test]
    fn test_updating_convertable_value(){
        let mut required = Required::<String>::missing();
        required.update("something that can be converted");

        assert_eq!(required, Required::Value(String::from("something that can be converted")));
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