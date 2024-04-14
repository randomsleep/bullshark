
pub mod simple_vm;

use primitives::transaction::Transaction;

pub trait VM {
   fn execute(&mut self, transactions: Vec<Transaction>);
}