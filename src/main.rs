use std::fs::{self, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use csv::Writer;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Record {
    id: u32,
    name: String,
    value: String,
}

const JSON_FILE: &str = "data.json";
const CSV_FILE: &str = "data.csv";

fn main() {
    println!("Rust Data Storage System");

    let new_record = Record {
        id: 1,
        name: "Sample Data".to_string(),
        value: "42".to_string(),
    };

    println!("Saving record...");
    save_record_json(&new_record);
    save_record_csv(&new_record);

    println!("Retrieving records...");
    load_records_json();
    load_records_csv();
}

fn save_record_json(record: &Record) {
    let path = Path::new(JSON_FILE);
    let mut records = load_records_json_internal().unwrap_or_else(|_| vec![]);
    records.push(record.clone());

    let file = OpenOptions::new().write(true).create(true).truncate(true).open(path).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &records).unwrap();

    println!("JSON record saved.");
}

fn save_record_csv(record: &Record) {
    let path = Path::new(CSV_FILE);
    let file_exists = path.exists();

    let mut writer = Writer::from_writer(OpenOptions::new().append(true).create(true).open(path).unwrap());

    if !file_exists {
        writer.write_record(&["id", "name", "value"]).unwrap();
    }
    
    writer.serialize(record).unwrap();
    writer.flush().unwrap();

    println!("CSV record saved.");
}

fn load_records_json() {
    match load_records_json_internal() {
        Ok(records) => {
            if records.is_empty() {
                println!("No JSON records found.");
            } else {
                println!("JSON Records: {:?}", records);
            }
        }
        Err(_) => println!("Error loading JSON records."),
    }
}

fn load_records_json_internal() -> Result<Vec<Record>, serde_json::Error> {
    let path = Path::new(JSON_FILE);
    if path.exists() {
        let file = fs::File::open(path).unwrap();
        let reader = BufReader::new(file);
        let records: Vec<Record> = serde_json::from_reader(reader)?;
        Ok(records)
    } else {
        Ok(vec![])
    }
}

fn load_records_csv() {
    let path = Path::new(CSV_FILE);
    if path.exists() {
        let mut reader = csv::Reader::from_path(path).unwrap();
        for result in reader.deserialize::<Record>() {
            match result {
                Ok(record) => println!("CSV Record: {:?}", record),
                Err(e) => println!("Error reading CSV: {}", e),
            }
        }
    } else {
        println!("No CSV records found.");
    }
}

