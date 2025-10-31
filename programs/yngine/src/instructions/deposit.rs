#![allow(deprecated)]
#![allow(unexpected_cfgs)]

use std::task::Context;

use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount,Mint};
use anchor_spl::associated_token::AssocaitedToken;
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
    //vault token account for recieving user's funds
    pub vault_token_account: Account<'info,TokenAccount>,
    //ynSOL which is minted to User
    #[account(mut)]
    pub ynsol_mint: Account<'info,Mint>,
    //ata account user recieves ynsol
    pub user_ynsol_ata: Account<'info,TokenAccount>,
    //token program
    pub token_program: Program<'info,Token>,

    pub system_program: Program<'info,System>,
    pub associated_token_program: Program<'info,AssocaitedToken>,
    pub rent: Sysvar<'info,Rent>
}
pub fn deposit(
    ctx:Context<Deposit>,

)->Result<()>{

}


