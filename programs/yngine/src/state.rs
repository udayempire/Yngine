use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub owner: Pubkey,
    pub ynsol_mint: Pubkey, 
    pub sol_balance: u64,
    pub active_provider:Vec<Pubkey>,
    pub created_at: i64,
    pub bump: u8,
}

impl Vault {
    pub const SIZE: usize = 8+32+32+32+4+2+1+1;
}

#[account]
pub struct User {
    pub owner: Pubkey,
    pub vault: Pubkey, //user's vault address
    pub deposited_sol:Pubkey,
    pub withdrawn_sol: Pubkey,
    pub created_at: u8,
    pub bump: u8,
}