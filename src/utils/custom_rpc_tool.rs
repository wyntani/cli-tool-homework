use solana_sdk::pubkey::Pubkey;

pub fn get_associated_token_address(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let owner_bytes = owner.to_bytes();
    let token_program_bytes = spl_token::id().to_bytes();
    let mint_bytes = mint.to_bytes();

    let seeds = &[
        owner_bytes.as_ref(),
        token_program_bytes.as_ref(),
        mint_bytes.as_ref(),
    ];

    Pubkey::find_program_address(seeds, &spl_token::id()).0
}
