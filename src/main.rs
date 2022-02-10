use clap::Parser;
use csv::{self, Trim, StringRecordsIter};
use std::{fs::File};
mod record;

/// The transaction engine takes in a CSV file and compiles account snapshots from the transactions in the CSV file.
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Filepath for a CSV file to parse
    #[clap(short, long)]
    filepath: String
}

fn main() {
    let args = Args::parse();
    let reader = csv::ReaderBuilder::new().trim(Trim::All).from_path(args.filepath);
    match reader {
        Ok(mut records) => {
            collection_handler(records.records());
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn collection_handler(collection: StringRecordsIter<File>) {
    for row in collection {
        for column in row {
            println!("{}", column);
        }
    }
}
