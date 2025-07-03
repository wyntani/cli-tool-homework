use clap::Parser;
use spl_airdropper::cli::interface::{Cli, Commands};
use spl_airdropper::executor::rpc::TokenExecutor;
use spl_airdropper::utils::custom_io_tools::load_recipients;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let executor = TokenExecutor::new()?;

    // from_str and println! shoud be replaced outside of the function to be more idiomatic and "real usage case"
    // BUT it will be better for prototype
    match cli.command {
        Commands::CreateToken { decimals } => {
            executor.create_token(decimals).await?;
        }
        Commands::Airdrop {
            token_mint,
            amount,
            recipients,
        } => {
            let recipients = load_recipients(&recipients)?;

            executor
                .airdrop_to_recipients(&token_mint, amount, recipients)
                .await?;
        }
        Commands::BatchAirdrop {
            token_mint,
            amount,
            recipients,
        } => {
            let recipients = load_recipients(&recipients)?;

            executor
                .batch_airdrop_to_recipients(&token_mint, recipients, amount)
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
