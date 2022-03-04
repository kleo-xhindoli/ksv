use std::{
    error::Error,
    fs::File,
    io::{self, Stdin},
    process,
};

use tabled::builder::Builder;

pub struct CSV<T: io::Read> {
    reader: csv::Reader<T>,
}

impl<T: io::Read> CSV<T> {
    pub fn print_csv(&mut self) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_writer(io::stdout());

        wtr.write_record(self.reader.headers()?)?;

        for result in self.reader.records() {
            let record = result?;
            wtr.write_record(&record)?;
        }

        wtr.flush()?;
        Ok(())
    }

    pub fn print_table(&mut self) -> Result<(), Box<dyn Error>> {
        let records: Vec<csv::StringRecord> = self.reader.records().map(|r| r.unwrap()).collect();
        let headers = self.reader.headers()?;

        let table = Builder::from_iter(&records).set_header(headers).build();

        println!("{}", table);

        Ok(())
    }

    pub fn count(&mut self) -> usize {
        self.reader.records().count()
    }

    pub fn print_headers(&mut self) -> Result<(), Box<dyn Error>> {
        let headers = self.reader.headers()?;

        for (i, header) in headers.into_iter().enumerate() {
            println!("{}. {}", i, header);
        }

        Ok(())
    }
}

impl CSV<Stdin> {
    pub fn from_stdin() -> Self {
        let reader = csv::Reader::from_reader(io::stdin());

        CSV { reader }
    }
}

impl CSV<File> {
    pub fn from_file(path: &str) -> Self {
        let reader = csv::Reader::from_path(path).unwrap();

        CSV { reader }
    }
}

fn main() {
    let mut data = CSV::from_stdin();

    if let Err(e) = data.print_table() {
        println!("{}", e);
        process::exit(1);
    }
}
