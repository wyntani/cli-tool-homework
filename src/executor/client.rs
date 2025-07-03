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

    pub fn send_transaction_with_signers(
        &self,
        instructions: Vec<solana_sdk::instruction::Instruction>,
        signers: &[&dyn Signer],
    ) -> Result<String> {
        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &instructions,
            Some(&self.payer.pubkey()),
            signers,
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        Ok(signature.to_string())
    }
}
