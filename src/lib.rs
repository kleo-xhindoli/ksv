use rand::prelude::IteratorRandom;
use std::{error::Error, io};

use tabled::builder::Builder;

pub struct CSV {
    headers: csv::StringRecord,
    records: Vec<csv::StringRecord>,
}

impl CSV {
    pub fn from_stdin() -> Result<Self, Box<dyn Error>> {
        let mut reader = csv::Reader::from_reader(io::stdin());
        let headers = reader.headers()?.clone();
        let records: Vec<csv::StringRecord> = reader.records().map(|r| r.unwrap()).collect();

        Ok(CSV { headers, records })
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut reader = csv::Reader::from_path(path).unwrap();
        let headers = reader.headers()?.clone();
        let records: Vec<csv::StringRecord> = reader.records().map(|r| r.unwrap()).collect();

        Ok(CSV { headers, records })
    }

    pub fn new(path: Option<String>) -> Result<Self, Box<dyn Error>> {
        if let Some(p) = path {
            CSV::from_file(&p)
        } else {
            CSV::from_stdin()
        }
    }

    pub fn sample(&mut self, count: usize) -> &Self {
        let mut rng = rand::thread_rng();
        let sample: Vec<csv::StringRecord> = self
            .records
            .iter()
            .choose_multiple(&mut rng, count)
            .into_iter()
            .map(|x| x.clone())
            .collect();

        self.records = sample;
        self
    }

    pub fn count(&self) -> usize {
        self.records.len()
    }

    pub fn print_csv(&self) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        wtr.write_record(&self.headers)?;

        for record in self.records.iter() {
            wtr.write_record(record)?;
        }

        wtr.flush()?;
        Ok(())
    }

    pub fn print_table(&self) -> Result<(), Box<dyn Error>> {
        let table = Builder::from_iter(&self.records)
            .set_header(&self.headers)
            .build();

        println!("{}", table);

        Ok(())
    }

    pub fn print_headers(&self) -> Result<(), Box<dyn Error>> {
        for (i, header) in self.headers.into_iter().enumerate() {
            println!("{}. {}", i, header);
        }

        Ok(())
    }
}
