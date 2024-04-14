
use std::fmt;

use crate::address::Address;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Transaction {
    pub address: Address,
    pub balance: u128,
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction: address: {}, balance: {}", self.address, self.balance)
    }
}