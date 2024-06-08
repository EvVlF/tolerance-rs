use rust_decimal::Decimal;

#[derive(Debug, Copy, Eq, Hash, PartialEq)]
pub struct ToleranceValue {
    pub(crate) upper: Decimal,
    pub(crate) lower: Decimal,
}