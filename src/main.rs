//use clap::Parser;
use csv::{self, StringRecordsIter, Trim};
use std::fs::{metadata, File};
use std::{env, thread};
pub mod client;
mod record;
use client::Client;
use log::{error, info, LevelFilter};
use record::Record;
use simple_logging;
use sysinfo::{System, SystemExt};

// /// The transaction engine takes in a CSV file and compiles account snapshots from the transactions in the CSV file.
// #[derive(Parser, Debug)]
// #[clap(author, version, about)]
// struct Args {
//     /// Filepath for a CSV file to parse
//     #[clap(short, long)]
//     filepath: String
// }

fn main() {
    //let args = Args::parse();
    // Set environment for debugging
    std::env::set_var("RUST_BACKTRACE", "full");
    simple_logging::log_to_file("./log/log.txt", LevelFilter::Info).unwrap();
    let args: Vec<String> = env::args().collect();
    // Create a stack memory size based on 2 times the file size or 2 MB
    let stack_size: usize = match metadata(&args[1]) {
        Ok(l) => {
            let file_size = l.len();
            info!("Stack size set to {} Bytes", 2 * file_size);
            2 * file_size as usize
        }
        Err(e) => {
            let default_windows = 1024 * 1024;
            error!("{}", e);
            info!("Stack size set to {} Bytes", 2 * default_windows);
            2 * default_windows
        }
    };
    thread_handler(stack_size, args[1].clone());
}

fn thread_handler(mut stack_size: usize, path: String) {
    let mut sys = System::new();
    sys.refresh_memory();
    // Convert to Bytes from KB
    let free_mem = (sys.free_memory() * 1024) as usize;
    // if stack size is over half available/free memory, set to half available/free memory
    if stack_size > free_mem {
        stack_size = free_mem / 2;
        info!("Stack size is too large. Resetting to {} Bytes", stack_size);
    }
    // Create a child thread that is larger than the Windows default to handle larger files
    let engine_thread = thread::Builder::new()
        .stack_size(stack_size)
        .spawn(move || {
            let reader = csv::ReaderBuilder::new().trim(Trim::All).from_path(path);
            match reader {
                Ok(mut file) => {
                    print_output(file_handler(file.records()));
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        })
        .unwrap();
    engine_thread
        .join()
        .expect("Couldn't join on the associated thread");
}

fn file_handler(file: StringRecordsIter<File>) -> Vec<Client> {
    let mut transactions: Vec<Record> = Vec::new();
    'parse_file: for row in file {
        let record: Result<Record, csv::Error> = match row {
            Ok(r) => r.deserialize(None),
            Err(e) => {
                error!("{}", e);
                continue 'parse_file;
            }
        };
        match record {
            Ok(r) => {
                // Transactions will be in the reverse order of the CSV file
                transactions.push(r);
            }
            Err(e) => {
                error!("{}", e);
                continue 'parse_file;
            }
        }
    }
    // Reverse the transactions so they are in the correct order
    transactions.reverse();
    compile_snapshot(transactions)
}

fn compile_snapshot(transactions: Vec<Record>) -> Vec<Client> {
    let transactions_copy = transactions.clone();
    let mut output: Vec<Client> = Vec::new();
    // Iterate through transactions in the correct order due to reversal
    for r in transactions.iter().rev() {
        output = r.process(&transactions_copy, output);
    }
    output
}

fn print_output(output: Vec<Client>) {
    println!("Client, Available, Held, Total, Locked");
    for client in output {
        println!("{}", client);
    }
}
