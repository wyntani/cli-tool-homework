use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Spl-Airdropper")]
#[command(about = "A CLI tool for creating SPL tokens and airdropping them to recipients")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Create a new SPL token mint")]
    CreateToken {
        #[arg(long, help = "Number of decimal places for the token")]
        decimals: u8,
    },
    #[command(about = "Airdrop tokens to a list of recipients")]
    Airdrop {
        #[arg(long, help = "Token mint address")]
        token_mint: String,
        #[arg(long, help = "Amount of tokens to airdrop to each recipient")]
        amount: u64,
        #[arg(long, help = "Path to file containing recipient addresses")]
        recipients: PathBuf,
    },
    #[command(about = "Check token balance for an address")]
    CheckBalance {
        #[arg(long, help = "Token mint address")]
        token_mint: String,
        #[arg(long, help = "Wallet address to check balance for")]
        address: String,
    },
}
