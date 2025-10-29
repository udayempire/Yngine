use anchor_lang::prelude::*;
#[account]

pub struct Vault {
    pub owner: Pubkey,
    pub ynsol_mint: Pubkey, 
    pub sol_balance: u64,
    pub bump: u8,
    pub active_provider: u16,
}
pub struct User {
    pub owner: Pubkey,
    pub vault: Pubkey, //user's vault address
    pub deposited_sol:Pubkey,
    pub withdrawn_sol: Pubkey,
    pub bump: u8,
}