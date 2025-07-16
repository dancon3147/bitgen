use serde::{Serialize, Deserialize};
use crate::transaction::SignedTransaction;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub previous_hash: String,
    pub timestamp: i64,
    pub nonce: u64,
    pub merkle_root: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<SignedTransaction>,
}

impl Block {
    pub fn hash(&self) -> String {
        let header_str = format!(
            "{}{}{}{}",
            self.header.previous_hash,
            self.header.timestamp,
            self.header.nonce,
            self.header.merkle_root
        );
        let hash = Sha256::digest(header_str.as_bytes());
        format!("{:x}", hash)
    }

    pub fn calculate_merkle_root(transactions: &[SignedTransaction]) -> String {
        if transactions.is_empty() {
            return "0".repeat(64);
        }
        let mut hashes: Vec<String> = transactions
            .iter()
            .map(|tx| {
                let s = serde_json::to_string(tx).unwrap();
                format!("{:x}", Sha256::digest(s.as_bytes()))
            })
            .collect();

        while hashes.len() > 1 {
            let mut next_level = vec![];
            for i in (0..hashes.len()).step_by(2) {
                let left = &hashes[i];
                let right = if i + 1 < hashes.len() { &hashes[i + 1] } else { left };
                let combined = format!("{}{}", left, right);
                next_level.push(format!("{:x}", Sha256::digest(combined.as_bytes())));
            }
            hashes = next_level;
        }

        hashes[0].clone()
    }
}