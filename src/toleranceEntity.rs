use crate::accuracy::Accuracy;
use crate::toleranceField::ToleranceField;
use crate::toleranceValue::ToleranceValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ToleranceEntity {
    field: ToleranceField,
    grade: Accuracy,
    value: ToleranceValue,
}