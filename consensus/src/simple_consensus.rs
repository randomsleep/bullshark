use std::sync::{Arc, Mutex};

use primitives::{address::Address, transaction::Transaction};
use tokio::sync::broadcast::{Receiver, Sender};
use rand::Rng;

use crate::Consensus;

type Transactions = Vec<Transaction>;

pub struct SimpleConsensus {
    transaction_sender: Arc<Mutex<Sender<Transactions>>>,
}

impl Consensus for SimpleConsensus {
    fn subscribe_transaction(&self) -> Receiver<Transactions> {
        self.transaction_sender.lock().expect("Lock poisoned").subscribe()
    }
}

impl SimpleConsensus {
    pub fn spawn(interval_sec: u64) -> SimpleConsensus {
        let (transaction_sender, _) = tokio::sync::broadcast::channel(100);
        let sender = Arc::new(Mutex::new(transaction_sender));

        let sender1 = sender.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(interval_sec)).await;
                let transactions = vec![
                    Transaction {
                        address: Address::from("Alice"),
                        balance: rand::thread_rng().gen_range(0..100),
                    },
                    Transaction {
                        address: Address::from("Bob"),
                        balance: rand::thread_rng().gen_range(0..100),
                    },
                ];
                sender1.lock().expect("").send(transactions).unwrap();
            }
        });
            
        SimpleConsensus {
            transaction_sender: sender,
        }
    }
}
