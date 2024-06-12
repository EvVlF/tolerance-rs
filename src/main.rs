use std::io::{self, Write};
use std::collections::HashMap;
use lazy_static::lazy_static;
use rust_decimal::Decimal;
use crate::accuracy::Accuracy;
use crate::size_range::SizeRange;
use crate::tolerance_accuracy::ToleranceAccuracy;
use crate::tolerance_field::{ToleranceField, TolHole, TolShaft};
use crate::tolerance_value::ToleranceValue;

mod tolerance_field;
mod accuracy;
mod size_range;
mod tolerance_accuracy;
mod tolerance_entity;
mod tolerance_value;

type SizeToleranceAccuracy = (SizeRange, ToleranceAccuracy);
type ToleranceTable = HashMap<SizeToleranceAccuracy, ToleranceValue>;

static TOLERANCE_TABLE: ToleranceTable = initialize_tolerance_table();

fn main() {
    println!("Hello, world!");
}

fn initialize_tolerance_table() -> ToleranceTable {
    let mut table: ToleranceTable = HashMap::new();

    table.insert(
        (
            SizeRange { min: 1, max: 3 },
            ToleranceAccuracy {
                tolerance: ToleranceField::Shaft(TolShaft::g),
                accuracy: Accuracy::Class5,
            },
        ),
        ToleranceValue { upper: Decimal::from(-2), lower: Decimal::from(-6) },
    );
    table
}

fn find_tolerance_entry(size: u16, field: ToleranceField, accuracy: u8) -> Option<ToleranceValue> {
    if let Some(accuracy) = Accuracy::match_accuracy(accuracy) {
        let key = ToleranceAccuracy {
            tolerance: field,
            accuracy,
        };

        for ((size_range, tolerance_accuracy), value) in TOLERANCE_TABLE.iter() {
            if size > size_range.min && size <= size_range.max && *tolerance_accuracy == key {
                return Some(value.copy());
            }
        }
    }
    None
}