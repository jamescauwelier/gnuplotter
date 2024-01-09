#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Required<T> {
    Missing,
    Present(T)
}

impl<T> Default for Required<T> {
    fn default() -> Self {
        Required::Missing
    }
}

impl<T> Required<T> {
    pub fn new(value: T) -> Self {
        Required::Present(value)
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
}