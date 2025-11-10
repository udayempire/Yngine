#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::{User, Vault};

pub fn withdraw()->Result<()>{
    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(
        mut,
        seeds=[b"user",owner.key().as_ref()],
        bump= user.bump
    )]
    pub user: Account<'info,User>, //tracks balances and validates owner
    #[account(
        mut,
        seeds=[b"vault"],
        bump = vault.bump
    )]
    pub vault: Account<'info,Vault>,// holds assests and authority to mint/burn
    pub owner: Signer<'info>, //user withrawing
    pub ynsol_mint: Account<'info,Mint>,
    #[account(
        mut,
        associated_token::mint = ynsol_mint,
        associated_token::authority = owner
    )]
    pub user_ynsol_ata: Account<'info,TokenAccount>, //where ynSOL gets burned FROM
    #[account(
        mut,
        token::authority = vault
    )]
    pub vault_token_account: Account<'info,TokenAccount>,
    #[account(
        mut
    )]
    pub user_token_account: Account<'info,TokenAccount>,
    pub token_program: Program<'info,Token>,
}