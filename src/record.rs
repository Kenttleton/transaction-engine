use std::fmt;

pub enum TransactionType {
    NONE,
    DEPOSIT,
    WITHDRAWAL,
    DISPUTE,
    RESOLVE,
    CHARGEBACK
}

impl TransactionType {
    fn new(transaction: String) -> Self {
        match transaction.to_lowercase().as_str() {
            "deposit" => TransactionType::DEPOSIT,
            "withdrawal" => TransactionType::WITHDRAWAL,
            "dispute" => TransactionType::DISPUTE,
            "resolve" => TransactionType::RESOLVE,
            "chargeback" => TransactionType::CHARGEBACK,
            _ => TransactionType::NONE
        }
    }
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TransactionType::DEPOSIT => write!(f, "deposit"),
            TransactionType::WITHDRAWAL => write!(f, "withdrawal"),
            TransactionType::DISPUTE => write!(f, "dispute"),
            TransactionType::RESOLVE => write!(f, "resolve"),
            TransactionType::CHARGEBACK => write!(f, "chargeback"),
        }
    }
}

pub struct Record {
    transaction_type: TransactionType,
    client: u16,
    tx: u32,
    amount: f64
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type: {}, client: {}, tx: {}, amount: {}", self.transaction_type, self.client, self.tx, self.amount)
    }
}

impl Record {
    fn new() -> Self {

    }
}