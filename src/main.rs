use std::error::Error;
use std::fs::File;
use std::path::Path;

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    let file: File = File::open(filename)?;
    let mut rdr: csv::Reader<File> = csv::Reader::from_reader(file);

    if let Ok(header) = rdr.headers() {
        println!("{:?}", header);
    }

    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "sample.csv";
    read_csv(filename)
}