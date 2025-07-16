# BitGen Architecture

BitGen is a minimal, modular blockchain written in Rust that serves as a next-generation store of value (digital gold). Key features include:

- **Proof of Work (PoW)** using SHA-256 with a Blake3-inspired structure
- **Merkle root hashing** for transaction integrity
- **Persistent storage** via JSON-based chain files
- **Modular CLI** for mining, wallet, and blockchain exploration
- **Wallet system** with Ed25519 cryptographic signatures
- **Explorer** for both terminal and HTML views