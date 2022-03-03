use std::fs;
use tabled::{builder::Builder, Table};

pub struct CSV<'a> {
    headers: Vec<&'a str>,
    rows: Vec<Vec<&'a str>>,
}

impl<'a> CSV<'a> {
    pub fn parse(csv: &'a str, delimiter: Option<char>) -> Self {
        let delimiter = delimiter.unwrap_or(',');

        let mut lines = csv.lines();
        let headers: Vec<&str> = lines.next().unwrap_or("").split(delimiter).collect();
        let rows: Vec<Vec<&str>> = lines
            .map(|line: &str| line.split(delimiter).map(|el| el.trim()).collect())
            .collect();

        CSV { headers, rows }
    }

    pub fn to_csv_string(&self, delimiter: Option<char>) -> String {
        let delimiter = &delimiter.unwrap_or(',').to_string();
        let headers = self.headers.join(delimiter);
        let rows = self
            .rows
            .iter()
            .map(|row| row.join(delimiter))
            .collect::<Vec<String>>()
            .join("\n");

        (vec![headers, rows]).join("\n")
    }

    pub fn to_table(&self) -> Table {
        Builder::from_iter(&self.rows)
            .set_header(&self.headers)
            .build()
    }
}

fn main() {
    let csv = fs::read_to_string("test-data.csv").unwrap();
    let csv = CSV::parse(&csv, None);

    println!("{}", csv.to_table());
}
