#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::state::{Vault};

#[derive(Accounts)]
pub struct InitializeVault<'info>{
    #[account(
        init,
        payer= owner,
        space= Vault::SIZE,
        seeds=[b"vault",owner.key().as_ref()], //pda seeds
        bump
    )]
    pub vault: Account<'info,Vault>,
    #[account(mut)]
    pub owner:Signer<'info>,
    pub system_program: Program<'info,System>
}

pub fn initialize_vault(
    ctx:Context<InitializeVault>,
    ynsol_mint: Pubkey,
    sol_balance: u64,
    active_provider:u16,
    created_at: u8,
)->Result<()>{
    let vault = &mut ctx.accounts.vault;
    vault.owner = ctx.accounts.owner.key();
    vault.ynsol_mint = ynsol_mint;
    vault.sol_balance = sol_balance;
    vault.active_provider = active_provider;
    vault.created_at = created_at;
    vault.bump= ctx.bumps.vault;
    Ok(())
}