#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount, Transfer};
use crate::state::{User, Vault};

pub fn withdraw(
    ctx: Context<Withdraw>,
    amount: u64
)->Result<()>{
    let user = &mut ctx.accounts.user;
    let vault = &mut ctx.accounts.vault;
    //burn ynsol from user_ynsol_ata
    let cpi_program = &ctx.accounts.token_program;
    let cpi_accounts = Burn{
        from: ctx.accounts.user_ynsol_ata.to_account_info(),
        mint: ctx.accounts.ynsol_mint.to_account_info(),
        authority: ctx.accounts.owner.to_account_info()
    };
    token::burn(
        CpiContext::new(
            cpi_program.to_account_info(),
            cpi_accounts
        ),
        amount
    )?;
    // transfer sol from vault to user account
    let cpi_transfer_accounts = Transfer{
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let seeds: &[&[u8]] = &[b"vault",&[vault.bump]];
    let signer_seeds = &[seeds];
    token::transfer(
        CpiContext::new_with_signer(
            cpi_program.to_account_info(), 
            cpi_transfer_accounts,signer_seeds), 
            amount
        )?;
    // updating balances
    vault.vault_balance -= amount;
    vault.total_shares -= amount;
    user.total_deposited -= amount;
    user.total_withdrawn += amount;
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
    pub authority: Signer<'info>,
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