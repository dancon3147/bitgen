use crate::block::{Block, BlockHeader};
use crate::blockchain::{load_chain, save_chain};
use crate::transaction::SignedTransaction;
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::fs::{File};
use std::io::BufReader;

pub fn start_miner() {
    let mut chain = load_chain("bitgen_chain.json");

    let previous_hash = chain.last().map(|b| b.hash()).unwrap_or_else(|| "0".repeat(64));
    let mempool: Vec<SignedTransaction> = if let Ok(file) = File::open("mempool_tx.json") {
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    let valid_txs: Vec<SignedTransaction> = mempool
        .into_iter()
        .filter(|tx| tx.verify())
        .collect();

    let merkle_root = Block::calculate_merkle_root(&valid_txs);
    let timestamp = Utc::now().timestamp();
    let mut nonce = 0;

    println!("⛏️  Mining block...");
    loop {
        let header = BlockHeader {
            previous_hash: previous_hash.clone(),
            timestamp,
            nonce,
            merkle_root: merkle_root.clone(),
        };

        let candidate = Block {
            header,
            transactions: valid_txs.clone(),
        };

        let hash = candidate.hash();
        if hash.starts_with("0000") {
            println!("✅ Block mined! Hash: {}", hash);
            chain.push(candidate);
            save_chain("bitgen_chain.json", &chain);
            break;
        }

        nonce += 1;
    }
}