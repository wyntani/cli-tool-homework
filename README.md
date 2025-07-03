# SPL Airdropper CLI

A command-line tool for creating SPL tokens and distributing them to multiple recipients on Solana.

## Setup

### Prerequisites
- Rust (latest stable)
- Solana CLI with configured keypair
- SOL balance for transaction fees

### Installation
```bash
git clone <repository-url>
cd cli-tool-homework
cargo build --release
```

Binary location: `target/release/spl_airdropper`

## Configuration

Uses Solana Devnet (`https://api.devnet.solana.com`) and default keypair (`~/.config/solana/id.json`).
E4NysZadugnzGW6Ze2uhproTRP4iT5vPM8BFfGboh3Ty
## Commands

### Create Token
```bash
cargo run --release create-token --decimals 9
```
Creates a new SPL token with specified decimal places.

### Airdrop (Individual Transactions)
```bash
cargo run --release airdrop --token-mint <MINT_ADDRESS> --amount 1000 --recipients recipients.txt
```
Sends tokens to each recipient in separate transactions.

### Batch Airdrop (Single Transaction)
```bash
cargo run --release batch-airdrop --token-mint <MINT_ADDRESS> --amount 1000 --recipients recipients.txt
```
Sends tokens to all recipients in one transaction.

### Check Balance
```bash
cargo run --release check-balance --token-mint <MINT_ADDRESS> --address <WALLET_ADDRESS>
```
Queries token balance for a wallet address.

## Recipients File Format

One public key per line:
```
9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM
2xNweLHLqrbx4zo1waDvgWJHgsUpPj8Y8icbAFeR4a8i
5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1
```

Comments (`#`) and empty lines are ignored.

## Example Usage

1. Create token:
```bash
cargo run --release create-token --decimals 6
# Returns: Token Mint Address: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
```

2. Prepare recipients file:
```bash
echo "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM" > recipients.txt
```

3. Distribute tokens:
```bash
cargo run --release airdrop --token-mint 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU --amount 1000000 --recipients recipients.txt
```

4. Verify balance:
```bash
cargo run --release check-balance --token-mint 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU --address 9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM
```

## Architecture

- `src/cli/` - Command-line interface definitions
- `src/executor/` - Solana RPC client and token operations
- `src/utils/` - File I/O and helper utilities
- `src/main.rs` - Application entry point

## Dependencies

- **clap** - CLI argument parsing
- **solana-client** - Solana RPC operations
- **spl-token** - SPL token program bindings
- **tokio** - Async runtime
- **anyhow** - Error handling