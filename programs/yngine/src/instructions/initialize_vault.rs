#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::state::{Vault};

#[derive(Accounts)]
pub struct InitializeVault<'info>{
    #[account(
        init,
        payer= authority,
        space= Vault::SIZE,
        seeds=[b"vault"], 
        bump
    )]
    pub vault: Account<'info,Vault>,
    #[account(mut)]
    pub authority:Signer<'info>,
    pub system_program: Program<'info,System>
}

pub fn initialize_vault(
    ctx:Context<InitializeVault>,
    active_provider:Pubkey,
)->Result<()>{
    let vault = &mut ctx.accounts.vault;
    vault.authority = ctx.accounts.authority.key();
    vault.active_provider = active_provider;
    vault.created_at = Clock::get()?.unix_timestamp;
    vault.bump= ctx.bumps.vault;
    Ok(())
}