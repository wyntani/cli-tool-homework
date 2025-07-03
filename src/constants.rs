use solana_sdk::commitment_config::CommitmentConfig;

pub const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";
pub const DEFAULT_KEYPAIR_PATH: &str = "~/.config/solana/id.json";

pub const DEFAULT_COMMITMENT: CommitmentConfig = CommitmentConfig::confirmed();
