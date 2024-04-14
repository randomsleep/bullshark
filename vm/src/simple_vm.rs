use std::collections::HashMap;

use primitives::{address::Address, transaction::Transaction};

use crate::VM;


#[derive(Default)]
pub struct SimpleVM {
    state: HashMap<Address, u128>,
}

impl VM for SimpleVM {
    fn execute(&mut self, transactions: Vec<Transaction>) {
        for transaction in transactions {
            println!("Executing transaction: {:}", transaction);
            self.state.insert(transaction.address, transaction.balance);
        }
    }
}