extern crate alloc;

use crate::tolerance_field::{ToleranceField};
use alloc::boxed::Box;
use alloc::string::String;
use regex::Regex;
use rusqlite::{Result};
use rust_decimal::Decimal;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::sync::{Arc};
use lazy_static::lazy_static;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rayon::prelude::*;
use crate::tolerance_table::{ToleranceTable};

mod accuracy;
mod size_range;
mod tolerance_accuracy;
mod tolerance_entity;
mod tolerance_field;
mod tolerance_value;
mod tolerance_table;

lazy_static! {
    static ref POOL: Arc<Pool<SqliteConnectionManager>> = Arc::new(ToleranceTable::new().pool);
}

fn main() {
    search_in_table(&POOL, "H6", "ISO_Hole_Limits_H_JS", 90).expect("ooops");
    // loop {
    //     if let Some((size, field, accuracy)) = get_input() {
    //         if let Some(tolerance_value) = find_tolerance_entry(size, field, accuracy) {
    //             print_result(size, tolerance_value);
    //         } else {
    //             println!("Значение не найдено");
    //         }
    //     } else {
    //         println!("Неверный формат ввода. Пожалуйста, попробуйте снова.");
    //     }
    // }
}

// fn search_in_tables(pool: &Arc<Pool<SqliteConnectionManager>>, search_value: String) -> Result<(), Box<dyn Error>> {
//     let search_table_ids = vec!["ISO_Hole_Limits_A_C", "ISO_Hole_Limits_CD_G", "ISO_Hole_Limits_H_JS", "ISO_Hole_Limits_J_P", "ISO_Hole_Limits_R_S", "ISO_Hole_Limits_T_X", "ISO_Hole_Limits_Z_ZC", "ISO_Shaft_Limits_a_c", "ISO_Shaft_Limits_cd_g", "ISO_Shaft_Limits_h_js", "ISO_Shaft_Limits_j_p", "ISO_Shaft_Limits_r_s", "ISO_Shaft_Limits_t_u", "ISO_Shaft_Limits_v_zc", "AI_Shaft_Limits_a_js"];
//
// }

fn search_in_table(pool: &Arc<Pool<SqliteConnectionManager>>, tol_field_accuracy: &str, table_name: &str, dia: u16) -> Result<(), Box<dyn Error>> {
    let connection = pool.get().expect("Не удалось получить пул соединения БД");
    let query = format!("SELECT {} FROM {} WHERE MIN_DIA <= {2} AND MAX_DIA > {2}", tol_field_accuracy, table_name, dia);
    let mut stmt = connection.prepare(&query)?;

    let mut rows = stmt.query_map([], |row| {
        let tol: String = row.get(0)?;

        Ok(tol)
    })?;
    let min_tol = rows.next().unwrap()?;
    let max_tol = rows.next().unwrap()?;
    println!("Min Tol: {}, \nMax Tol: {}", min_tol, max_tol);

    Ok(())
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
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка при чтении строки");
    input
}

fn parse_input(input: &str) -> Option<(Decimal, ToleranceField, u8)> {
    let regex = Regex::new(r"^(\d{1,3})([a-zA-Z]+)(\d{1,2})$").unwrap();
    if let Some(captures) = regex.captures(input) {
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
    println!(
        "среднее отклонение:  {:>7} мм",
        (upper_mm + lower_mm) / Decimal::new(2, 0)
    );
    println!("нижнее отклонение:   {:>7} мм", lower_mm);
    println!("максимальный размер: {:>7} мм", size + upper_mm);
    println!(
        "средний размер:      {:>7} мм",
        size + (upper_mm + lower_mm) / Decimal::new(2, 0)
    );
    println!("минимальный размер:  {:>7} мм", size + lower_mm);
}
