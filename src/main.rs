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
        Ok(mut file) => {
            // unhandled error
            collection_handler(file.records());
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn collection_handler(collection: StringRecordsIter<File>) {
    let mut records: Vec<record::Record> = Vec::new();
    for row in collection {
        // Note: records will be in reverse order from collection
        let record: Result<record::Record, csv::Error> = row.unwrap().deserialize(None);
        match record {
            Ok(r) => {
                println!("{}", &r);
                records.push(r);
            },
            Err(e) => { println!("{}", e); }
        }
        
    }
}
