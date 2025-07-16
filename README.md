# BitGen

> A next-generation digital gold blockchain — decentralized, verifiable, and built for long-term value.

![MIT License](https://img.shields.io/badge/license-MIT-green.svg)
![Rust](https://img.shields.io/badge/language-Rust-orange)
[![GitHub Pages](https://img.shields.io/badge/Explorer-Live-blue)](https://dancon3147.github.io/bitgen/)

---

## 🧱 What is BitGen?

**BitGen** is a lightweight, modular blockchain written in Rust that simulates the store-of-value properties of Bitcoin with:

- 💠 Blake3-based Proof of Work
- 🔐 Ed25519 transaction signing
- 📦 JSON-based chain + mempool
- 🧭 CLI explorer and static HTML output
- 🧪 Wallet generation, transaction signing, block mining

---

## 🚀 Quickstart

```bash
# Build
cargo build --release

# Create Wallet
cargo run -- wallet --create --file mywallet.dat
cargo run -- wallet --show --file mywallet.dat

# Submit Transaction
cargo run -- tx --sender-file mywallet.dat --recipient <ADDRESS> --amount 10

# Mine Block
cargo run -- miner

# Explore Chain
cargo run -- explorer --summary
cargo run -- explorer --export-html blockchain.html
