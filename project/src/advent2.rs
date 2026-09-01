use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file = File::open("input2").expect("Failed to Read File.");
    let reader = io::BufReader::new(file);

    find_invalids(reader);
}

fn find_invalids(reader: BufReader<File>) -> i32 {
    let mut total: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("failed to read line.");
        let ids: Vec<&str> = line.split(",").collect();
        total += filter_ids(ids);
    }
    println!("{total}");
    total
}

fn filter_ids(ids: Vec<&str>) -> i32 {
    let mut total: i32 = 0;
    for id in ids {
        total += parse_id(id)
    }
    total
}

fn parse_id(id: &str) -> i32 {
    let mut invalid_id:i32 = 0;
    let bytes = id.as_bytes();

    for index in 1..=bytes.len() / 2 {
        if bytes.len() % index != 0 {
            continue;
        }
        let pattern = &bytes[..index];

        if bytes.chunks(index).all(|chunk| chunk == pattern) {
            invalid_id = id.parse::<i32>().expect("Invalid number");
        }
    }
    invalid_id
}