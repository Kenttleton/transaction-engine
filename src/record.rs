use std::fmt;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAWAL,
    DISPUTE,
    RESOLVE,
    CHARGEBACK
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TransactionType::DEPOSIT => write!(f, "deposit"),
            TransactionType::WITHDRAWAL => write!(f, "withdrawal"),
            TransactionType::DISPUTE => write!(f, "dispute"),
            TransactionType::RESOLVE => write!(f, "resolve"),
            TransactionType::CHARGEBACK => write!(f, "chargeback")
        }
    }
}

#[derive(Deserialize)]
pub struct Record {
    transaction_type: TransactionType,
    client: u16,
    tx: u32,
    amount: Option<f64>
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut amount = "".to_string();
        match self.amount {
            Some(value) => { amount = value.to_string(); },
            None => {}
        }
        write!(f, "type: {}, client: {}, tx: {}, amount: {}", self.transaction_type, self.client, self.tx, amount)
    }
}