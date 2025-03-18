use csv::ReaderBuilder;
use chrono::NaiveDate;
use std::error::Error;
use std::collections::HashMap;

fn guess_column_types(file_path: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let headers = rdr.headers()?.clone();  // Get headers (column names)
    let mut column_types: HashMap<usize, String> = HashMap::new();

    // Read the first few rows to guess the types (e.g., 5 rows)
    for result in rdr.records().take(5) {
        let record = result?;
        
        // Iterate over all columns in the current row
        for (index, value) in record.iter().enumerate() {
            let type_guess = if value == "true" || value == "false" {
                "boolean".to_string()
            } else if value.parse::<i64>().is_ok() {
                "integer".to_string()
            } else if value.parse::<f64>().is_ok() {
                "float".to_string()
            } else if NaiveDate::parse_from_str(value, "%Y-%m-%d").is_ok() {
                "date".to_string()
            } else {
                "string".to_string()
            };

            // Store the guessed type for the column, prioritizing the first guess
            column_types.entry(index).or_insert(type_guess);
        }
    }

    // Collect column names and their guessed types in order
    let mut column_info: Vec<(String, String)> = Vec::new();
    for (index, column_type) in column_types {
        let column_name = headers.get(index).unwrap_or(&"Unknown".to_string()).to_string();
        column_info.push((column_name, column_type));
    }

    Ok(column_info)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "sample.csv";
    let column_types = guess_column_types(file_path)?;

    // Output the column name and its guessed type in order
    for (column_name, column_type) in column_types {
        println!("Column '{}' has type: {}", column_name, column_type);
    }

    Ok(())
}
