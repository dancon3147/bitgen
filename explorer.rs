use crate::block::{Block};
use crate::blockchain::load_chain;
use std::fs::File;
use std::io::Write;

pub fn explore(summary: bool, latest: bool, export_html: Option<String>) {
    let chain = load_chain("bitgen_chain.json");

    if let Some(path) = export_html {
        export_to_html(&chain, &path);
        return;
    }

    if summary {
        println!("📦 BitGen Chain Summary");
        println!("Total Blocks: {}", chain.len());
        for (i, block) in chain.iter().enumerate() {
            println!(
                "#{} | Hash: {} | Tx: {} | Timestamp: {}",
                i,
                block.hash(),
                block.transactions.len(),
                block.header.timestamp
            );
        }
    } else if latest {
        if let Some(block) = chain.last() {
            println!("🔍 Latest Block:");
            println!("{:#?}", block);
        } else {
            println!("No blocks found.");
        }
    }
}

fn export_to_html(chain: &[Block], path: &str) {
    let mut html = String::from("<!DOCTYPE html><html><head><meta charset='UTF-8'><title>BitGen Explorer</title><style>body{font-family:sans-serif;}table{width:100%;border-collapse:collapse;}th,td{border:1px solid #ccc;padding:8px;}th{background:#eee;}</style></head><body>");
    html.push_str("<h1>BitGen Blockchain Explorer</h1><table><tr><th>#</th><th>Hash</th><th>Tx Count</th><th>Timestamp</th></tr>");

    for (i, block) in chain.iter().enumerate() {
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            i,
            block.hash(),
            block.transactions.len(),
            block.header.timestamp
        ));
    }

    html.push_str("</table></body></html>");
    let mut file = File::create(path).expect("Failed to create HTML file");
    file.write_all(html.as_bytes()).expect("Write failed");
    println!("📄 Explorer HTML generated at {}", path);
}