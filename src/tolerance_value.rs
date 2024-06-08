#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct ToleranceValue {
    pub(crate) upper: i32,
    pub(crate) lower: i32,
}