use chrono::NaiveDate;
use csv::ReaderBuilder;
use rusqlite::Connection;
use std::collections::HashMap;
use std::error::Error;

/// Guess column types from the first few rows of the CSV.
fn guess_column_types(file_path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;
    let headers = rdr.headers()?.clone();
    let mut column_types: HashMap<usize, String> = HashMap::new();

    for result in rdr.records().take(5) {
        let record = result?;

        for (index, value) in record.iter().enumerate() {
            let type_guess = if value == "true" || value == "false" {
                "BOOLEAN"
            } else if value.parse::<i64>().is_ok() {
                "INTEGER"
            } else if value.parse::<f64>().is_ok() {
                "REAL"
            } else if NaiveDate::parse_from_str(value, "%Y-%m-%d").is_ok() {
                "DATE"
            } else {
                "TEXT"
            };
            column_types.entry(index).or_insert(type_guess.to_string());
        }
    }

    let column_info = headers
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let col_type = column_types.get(&i).cloned().unwrap_or("TEXT".to_string());
            (name.to_string(), col_type)
        })
        .collect();

    Ok(column_info)
}

/// Create a table and insert CSV contents into an SQLite database.
fn csv_to_sqlite(csv_path: &str, db_path: &str, table_name: &str) -> Result<(), Box<dyn Error>> {
    let column_info = guess_column_types(csv_path)?;
    let conn = Connection::open(db_path)?;

    // Build CREATE TABLE statement
    let columns_sql: Vec<String> = column_info
        .iter()
        .map(|(name, typ)| format!("\"{}\" {}", name, typ))
        .collect();
    let create_stmt = format!(
        "CREATE TABLE IF NOT EXISTS \"{}\" ({})",
        table_name,
        columns_sql.join(", ")
    );
    conn.execute(&create_stmt, [])?;

    // Read CSV again for insertion
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(csv_path)?;
    let headers = rdr.headers()?.clone();
    let placeholders = vec!["?"; headers.len()].join(", ");
    let insert_stmt = format!(
        "INSERT INTO \"{}\" ({}) VALUES ({})",
        table_name,
        headers
            .iter()
            .map(|h| format!("\"{}\"", h))
            .collect::<Vec<_>>()
            .join(", "),
        placeholders
    );
    let mut stmt = conn.prepare(&insert_stmt)?;

    for result in rdr.records() {
        let record = result?;
        let values: Vec<rusqlite::types::Value> = record
            .iter()
            .enumerate()
            .map(|(i, val)| match column_info[i].1.as_str() {
                "INTEGER" => val
                    .parse::<i64>()
                    .map_or(rusqlite::types::Value::Null, rusqlite::types::Value::from),
                "REAL" => val
                    .parse::<f64>()
                    .map_or(rusqlite::types::Value::Null, rusqlite::types::Value::from),
                "BOOLEAN" => val
                    .parse::<bool>()
                    .map_or(rusqlite::types::Value::Null, rusqlite::types::Value::from),
                "DATE" => NaiveDate::parse_from_str(val, "%Y-%m-%d")
                    .map_or(rusqlite::types::Value::Null, |d| {
                        rusqlite::types::Value::from(d.to_string())
                    }),
                _ => rusqlite::types::Value::from(val.to_string()),
            })
            .collect();

        stmt.execute(rusqlite::params_from_iter(values))?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv-file>", args[0]);
        std::process::exit(1);
    }

    let csv_path = &args[1];
    let db_path = if let Some(stripped) = csv_path.strip_suffix(".csv") {
        format!("{}.db", stripped)
    } else {
        format!("{}.db", csv_path)
    };
    let table_name = "table";

    csv_to_sqlite(csv_path, &db_path, table_name)?;
    println!(
        "CSV data inserted into '{}' table in '{}'",
        table_name, db_path
    );
    Ok(())
}
