use crate::accuracy::Accuracy;
use crate::toleranceField::ToleranceField;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ToleranceEntity {
    field: ToleranceField,
    grade: Accuracy,
}