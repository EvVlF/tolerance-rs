use std::collections::HashMap;
use lazy_static::lazy_static;
use rust_decimal::Decimal;
use crate::accuracy::Accuracy;
use crate::size_range::SizeRange;
use crate::tolerance_accuracy::ToleranceAccuracy;
use crate::tolerance_field::{ToleranceField, TolShaft};
use crate::tolerance_value::ToleranceValue;

type SizeToleranceAccuracy = (SizeRange, ToleranceAccuracy);
type ToleranceTable = HashMap<SizeToleranceAccuracy, ToleranceValue>;

lazy_static! {
pub static ref TOLERANCE_TABLE: ToleranceTable = initialize_tolerance_table();
    }

fn initialize_tolerance_table() -> ToleranceTable {
    let mut table: ToleranceTable = HashMap::new();

    table.insert(
        (
            SizeRange { min: Decimal::from(1), max: Decimal::from(3) },
            ToleranceAccuracy {
                tolerance: ToleranceField::Shaft(TolShaft::g),
                accuracy: Accuracy::Class5,
            },
        ),
        ToleranceValue { upper: Decimal::from(-2), lower: Decimal::from(-6) },
    );
    table
}
