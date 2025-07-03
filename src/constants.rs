use solana_sdk::commitment_config::CommitmentConfig;

pub const DEVNET_RPC_URL: &str = "https://broken-cosmopolitan-sponge.solana-devnet.quiknode.pro/3ccb081aac010fb6251a9f2c5dc820c8913724b8/";
pub const DEFAULT_KEYPAIR_PATH: &str = "~/.config/solana/id.json";

pub const DEFAULT_COMMITMENT: CommitmentConfig = CommitmentConfig::confirmed();
