extern crate alloc;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use std::error::Error;
use std::io::{self, Write};
use std::str::FromStr;
use std::sync::Arc;

use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Color, ContentArrangement, Table};
use lazy_static::lazy_static;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rayon::prelude::*;
use regex::Regex;
use rusqlite::Result;
use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy::MidpointAwayFromZero;
use rust_decimal_macros::dec;

use crate::tolerance_table::ToleranceTable;

mod tolerance_table;

lazy_static! {
    static ref POOL: Arc<Pool<SqliteConnectionManager>> = Arc::new(ToleranceTable::new().pool);
    static ref TABLES_HOLES: Vec<Cow<'static, str>> = vec![
        Cow::from("ISO_Hole_Limits_A_C"),
        Cow::from("ISO_Hole_Limits_CD_G"),
        Cow::from("ISO_Hole_Limits_H_JS"),
        Cow::from("ISO_Hole_Limits_J_P"),
        Cow::from("ISO_Hole_Limits_R_S"),
        Cow::from("ISO_Hole_Limits_T_X"),
        Cow::from("ISO_Hole_Limits_Z_ZC"),
    ];
    static ref TABLES_SHAFTS: Vec<Cow<'static, str>> = vec![
        Cow::from("ISO_Shaft_Limits_a_c"),
        Cow::from("ISO_Shaft_Limits_cd_g"),
        Cow::from("ISO_Shaft_Limits_h_js"),
        Cow::from("ISO_Shaft_Limits_j_p"),
        Cow::from("ISO_Shaft_Limits_r_s"),
        Cow::from("ISO_Shaft_Limits_t_u"),
        Cow::from("ISO_Shaft_Limits_v_zc"),
    ];
}

fn main() {
    loop {
        let size_field_accuracy = get_input_values();
        handle_search(
            &size_field_accuracy.0,
            &size_field_accuracy.1,
            &size_field_accuracy.2,
        )
    }
}

fn handle_search(size: &str, field: &str, accuracy: &str) {
    let upper_lower_tol = search_in_tables(&POOL, size, field, accuracy);
    if let Ok(Some((upper_tol, lower_tol))) = upper_lower_tol {
        let decimals = size_tols_map_decimal(size, (&upper_tol, &lower_tol));
        let average_tol = calc_average_tol(&decimals.1, &decimals.2);
        let calc_sizes = calc_sizes_with_tols(decimals, average_tol);
        print_result(
            size,
            field,
            accuracy,
            (&upper_tol, &lower_tol),
            &calc_sizes,
            &average_tol,
        )
    } else if let Ok(None) = upper_lower_tol {
        println!("Ничего не найдено для заданных параметров");
    } else if let Err(e) = upper_lower_tol {
        println!("Ошибка при поиске в БД: {}", e);
    }
}

fn search_in_tables(
    pool: &POOL,
    size: &str,
    field: &str,
    accuracy: &str,
) -> Result<Option<(String, String)>, Box<dyn Error + Send + Sync>> {
    let tables: &Vec<Cow<'static, str>> = if is_uppercase(field) {
        &TABLES_HOLES
    } else {
        &TABLES_SHAFTS
    };

    let field_accuracy = format!("{}{}", field, accuracy);

    let result = tables
        .par_iter()
        .map(|table_name| search_in_table(pool, size, &field_accuracy, table_name))
        .find_any(|res| matches!(res, Ok(Some(_))));
    result.unwrap_or(Ok(None))
}

fn search_in_table(
    pool: &POOL,
    size: &str,
    field_accuracy: &str,
    table_name: &str,
) -> Result<Option<(String, String)>, Box<dyn Error + Send + Sync>> {
    let query = format!(
        "SELECT {1} FROM {2} WHERE MIN_DIA <= {0} AND MAX_DIA >= {0}",
        size, field_accuracy, table_name
    );
    let connection = pool.get().expect("Не удалось получить пул соединения БД");
    let mut stmt = connection.prepare(&query)?;

    let mut rows = stmt.query_map([], |row| {
        let tol: String = row.get(0)?;
        Ok(tol)
    })?;
    let upper_tol = match rows.next() {
        Some(row) => row?,
        None => return Ok(None),
    };

    if upper_tol.trim().is_empty() {
        return Ok(None);
    }

    let lower_tol = match rows.next() {
        Some(row) => row?,
        None => return Ok(None),
    };

    if lower_tol.trim().is_empty() {
        return Ok(None);
    }

    Ok(Some((upper_tol, lower_tol)))
}

