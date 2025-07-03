use crate::constants::*;
use crate::utils::custom_io_tools::{load_keypair, load_keypair_from_path};
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

pub struct SolanaClient {
    pub rpc_client: RpcClient,
    pub payer: Keypair,
}

impl SolanaClient {
    pub fn new() -> Result<Self> {
        let rpc_client =
            RpcClient::new_with_commitment(DEVNET_RPC_URL.to_string(), DEFAULT_COMMITMENT);

        let payer = load_keypair(DEFAULT_KEYPAIR_PATH)?;

        Ok(Self { rpc_client, payer })
    }
    pub fn new_with_keypair_path(keypair_path: &str) -> Result<Self> {
        let rpc_client = RpcClient::new_with_commitment(DEVNET_RPC_URL, DEFAULT_COMMITMENT);

        let payer = load_keypair_from_path(keypair_path)?;

        Ok(Self { rpc_client, payer })
    }

    pub fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }
}
