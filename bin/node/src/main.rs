use consensus::{simple_consensus::SimpleConsensus, Consensus};
use vm::{simple_vm::SimpleVM, VM};

#[tokio::main]
async fn main() {
    let consensus = SimpleConsensus::spawn(1);
    let mut vm = SimpleVM::default();

    let handle = tokio::spawn(async move {
        let mut receiver = consensus.subscribe_transaction(); // Declare receiver as mutable
        loop {
            let transactions = receiver.recv().await.unwrap();
            vm.execute(transactions);
        }
    });

    tokio::join!(handle).0.unwrap();
}