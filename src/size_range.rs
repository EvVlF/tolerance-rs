use rust_decimal::Decimal;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SizeRange {
    pub(crate) min: Decimal,
    pub(crate) max: Decimal,
}