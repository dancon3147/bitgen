# BitGen Developer Guide

## 🛠 Build
```bash
cargo build --release
```

## 🧪 Test Wallet + Transactions
```bash
cargo run -- wallet --create --file mywallet.dat
cargo run -- wallet --show --file mywallet.dat
cargo run -- tx --sender-file mywallet.dat --recipient <ADDRESS> --amount 25
```

## ⛏ Mine a Block
```bash
cargo run -- miner
```

## 🔍 Explore Blockchain
```bash
cargo run -- explorer --summary
cargo run -- explorer --latest
cargo run -- explorer --export-html blockchain.html
```

## 📂 Files
- `bitgen_chain.json` – stores mined blocks
- `mempool_tx.json` – holds unconfirmed signed transactions