use clap::{Parser, Subcommand};

mod blockchain;
mod block;
mod transaction;
mod miner;
mod wallet;
mod explorer;

#[derive(Parser)]
#[command(name = "bitgen", version = "0.1.0", author = "Daniel Conrad")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Mine new blocks
    Miner,
    /// Create or use wallet
    Wallet {
        #[arg(long)] create: bool,
        #[arg(long)] file: Option<String>,
        #[arg(long)] show: bool,
    },
    /// Submit a transaction
    Tx {
        #[arg(long)] sender_file: String,
        #[arg(long)] recipient: String,
        #[arg(long)] amount: u64,
    },
    /// Explore the blockchain
    Explorer {
        #[arg(long)] summary: bool,
        #[arg(long)] latest: bool,
        #[arg(long)] export_html: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Miner => miner::start_miner(),
        Commands::Wallet { create, file, show } => wallet::handle_wallet(create, file, show),
        Commands::Tx { sender_file, recipient, amount } => {
            wallet::submit_transaction(sender_file, recipient, amount)
        }
        Commands::Explorer { summary, latest, export_html } => {
            explorer::explore(summary, latest, export_html)
        }
    }
}