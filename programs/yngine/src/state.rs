use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub ynsol_mint: Pubkey, 
    pub sol_balance: u64,
    pub active_provider:Pubkey,
    pub created_at: i64,
    pub bump: u8,
}

impl Vault {
    pub const SIZE: usize = 8+32+32+32+4+32+4+1;
}

#[account]
pub struct User {
    pub owner: Pubkey,
    pub deposited_sol:u64,
    pub withdrawn_sol: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl User {
    pub const SIZE: usize = 8+32+4+4+4+1;
}