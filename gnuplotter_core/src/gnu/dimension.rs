use crate::prelude::*;

pub trait Dimension: Default {
    fn name() -> &'static str;
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct X {}
impl Dimension for X {
    fn name() -> &'static str {
        "x"
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Y {}
impl Dimension for Y {
    fn name() -> &'static str {
        "y"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_for_required_dimension() {
        let required: Required<X> = Required::default();
        assert_eq!(required, Required::Missing);
    }

    #[test]
    fn test_default_for_maybe_dimension() {
        let maybe: Maybe<Y> = Maybe::default();
        assert_eq!(maybe, Maybe::Nothing);
    }
}