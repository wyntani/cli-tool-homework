use clap::Parser;
use spl_airdropper::cli::interface::{Cli, Commands};
use spl_airdropper::executor::rpc::TokenExecutor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let executor = TokenExecutor::new()?;

    match cli.command {
        Commands::CreateToken { decimals } => {
            executor.create_token(decimals).await?;
        }
        Commands::Airdrop {
            token_mint,
            amount,
            recipients,
        } => {
            executor
                .airdrop_to_recipients(&token_mint, amount, &recipients)
                .await?;
        }
        Commands::CheckBalance {
            token_mint,
            address,
        } => {
            executor.check_balance(&token_mint, &address).await?;
        }
    }

    Ok(())
}
