use anyhow::{Context, Result, bail};
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::path::Path;
use std::str::FromStr;

pub fn load_recipients(file_path: &Path) -> Result<Vec<Pubkey>> {
    if !file_path.exists() {
        bail!("Recipients file not found: {}", file_path.display());
    }

    let content = std::fs::read_to_string(file_path)?;
    let recipients: Vec<Pubkey> = content
        .lines()
        .enumerate()
        .filter_map(|(line_num, line)| {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                return None;
            }

            match Pubkey::from_str(line) {
                Ok(pubkey) => Some(pubkey),
                Err(_) => {
                    eprintln!(
                        "Warning: Invalid public key at line {}: {}",
                        line_num + 1,
                        line
                    );
                    None
                }
            }
        })
        .collect();

    if recipients.is_empty() {
        bail!("No valid recipients found in file: {}", file_path.display());
    }

    Ok(recipients)
}

pub fn load_keypair_from_path(path: &str) -> Result<Keypair> {
    if !Path::new(path).exists() {
        bail!(
            "Keypair file not found at: {}. Please run 'solana-keygen new' to create a keypair",
            path
        );
    }

    let keypair_data = std::fs::read_to_string(path)
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

pub fn load_keypair(default_keypair_path: &str) -> Result<Keypair> {
    let keypair_path = shellexpand::tilde(default_keypair_path);
    load_keypair_from_path(&keypair_path)
}
