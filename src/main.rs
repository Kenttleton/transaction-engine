//use clap::Parser;
use csv::{self, StringRecordsIter, Trim};
use std::fs::File;
use std::{env, thread};
mod client;
mod record;
use client::Client;
use log::{error, LevelFilter};
use record::{Record, TransactionType};
use simple_logging;

// /// The transaction engine takes in a CSV file and compiles account snapshots from the transactions in the CSV file.
// #[derive(Parser, Debug)]
// #[clap(author, version, about)]
// struct Args {
//     /// Filepath for a CSV file to parse
//     #[clap(short, long)]
//     filepath: String
// }

const STACK_SIZE: usize = 2 * 1024 * 1024;

fn main() {
    //let args = Args::parse();
    // Set environment for debugging
    std::env::set_var("RUST_BACKTRACE", "full");
    simple_logging::log_to_file("./log/log.txt", LevelFilter::Info).unwrap();
    // Create a child thread that is larger than the Windows default to handle larger files
    let engine_thread = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || {
            let args: Vec<String> = env::args().collect();
            let reader = csv::ReaderBuilder::new()
                .trim(Trim::All)
                .from_path(&args[1]);
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
        output = process_record(*r, &transactions_copy, output);
    }
    output
}

fn process_record(record: Record, transactions: &Vec<Record>, output: Vec<Client>) -> Vec<Client> {
    //println!("{}", record);
    match record.transaction_type {
        TransactionType::DEPOSIT => {
            let (index, output) = find_or_add_client(record.clone(), output.clone());
            deposit(index, record.clone(), output)
        }
        TransactionType::WITHDRAWAL => {
            let (index, output) = find_or_add_client(record.clone(), output.clone());
            withdrawal(index, record.clone(), output)
        }
        TransactionType::DISPUTE => {
            let (index, output) = find_or_add_client(record.clone(), output.clone());
            dispute(index, record.clone(), transactions.clone(), output)
        }
        TransactionType::RESOLVE => {
            let (index, output) = find_or_add_client(record.clone(), output.clone());
            resolve(index, record.clone(), transactions.clone(), output)
        }
        TransactionType::CHARGEBACK => {
            let (index, output) = find_or_add_client(record.clone(), output.clone());
            chargeback(index, record.clone(), transactions.clone(), output)
        }
    }
}

fn find_or_add_client(record: Record, output: Vec<Client>) -> (usize, Vec<Client>) {
    let client = output.iter().position(|x| x.client == record.client);
    match client {
        Some(x) => (x, output),
        None => add_client(record, output),
    }
}

fn add_client(record: Record, mut output: Vec<Client>) -> (usize, Vec<Client>) {
    let client = Client {
        client: record.client,
        available: 0.0,
        held: 0.0,
        total: 0.0,
        locked: false,
    };
    output.push(client);
    find_or_add_client(record, output)
}

fn get_amount(record: Record, transactions: Vec<Record>) -> f64 {
    match transactions.iter().find(|x| {
        x.client == record.client
            && x.tx == record.tx
            && x.transaction_type != TransactionType::CHARGEBACK
            && x.transaction_type != TransactionType::RESOLVE
            && x.transaction_type != TransactionType::DISPUTE
    }) {
        Some(x) => x.amount.unwrap(),
        None => 0.0,
    }
}

fn has_dispute(record: Record, transactions: Vec<Record>) -> bool {
    match transactions.iter().find(|x| {
        x.client == record.client
            && x.tx == record.tx
            && x.transaction_type == TransactionType::DISPUTE
    }) {
        Some(_) => true,
        None => false,
    }
}

// Needed to distinguish between withdrawals and deposits on disputes
fn dispute_type(record: Record, transactions: Vec<Record>) -> TransactionType {
    match transactions.iter().find(|x| {
        x.client == record.client
            && x.tx == record.tx
            && x.transaction_type != TransactionType::CHARGEBACK
            && x.transaction_type != TransactionType::RESOLVE
            && x.transaction_type != TransactionType::DISPUTE
    }) {
        Some(x) => x.transaction_type,
        None => TransactionType::DISPUTE,
    }
}

fn is_not_locked(record: Record, output: Vec<Client>) -> bool {
    match output
        .iter()
        .find(|x| x.client == record.client && x.locked)
    {
        Some(_) => false,
        None => true,
    }
}

fn deposit(index: usize, record: Record, mut output: Vec<Client>) -> Vec<Client> {
    if is_not_locked(record, output.clone()) {
        output[index].available += record.amount.unwrap();
        output[index].total += record.amount.unwrap();
    }
    output
}

fn withdrawal(index: usize, record: Record, mut output: Vec<Client>) -> Vec<Client> {
    if output[index].available >= record.amount.unwrap() && is_not_locked(record, output.clone()) {
        output[index].available -= record.amount.unwrap();
        output[index].total -= record.amount.unwrap();
    }
    output
}

fn dispute(
    index: usize,
    record: Record,
    transactions: Vec<Record>,
    mut output: Vec<Client>,
) -> Vec<Client> {
    if is_not_locked(record, output.clone()) {
        let amount = get_amount(record, transactions.clone());
        //println!("dispute amount: {}", amount);
        let dispute_type = dispute_type(record, transactions);
        if dispute_type == TransactionType::DEPOSIT {
            output[index].held += amount;
            output[index].available -= amount;
        } else if dispute_type == TransactionType::WITHDRAWAL {
            output[index].held += amount;
        }
    }
    output
}

fn resolve(
    index: usize,
    record: Record,
    transactions: Vec<Record>,
    mut output: Vec<Client>,
) -> Vec<Client> {
    if is_not_locked(record, output.clone()) && has_dispute(record, transactions.clone()) {
        let amount = get_amount(record, transactions.clone());
        //println!("resolve amount: {}", amount);
        let dispute_type = dispute_type(record, transactions);
        if dispute_type == TransactionType::DEPOSIT {
            output[index].held -= amount;
            output[index].available += amount;
        } else if dispute_type == TransactionType::WITHDRAWAL {
            output[index].held -= amount;
        }
    }
    output
}

fn chargeback(
    index: usize,
    record: Record,
    transactions: Vec<Record>,
    mut output: Vec<Client>,
) -> Vec<Client> {
    if is_not_locked(record, output.clone()) && has_dispute(record, transactions.clone()) {
        let amount = get_amount(record, transactions.clone());
        //println!("chargeback amount: {}", amount);
        let dispute_type = dispute_type(record, transactions);
        if dispute_type == TransactionType::DEPOSIT {
            output[index].held -= amount;
            output[index].total -= amount;
            output[index].locked = true;
        } else if dispute_type == TransactionType::WITHDRAWAL {
            output[index].held -= amount;
            output[index].total += amount;
            output[index].available += amount;
            output[index].locked = true;
        }
    }
    output
}

fn print_output(output: Vec<Client>) {
    println!("Client, Available, Held, Total, Locked");
    for client in output {
        println!("{}", client);
    }
}
