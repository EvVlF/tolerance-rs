#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SizeRange {
    pub(crate) min: u32,
    pub(crate) max: u32,
}