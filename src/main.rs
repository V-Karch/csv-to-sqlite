use std::error::Error;
use std::fs::File;
use std::path::Path;

fn read_csv_headers<P: AsRef<Path>>(filename: P) -> Result<Vec<String>, Box<dyn Error>> {
    let file: File = File::open(filename)?;
    let mut rdr: csv::Reader<File> = csv::Reader::from_reader(file);

    let headers = rdr.headers()?.iter().map(|s| s.to_string()).collect();
    Ok(headers)
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "sample.csv";
    match read_csv_headers(filename) {
        Ok(headers) => println!("Headers: {:?}", headers),
        Err(e) => eprintln!("Error reading headers: {}", e),
    }
    Ok(())
}
