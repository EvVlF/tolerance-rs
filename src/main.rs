use std::io::{self, Write};
use std::collections::HashMap;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;
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

lazy_static! {
static ref TOLERANCE_TABLE: ToleranceTable = initialize_tolerance_table();
    }

fn main() {
    loop {
        if let Some((size, field, accuracy)) = get_input() {
            if let Some(tolerance_value) = find_tolerance_entry(size, field, accuracy) {
                print_result(size, tolerance_value);
            } else {
                println!("Значение не найдено");
            }
        } else {
            println!("Неверный формат ввода. Пожалуйста, попробуйте снова.");
        }
    }
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

fn find_tolerance_entry(size: Decimal, field: ToleranceField, accuracy: u8) -> Option<ToleranceValue> {
    if let Some(accuracy) = Accuracy::match_accuracy(accuracy) {
        let key = ToleranceAccuracy {
            tolerance: field,
            accuracy,
        };

        for ((size_range, tolerance_accuracy), value) in TOLERANCE_TABLE.iter() {
            if size > size_range.min && size <= size_range.max && *tolerance_accuracy == key {
                return Some(value.clone());
            }
        }
    }
    None
}

fn get_input() -> Option<(Decimal, ToleranceField, u8)> {
    print!("Введите данные в формате \"<размер><поле допуска><класс точности>\": ");
    io::stdout().flush().unwrap(); // Обработка ошибок вызова flush()
    let binding = read_input();
    let input = binding.trim();
    parse_input(input)
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка при чтении строки");
    input
}

fn parse_input(input: &str) -> Option<(Decimal, ToleranceField, u8)> {
    let re = Regex::new(r"^(\d{1,3})([a-zA-Z]+)(\d{1,2})$").unwrap();
    if let Some(captures) = re.captures(input) {
        let size: Decimal = captures[1].parse().ok()?;
        let field_str = &captures[2];
        let accuracy: u8 = captures[3].parse().ok()?;
        let field = ToleranceField::from_str(field_str).ok()?;
        Some((size, field, accuracy))
    } else {
        None
    }
}

fn print_result(size: Decimal, tolerance_value: tolerance_value::ToleranceValue) {
    let upper_mm = tolerance_value.upper / Decimal::new(1000, 0);
    let lower_mm = tolerance_value.lower / Decimal::new(1000, 0);

    println!("верхнее отклонение:  {:>7} мм", upper_mm);
    println!("среднее отклонение:  {:>7} мм", (upper_mm + lower_mm) / Decimal::new(2, 0));
    println!("нижнее отклонение:   {:>7} мм", lower_mm);
    println!("максимальный размер: {:>7} мм", size + upper_mm);
    println!("средний размер:      {:>7} мм", size + (upper_mm + lower_mm) / Decimal::new(2, 0));
    println!("минимальный размер:  {:>7} мм", size + lower_mm);
}
