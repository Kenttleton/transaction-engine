use crate::client::Client;
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    DEPOSIT,
    WITHDRAWAL,
    DISPUTE,
    RESOLVE,
    CHARGEBACK,
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

#[derive(Deserialize, Clone, Copy)]
pub struct Record {
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut amount = "".to_string();
        match self.amount {
            Some(value) => {
                amount = value.to_string();
            }
            None => {}
        }
        write!(
            f,
            "type: {}, client: {}, tx: {}, amount: {}",
            self.transaction_type, self.client, self.tx, amount
        )
    }
}

impl Record {
    pub fn process(self, transactions: &Vec<Record>, output: Vec<Client>) -> Vec<Client> {
        match self.transaction_type {
            TransactionType::DEPOSIT => {
                let (index, output) = self.find_or_add_client(output.clone());
                self.deposit(index, output)
            }
            TransactionType::WITHDRAWAL => {
                let (index, output) = self.find_or_add_client(output.clone());
                self.withdrawal(index, output)
            }
            TransactionType::DISPUTE => {
                let (index, output) = self.find_or_add_client(output.clone());
                self.dispute(index, transactions.clone(), output)
            }
            TransactionType::RESOLVE => {
                let (index, output) = self.find_or_add_client(output.clone());
                self.resolve(index, transactions.clone(), output)
            }
            TransactionType::CHARGEBACK => {
                let (index, output) = self.find_or_add_client(output.clone());
                self.chargeback(index, transactions.clone(), output)
            }
        }
    }

    fn find_or_add_client(self, output: Vec<Client>) -> (usize, Vec<Client>) {
        let client = output.iter().position(|x| x.client == self.client);
        match client {
            Some(x) => (x, output),
            None => self.add_client(output),
        }
    }

    fn add_client(self, mut output: Vec<Client>) -> (usize, Vec<Client>) {
        let client = Client {
            client: self.client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        };
        output.push(client);
        self.find_or_add_client(output)
    }

    fn get_amount(self, transactions: Vec<Record>) -> f64 {
        match transactions.iter().find(|x| {
            x.client == self.client
                && x.tx == self.tx
                && x.transaction_type != TransactionType::CHARGEBACK
                && x.transaction_type != TransactionType::RESOLVE
                && x.transaction_type != TransactionType::DISPUTE
        }) {
            Some(x) => x.amount.unwrap(),
            None => 0.0,
        }
    }

    fn has_dispute(self, transactions: Vec<Record>) -> bool {
        match transactions.iter().find(|x| {
            x.client == self.client
                && x.tx == self.tx
                && x.transaction_type == TransactionType::DISPUTE
        }) {
            Some(_) => true,
            None => false,
        }
    }
    fn dispute_type(self, transactions: Vec<Record>) -> TransactionType {
        match transactions.iter().find(|x| {
            x.client == self.client
                && x.tx == self.tx
                && x.transaction_type != TransactionType::CHARGEBACK
                && x.transaction_type != TransactionType::RESOLVE
                && x.transaction_type != TransactionType::DISPUTE
        }) {
            Some(x) => x.transaction_type,
            None => TransactionType::DISPUTE,
        }
    }

    fn is_not_locked(self, output: Vec<Client>) -> bool {
        match output.iter().find(|x| x.client == self.client && x.locked) {
            Some(_) => false,
            None => true,
        }
    }
    fn deposit(self, index: usize, mut output: Vec<Client>) -> Vec<Client> {
        if self.is_not_locked(output.clone()) {
            output[index].available += self.amount.unwrap();
            output[index].total += self.amount.unwrap();
        }
        output
    }
    fn withdrawal(self, index: usize, mut output: Vec<Client>) -> Vec<Client> {
        if output[index].available >= self.amount.unwrap() && self.is_not_locked(output.clone()) {
            output[index].available -= self.amount.unwrap();
            output[index].total -= self.amount.unwrap();
        }
        output
    }
    fn dispute(
        self,
        index: usize,
        transactions: Vec<Record>,
        mut output: Vec<Client>,
    ) -> Vec<Client> {
        if self.is_not_locked(output.clone()) {
            let amount = self.get_amount(transactions.clone());
            //println!("dispute amount: {}", amount);
            let dispute_type = self.dispute_type(transactions);
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
        self,
        index: usize,
        transactions: Vec<Record>,
        mut output: Vec<Client>,
    ) -> Vec<Client> {
        if self.is_not_locked(output.clone()) && self.has_dispute(transactions.clone()) {
            let amount = self.get_amount(transactions.clone());
            //println!("resolve amount: {}", amount);
            let dispute_type = self.dispute_type(transactions);
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
        self,
        index: usize,
        transactions: Vec<Record>,
        mut output: Vec<Client>,
    ) -> Vec<Client> {
        if self.is_not_locked(output.clone()) && self.has_dispute(transactions.clone()) {
            let amount = self.get_amount(transactions.clone());
            //println!("chargeback amount: {}", amount);
            let dispute_type = self.dispute_type(transactions);
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
}
