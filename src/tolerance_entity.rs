use crate::accuracy::Accuracy;
use crate::tolerance_field::ToleranceField;
use crate::tolerance_value::ToleranceValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ToleranceEntity {
    field: ToleranceField,
    grade: Accuracy,
    value: ToleranceValue,
}