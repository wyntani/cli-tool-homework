use crate::constants::*;
use anyhow::Result;
use anyhow::{Context, bail};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::fs;
use std::path::Path;

pub struct SolanaClient {
    pub rpc_client: RpcClient,
    pub payer: Keypair,
}

impl SolanaClient {
    pub fn new() -> Result<Self> {
        let rpc_client =
            RpcClient::new_with_commitment(DEVNET_RPC_URL.to_string(), DEFAULT_COMMITMENT);

        let payer = Self::load_keypair()?;

        Ok(Self { rpc_client, payer })
    }

    pub fn new_with_keypair_path(keypair_path: &str) -> Result<Self> {
        let rpc_client = RpcClient::new_with_commitment(DEVNET_RPC_URL, DEFAULT_COMMITMENT);

        let payer = Self::load_keypair_from_path(keypair_path)?;

        Ok(Self { rpc_client, payer })
    }

    fn load_keypair() -> Result<Keypair> {
        let keypair_path = shellexpand::tilde(DEFAULT_KEYPAIR_PATH);
        Self::load_keypair_from_path(&keypair_path)
    }

    fn load_keypair_from_path(path: &str) -> Result<Keypair> {
        if !Path::new(path).exists() {
            bail!(
                "Keypair file not found at: {}. Please run 'solana-keygen new' to create a keypair",
                path
            );
        }

        let keypair_data = fs::read_to_string(path)
            .with_context(|| format!("Failed to read keypair file: {}", path))?;

        let keypair_bytes: Vec<u8> = serde_json::from_str(&keypair_data)
            .with_context(|| "Failed to parse keypair file as JSON")?;

        if keypair_bytes.len() != 64 {
            bail!(
                "Invalid keypair file format: expected 64 bytes, got {}",
                keypair_bytes.len()
            );
        }

        Keypair::try_from(&keypair_bytes[..]).with_context(|| "Failed to create keypair from bytes")
    }

    pub fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }
}
