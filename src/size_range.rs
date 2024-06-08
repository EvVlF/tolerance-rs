#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SizeRange {
    pub(crate) min: u16,
    pub(crate) max: u16,
}