use crate::accuracy::Accuracy;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ToleranceClass {
    field: ToleranceField,
    grade: Accuracy,
}