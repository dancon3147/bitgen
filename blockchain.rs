use std::fs::{File};
use std::io::{BufReader, BufWriter};
use crate::block::Block;

pub fn load_chain(path: &str) -> Vec<Block> {
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

pub fn save_chain(path: &str, chain: &Vec<Block>) {
    let file = File::create(path).expect("Unable to create chain file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, chain).expect("Unable to write chain");
}