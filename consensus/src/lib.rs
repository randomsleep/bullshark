use tokio::sync::broadcast::Receiver;

use primitives::transaction::Transaction;

pub mod simple_consensus;
// Consensus drives the blockchain network
pub trait Consensus {
    fn subscribe_transaction(&self) -> Receiver<Vec<Transaction>>;
}
