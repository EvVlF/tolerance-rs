use crate::accuracy::Accuracy;
use crate::tolerance_field::ToleranceField;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ToleranceAccuracy {
    tolerance: ToleranceField,
    accuracy: Accuracy,
}