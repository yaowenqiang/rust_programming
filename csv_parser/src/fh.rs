use crate::types;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub fn read_visits_from_file(filename: &String) -> Vec<types::Visit> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut visits: Vec<types::Visit> = Vec::new();
    for line in reader.lines() {
        let visit = types::Visit::from_string(line.unwrap());
        visits.push(visit);
    }
    visits
}

pub fn write_visits_to_file(filename: &String, visits: &Vec<types::Visit>) {
    let file = File::create(filename).unwrap();
    let mut writer = BufWriter::new(file);
    for visit in visits {
        writer.write_all(visit.to_string().as_bytes()).unwrap();
    }
}
