use std::fmt;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, PartialEq)]
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

#[derive(Deserialize, Clone, Copy)]
pub struct Record {
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>
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