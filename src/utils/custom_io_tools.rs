use anyhow::{Result, bail};
use solana_sdk::pubkey::Pubkey;
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
