#![allow(deprecated)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use crate::state::{Vault,User};

#[derive(Accounts)]
pub struct Deposit<'info>{
    //user metadata pda
    #[account(
        mut,
        seeds=[b"user",owner.key().as_ref()],
        bump = user.bump
    )]
    pub user: Account<'info,User>,
    //Global vault pda-holds the liquidity
    #[account(
        mut,
        seeds= [b"vault"],
        bump = vault.bump
    )]
    pub vault: Account<'info,Vault>,
    //user paying/depositing
    pub owner: Signer<'info>,
    //user token account for wsol,jitosol etc
    #[account(mut)]
    pub user_token_account: Account<'info,TokenAccount>,
}


