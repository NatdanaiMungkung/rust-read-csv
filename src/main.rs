use csv;

use std::error::Error;

fn read_from_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;

    for (index, result) in reader.records().enumerate() {
        let record = result?;
        println!("{}{:?}", index, record);
    }
    Ok(())
}
fn main() {
    if let Err(e) = read_from_file("./test.csv") {
        eprintln!("{}", e);
    }
}
