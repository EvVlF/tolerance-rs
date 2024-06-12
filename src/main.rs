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
        print!("Введите данные в формате \"<размер><поле допуска><класс точности>\": ");
        io::stdout().flush().unwrap(); // Обработка ошибок вызова flush()
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка при чтении строки");

        // Удаляем пробелы в начале и конце строки
        let input = input.trim();

        // Регулярное выражение для анализа ввода
        let re = Regex::new(r"^(\d{1,3})([a-zA-Z]+)(\d{1,2})$").unwrap();

        if let Some(captures) = re.captures(input) {
            // Извлечение значений размера, поля допуска и класса точности из найденных совпадений
            let size: u16 = captures[1].parse().unwrap();
            let field_str = &captures[2];
            let accuracy: u8 = captures[3].parse().unwrap();

            let field = ToleranceField::from_str(field_str).expect("Ошибка при преобразовании поля допуска");

            if let Some(tolerance_value) = find_tolerance_entry(size, field, accuracy) {
                println!("Найденное значение: {:?}", tolerance_value);
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
                return Some(value.clone());
            }
        }
    }
    None
}