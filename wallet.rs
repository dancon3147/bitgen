use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use crate::transaction::{Transaction, SignedTransaction};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn handle_wallet(create: bool, file: Option<String>, show: bool) {
    let path = file.unwrap_or_else(|| "mywallet.dat".to_string());
    if create {
        let mut csprng = OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        let encoded = bincode::serialize(&keypair).unwrap();
        fs::write(&path, encoded).expect("Unable to write wallet file");
        println!("Wallet created: {}", path);
    } else if show {
        let data = fs::read(&path).expect("Unable to read wallet file");
        let keypair: Keypair = bincode::deserialize(&data).unwrap();
        let pubkey = keypair.public.as_bytes();
        let hash = Sha256::digest(pubkey);
        let address = bs58::encode(hash).into_string();
        println!("Address: {}", address);
    }
}

pub fn submit_transaction(sender_file: String, recipient: String, amount: u64) {
    let data = fs::read(&sender_file).expect("Unable to read wallet file");
    let keypair: Keypair = bincode::deserialize(&data).unwrap();

    let pubkey_bytes = keypair.public.as_bytes();
    let sender = bs58::encode(Sha256::digest(pubkey_bytes)).into_string();

    let tx = Transaction {
        sender,
        recipient,
        amount,
    };

    let signature = keypair.sign(tx.to_string().as_bytes()).to_bytes().to_vec();

    let signed_tx = SignedTransaction {
        transaction: tx,
        signature,
        public_key: pubkey_bytes.to_vec(),
    };

    let path = Path::new("mempool_tx.json");
    let mut txs = if path.exists() {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    txs.push(signed_tx);
    let file = File::create(path).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &txs).unwrap();
    println!("Transaction submitted to mempool_tx.json");
}