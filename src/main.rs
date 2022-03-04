use std::process;

use clap::{Parser, Subcommand};
use ksv::CSV;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Table { csv: Option<String> },
    Count { csv: Option<String> },
    Headers { csv: Option<String> },
    Sample { count: usize, csv: Option<String> },
    Search { query: String, csv: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Table { csv } => {
            let data = CSV::new(csv).unwrap();

            if let Err(e) = data.print_table() {
                println!("{}", e);
                process::exit(1);
            }
        }
        Commands::Count { csv } => {
            let data = CSV::new(csv).unwrap();

            let c = data.count();
            println!("{}", c);
        }
        Commands::Headers { csv } => {
            let data = CSV::new(csv).unwrap();

            if let Err(e) = data.print_headers() {
                println!("{}", e);
                process::exit(1);
            }
        }
        Commands::Sample { count, csv } => {
            let mut data = CSV::new(csv).unwrap();

            if let Err(e) = data.sample(count).print_csv() {
                println!("{}", e);
                process::exit(1);
            }
        }
        Commands::Search { query, csv } => {
            let mut data = CSV::new(csv).unwrap();

            if let Err(e) = data.search(&query).print_csv() {
                println!("{}", e);
                process::exit(1);
            }
        }
    }
}
