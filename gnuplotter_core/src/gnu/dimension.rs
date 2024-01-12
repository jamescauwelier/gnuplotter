pub trait Dimension: Default {}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct X;
impl Dimension for X {}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Y;
impl Dimension for Y {}
