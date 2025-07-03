use crate::executor::client::SolanaClient;
use crate::utils::custom_rpc_tool::get_associated_token_address;
use anyhow::Context;
use anyhow::Result;
use solana_sdk::{
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use solana_system_interface::instruction as system_instruction;
use spl_associated_token_account::instruction as ata_instruction;
use spl_token::{instruction as token_instruction, state::Mint};
use std::str::FromStr;

pub struct TokenExecutor {
    client: SolanaClient,
}

impl TokenExecutor {
    pub fn new() -> Result<Self> {
        let client = SolanaClient::new()?;
        Ok(Self { client })
    }

    pub fn new_with_keypair_path(keypair_path: &str) -> Result<Self> {
        let client = SolanaClient::new_with_keypair_path(keypair_path)?;
        Ok(Self { client })
    }

    pub async fn create_token(&self, decimals: u8) -> Result<Pubkey> {
        let mint_keypair = Keypair::new();
        let mint_pubkey = mint_keypair.pubkey();

        let rent = self
            .client
            .rpc_client
            .get_minimum_balance_for_rent_exemption(Mint::LEN)?;

        let instructions = vec![
            system_instruction::create_account(
                &self.client.payer.pubkey(),
                &mint_pubkey,
                rent,
                Mint::LEN as u64,
                &spl_token::id(),
            ),
            token_instruction::initialize_mint(
                &spl_token::id(),
                &mint_pubkey,
                &self.client.payer.pubkey(),
                Some(&self.client.payer.pubkey()),
                decimals,
            )?,
        ];

        let signature = self
            .client
            .send_transaction_with_signer(instructions, &[&self.client.payer, &mint_keypair])?;

        println!("Token created successfully!");
        println!("Token Mint Address: {}", mint_pubkey);
        println!("Transaction Signature: {}", signature);

        Ok(mint_pubkey)
    }

    pub async fn airdrop_to_recipients(
        &self,
        token_mint: &str,
        amount: u64,
        recipients: Vec<Pubkey>,
    ) -> Result<()> {
        let mint_pubkey = Pubkey::from_str(token_mint)
            .with_context(|| format!("Invalid token mint address: {}", token_mint))?;

        println!("Starting airdrop to {} recipients...", recipients.len());

        for (i, recipient) in recipients.iter().enumerate() {
            match self
                .airdrop_to_single_recipient(&mint_pubkey, *recipient, amount)
                .await
            {
                Ok(signature) => {
                    println!(
                        "[{}/{}] Airdropped {} tokens to {}",
                        i + 1,
                        recipients.len(),
                        amount,
                        recipient
                    );
                    println!("Transaction: {}", signature);
                }
                Err(e) => {
                    eprintln!(
                        "[{}/{}] Failed to airdrop to {}: {}",
                        i + 1,
                        recipients.len(),
                        recipient,
                        e
                    );
                }
            }
        }

        Ok(())
    }

    async fn airdrop_to_single_recipient(
        &self,
        mint: &Pubkey,
        recipient: Pubkey,
        amount: u64,
    ) -> Result<String> {
        let associated_token_address = get_associated_token_address(&recipient, mint);

        let mut instructions = vec![];

        let account_info = self
            .client
            .rpc_client
            .get_account(&associated_token_address);

        if account_info.is_err() {
            instructions.push(ata_instruction::create_associated_token_account(
                &self.client.payer.pubkey(),
                &recipient,
                mint,
                &spl_token::id(),
            ));
        }

        instructions.push(token_instruction::mint_to(
            &spl_token::id(),
            mint,
            &associated_token_address,
            &self.client.payer.pubkey(),
            &[],
            amount,
        )?);

        let signature = self
            .client
            .send_transaction_with_signer(instructions, &[&self.client.payer])?;

        Ok(signature)
    }

    pub async fn batch_airdrop_to_recipients(
        &self,
        token_mint: &str,
        recipients: Vec<Pubkey>,
        amount: u64,
    ) -> Result<String> {
        let mint_pubkey = Pubkey::from_str(token_mint)
            .with_context(|| format!("Invalid token mint address: {}", token_mint))?;

        let mut instructions = vec![];

        for recipient in recipients {
            let associated_token_address = get_associated_token_address(&recipient, &mint_pubkey);

            let account_info = self
                .client
                .rpc_client
                .get_account(&associated_token_address);

            if account_info.is_err() {
                instructions.push(ata_instruction::create_associated_token_account(
                    &self.client.payer.pubkey(),
                    &recipient,
                    &mint_pubkey,
                    &spl_token::id(),
                ));
            }

            instructions.push(token_instruction::mint_to(
                &spl_token::id(),
                &mint_pubkey,
                &associated_token_address,
                &self.client.payer.pubkey(),
                &[],
                amount,
            )?);
        }

        let signature = self
            .client
            .send_transaction_with_signer(instructions, &[&self.client.payer])?;

        println!("Batch airdrop completed successfully!");
        println!("Transaction signature: {}", signature);

        Ok(signature)
    }

    pub async fn check_balance(&self, token_mint: &str, address: &str) -> Result<u64> {
        let mint_pubkey = Pubkey::from_str(token_mint)
            .with_context(|| format!("Invalid token mint address: {}", token_mint))?;

        let address_pubkey = Pubkey::from_str(address)
            .with_context(|| format!("Invalid wallet address: {}", address))?;

        let associated_token_address = get_associated_token_address(&address_pubkey, &mint_pubkey);

        match self
            .client
            .rpc_client
            .get_token_account_balance(&associated_token_address)
        {
            Ok(balance) => {
                let amount = balance
                    .amount
                    .parse::<u64>()
                    .with_context(|| "Failed to parse token balance")?;

                println!("Token Balance for address {}: {}", address, amount);
                println!("Associated Token Account: {}", associated_token_address);

                Ok(amount)
            }
            Err(_) => {
                println!(
                    "No token account found for address {} with mint {}",
                    address, token_mint
                );
                Ok(0)
            }
        }
    }
}