fn get_input_values() -> (String, String, String) {
    loop {
        print!("(Для справки введите -h или help) Введите данные: ");
        io::stdout().flush().expect("Ошибка обработки вызова flush");
        let binding = read_input();
        let input = binding.trim();
        if input.eq_ignore_ascii_case("-h") || input.eq_ignore_ascii_case("help") {
            print_help_info();
            continue;
        }
        if let Some((size, field, accuracy)) = parse_input(input) {
            return (size, field, accuracy);
        } else {
            println!("Некорректный ввод. Пожалуйста, попробуйте снова");
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка при чтении строки");
    input
}

fn parse_input(input: &str) -> Option<(String, String, String)> {
    let regex = Regex::new(r"^(?P<size>([0-9]|[1-9][0-9]{1,2}|[12][0-9]{3}|30[0-9]{2}|31[0-4][0-9]|3150)([.,]\d{1,3})?)(?P<field>[a-z]+|[A-Z]+)(?P<accuracy>[1-9]|1[0-8])$").expect("Ошибка обработки RegEx");
    if let Some(captures) = regex.captures(input) {
        let size = replace_comma_with_dot(&captures["size"]).to_string();
        let field = captures["field"].to_string();
        let accuracy = captures["accuracy"].to_string();
        Some((size, field, accuracy))
    } else {
        None
    }
}

fn replace_comma_with_dot(size: &str) -> String {
    if size.contains(',') {
        size.replace(',', ".").to_string()
    } else {
        size.to_string()
    }
}

fn is_uppercase(field: &str) -> bool {
    field.chars().all(|c| c.is_uppercase())
}

fn size_tols_map_decimal(size: &str, tols: (&str, &str)) -> (Decimal, Decimal, Decimal) {
    let size: Decimal =
        Decimal::from_str(size).expect("Ошибка преобразования значения из String в Decimal");
    let upper_tol: Decimal =
        Decimal::from_str(tols.0).expect("Ошибка преобразования значения из String в Decimal");
    let lower_tol: Decimal =
        Decimal::from_str(tols.1).expect("Ошибка преобразования значения из String в Decimal");
    (size, upper_tol, lower_tol)
}

fn calc_sizes_with_tols(
    size_tols: (Decimal, Decimal, Decimal),
    average_tol: Decimal,
) -> (String, String, String) {
    let size = size_tols.0;
    let upper_size = size_tols.1 + size;
    let lower_size = size_tols.2 + size;
    let average_size = average_tol + size;
    (
        average_size.normalize().to_string(),
        upper_size.normalize().to_string(),
        lower_size.normalize().to_string(),
    )
}

fn calc_average_tol(upper_tol: &Decimal, lower_tol: &Decimal) -> Decimal {
    Decimal::round_dp_with_strategy(
        &((upper_tol - lower_tol) * dec!(0.5) + lower_tol),
        6,
        MidpointAwayFromZero,
    )
        .normalize()
}

fn print_result(
    size: &str,
    field: &str,
    accuracy: &str,
    tols: (&str, &str),
    sizes: &(String, String, String),
    average_tol: &Decimal,
) {
    let mut table_result = Table::new();
    table_result
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .add_row(vec![&format!("{}{}{}", size, field, accuracy), "", ""])
        .add_row(vec![
            Cell::new(sizes.0.to_string()).fg(Color::Green),
            Cell::new(sizes.1.to_string()).fg(Color::Red),
            Cell::new(sizes.2.to_string()).fg(Color::Cyan),
        ])
        .add_row(vec![
            Cell::new(average_tol.to_string()).fg(Color::Blue),
            Cell::new(tols.0.to_string()).fg(Color::Magenta),
            Cell::new(tols.1.to_string()).fg(Color::Yellow),
        ]);
    println!("{table_result}")
}

fn print_help_info() {
    let mut help_info = Table::new();
    help_info
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .add_row(vec![
            Cell::new("средний размер").fg(Color::Green),
            Cell::new("верхний размер").fg(Color::Red),
            Cell::new("нижний размер").fg(Color::Cyan),
        ])
        .add_row(vec![
            Cell::new("средний допуск").fg(Color::Blue),
            Cell::new("верхний допуск").fg(Color::Magenta),
            Cell::new("нижний допуск").fg(Color::Yellow),
        ]);
    println!("{help_info}")
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_search_in_hole_tables() {
        let result = search_in_tables(&POOL, "0.5", "A", "9").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "1", "A", "9").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "1.001", "A", "9").unwrap();
        assert_eq!(result.unwrap(), ("0.295".to_string(), "0.27".to_string()));

        let result = search_in_tables(&POOL, "112", "B", "11").unwrap();
        assert_eq!(result.unwrap(), ("0.46".to_string(), "0.24".to_string()));

        let result = search_in_tables(&POOL, "120", "B", "11").unwrap();
        assert_eq!(result.unwrap(), ("0.46".to_string(), "0.24".to_string()));

        let result = search_in_tables(&POOL, "120.001", "B", "11").unwrap();
        assert_eq!(result.unwrap(), ("0.51".to_string(), "0.26".to_string()));

        let result = search_in_tables(&POOL, "475", "C", "13").unwrap();
        assert_eq!(result.unwrap(), ("1.45".to_string(), "0.48".to_string()));

        let result = search_in_tables(&POOL, "500", "C", "13").unwrap();
        assert_eq!(result.unwrap(), ("1.45".to_string(), "0.48".to_string()));

        let result = search_in_tables(&POOL, "500.001", "C", "13").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "0", "H", "1").unwrap();
        assert_eq!(result.unwrap(), ("0.0008".to_string(), "0".to_string()));

        let result = search_in_tables(&POOL, "0.5", "H", "1").unwrap();
        assert_eq!(result.unwrap(), ("0.0008".to_string(), "0".to_string()));

        let result = search_in_tables(&POOL, "1", "H", "1").unwrap();
        assert_eq!(result.unwrap(), ("0.0008".to_string(), "0".to_string()));

        let result = search_in_tables(&POOL, "1.001", "H", "1").unwrap();
        assert_eq!(result.unwrap(), ("0.0008".to_string(), "0".to_string()));

        let result = search_in_tables(&POOL, "0", "JS", "18").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "0.5", "JS", "18").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "1.001", "JS", "18").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "5", "JS", "3").unwrap();
        assert_eq!(
            result.unwrap(),
            ("0.00125".to_string(), "-0.00125".to_string())
        );

        let result = search_in_tables(&POOL, "2500", "JS", "18").unwrap();
        assert_eq!(result.unwrap(), ("14".to_string(), "-14".to_string()));

        let result = search_in_tables(&POOL, "3150", "JS", "18").unwrap();
        assert_eq!(result.unwrap(), ("16.5".to_string(), "-16.5".to_string()));
    }

    #[test]
    fn test_search_in_shaft_tables() {
        let result = search_in_tables(&POOL, "0.5", "a", "9").unwrap();
        assert_eq!(result.unwrap(), ("-0.27".to_string(), "-0.295".to_string()));

        let result = search_in_tables(&POOL, "1", "a", "9").unwrap();
        assert_eq!(result.unwrap(), ("-0.27".to_string(), "-0.295".to_string()));

        let result = search_in_tables(&POOL, "1.001", "a", "9").unwrap();
        assert_eq!(result.unwrap(), ("-0.27".to_string(), "-0.295".to_string()));

        let result = search_in_tables(&POOL, "112", "b", "11").unwrap();
        assert_eq!(result.unwrap(), ("-0.24".to_string(), "-0.46".to_string()));

        let result = search_in_tables(&POOL, "120", "b", "11").unwrap();
        assert_eq!(result.unwrap(), ("-0.24".to_string(), "-0.46".to_string()));

        let result = search_in_tables(&POOL, "120.001", "b", "11").unwrap();
        assert_eq!(result.unwrap(), ("-0.26".to_string(), "-0.51".to_string()));

        let result = search_in_tables(&POOL, "475", "c", "12").unwrap();
        assert_eq!(result.unwrap(), ("-0.48".to_string(), "-1.11".to_string()));

        let result = search_in_tables(&POOL, "500", "c", "12").unwrap();
        assert_eq!(result.unwrap(), ("-0.48".to_string(), "-1.11".to_string()));

        let result = search_in_tables(&POOL, "500.001", "c", "12").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "500.001", "c", "13").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "0", "h", "1").unwrap();
        assert_eq!(result.unwrap(), ("0".to_string(), "-0.0008".to_string()));

        let result = search_in_tables(&POOL, "0.5", "h", "1").unwrap();
        assert_eq!(result.unwrap(), ("0".to_string(), "-0.0008".to_string()));

        let result = search_in_tables(&POOL, "1", "h", "1").unwrap();
        assert_eq!(result.unwrap(), ("0".to_string(), "-0.0008".to_string()));

        let result = search_in_tables(&POOL, "1.001", "h", "1").unwrap();
        assert_eq!(result.unwrap(), ("0".to_string(), "-0.0008".to_string()));

        let result = search_in_tables(&POOL, "0", "js", "18").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "0.5", "js", "18").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "1.001", "js", "18").unwrap();
        assert!(result.is_none());

        let result = search_in_tables(&POOL, "5", "js", "3").unwrap();
        assert_eq!(
            result.unwrap(),
            ("0.00125".to_string(), "-0.00125".to_string())
        );

        let result = search_in_tables(&POOL, "2500", "js", "18").unwrap();
        assert_eq!(result.unwrap(), ("14".to_string(), "-14".to_string()));

        let result = search_in_tables(&POOL, "3150", "js", "18").unwrap();
        assert_eq!(result.unwrap(), ("16.5".to_string(), "-16.5".to_string()));
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("0H2").unwrap(),
            ("0".to_string(), "H".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("0.123h2").unwrap(),
            ("0.123".to_string(), "h".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("0,123h2").unwrap(),
            ("0.123".to_string(), "h".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("1h2").unwrap(),
            ("1".to_string(), "h".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("1H2").unwrap(),
            ("1".to_string(), "H".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("18k5").unwrap(),
            ("18".to_string(), "k".to_string(), "5".to_string())
        );
        assert_eq!(
            parse_input("315h9").unwrap(),
            ("315".to_string(), "h".to_string(), "9".to_string())
        );
        assert_eq!(
            parse_input("1.0H2").unwrap(),
            ("1.0".to_string(), "H".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("1,0H2").unwrap(),
            ("1.0".to_string(), "H".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("1.92h2").unwrap(),
            ("1.92".to_string(), "h".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("1,92h2").unwrap(),
            ("1.92".to_string(), "h".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("1.685h2").unwrap(),
            ("1.685".to_string(), "h".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("1,685h2").unwrap(),
            ("1.685".to_string(), "h".to_string(), "2".to_string())
        );
        assert_eq!(parse_input("1.6853K2"), None);
        assert_eq!(
            parse_input("1,685j12").unwrap(),
            ("1.685".to_string(), "j".to_string(), "12".to_string())
        );
        assert_eq!(
            parse_input("1,685j18").unwrap(),
            ("1.685".to_string(), "j".to_string(), "18".to_string())
        );
        assert_eq!(
            parse_input("1,685cd18").unwrap(),
            ("1.685".to_string(), "cd".to_string(), "18".to_string())
        );
        assert_eq!(
            parse_input("1,685CD18").unwrap(),
            ("1.685".to_string(), "CD".to_string(), "18".to_string())
        );
        assert_eq!(parse_input("1.685J19"), None);
        assert_eq!(parse_input("1,685j19"), None);
        assert_eq!(parse_input("1.685j123"), None);
        assert_eq!(parse_input("1,685j123"), None);

        assert_eq!(
            parse_input("3150h2").unwrap(),
            ("3150".to_string(), "h".to_string(), "2".to_string())
        );
        assert_eq!(
            parse_input("1h2").unwrap(),
            ("1".to_string(), "h".to_string(), "2".to_string())
        );
    }

    #[test]
    fn test_replace_comma_with_dot() {
        assert_eq!(replace_comma_with_dot("0,1"), "0.1");
        assert_eq!(replace_comma_with_dot("0.1"), "0.1");
        assert_eq!(replace_comma_with_dot(",1"), ".1");
        assert_eq!(replace_comma_with_dot("10,01"), "10.01");
        assert_eq!(replace_comma_with_dot("10.01"), "10.01")
    }

    #[test]
    fn test_is_uppercase() {
        assert!(is_uppercase("A"));
        assert!(is_uppercase("AC"));
        assert!(!is_uppercase("Ac"));
        assert!(!is_uppercase("aC"));
        assert!(!is_uppercase(" "));
        assert!(!is_uppercase("."));
    }

    #[test]
    fn test_calculate_size() {
        assert_eq!(
            calc_sizes_with_tols((dec!(1), dec!(0.2), dec!(0.1)), dec!(0.15)),
            ("1.15".to_string(), "1.2".to_string(), "1.1".to_string())
        );
        assert_eq!(
            calc_sizes_with_tols((dec!(1.234), dec!(0.2), dec!(0.1)), dec!(0.15)),
            (
                "1.384".to_string(),
                "1.434".to_string(),
                "1.334".to_string()
            )
        );
        assert_eq!(
            calc_sizes_with_tols((dec!(1.234), dec!(0.789), dec!(0.123)), dec!(0.456)),
            ("1.69".to_string(), "2.023".to_string(), "1.357".to_string())
        );
        assert_eq!(
            calc_sizes_with_tols((dec!(1), dec!(0.789), dec!(-0.123)), dec!(0.333)),
            (
                "1.333".to_string(),
                "1.789".to_string(),
                "0.877".to_string()
            )
        );
        assert_eq!(
            calc_sizes_with_tols((dec!(1.234), dec!(0.789), dec!(-0.123)), dec!(0.333)),
            (
                "1.567".to_string(),
                "2.023".to_string(),
                "1.111".to_string()
            )
        );
        assert_eq!(
            calc_sizes_with_tols((dec!(1), dec!(-0.123), dec!(-0.789)), dec!(-0.456)),
            (
                "0.544".to_string(),
                "0.877".to_string(),
                "0.211".to_string()
            )
        );
    }
}
