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
