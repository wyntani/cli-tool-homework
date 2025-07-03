// use spl_airdropper::executor::TokenExecutor;
use clap::Parser;
use spl_airdropper::cli::interface::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateToken { decimals } => {
            unimplemented!();
        }
        Commands::Airdrop {
            token_mint,
            amount,
            recipients,
        } => {
            unimplemented!();
        }
        Commands::CheckBalance {
            token_mint,
            address,
        } => {
            unimplemented!();
        }
    }

    Ok(())
}
