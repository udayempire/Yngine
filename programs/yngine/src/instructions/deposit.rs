#![allow(deprecated)]
#![allow(unexpected_cfgs)]

use crate::state::{User, Vault};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount, Transfer};

//move tokens from user_token_account to vault_token_account
//mint ynSOL to user_ynsol_ata
//update user+vault state

#[derive(Accounts)]
pub struct Deposit<'info> {
    //user metadata pda
    #[account(
        mut,
        seeds=[b"user",owner.key().as_ref()],
        bump = user.bump
    )]
    pub user: Account<'info, User>,
    //Global vault pda-holds the liquidity
    #[account(
        mut,
        seeds= [b"vault"],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    //user paying/depositing
    pub owner: Signer<'info>,
    //user token account for wsol,jitosol etc
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    //vault token account for recieving user's funds
    #[account(
        mut,
        // token::mint = ynsol_mint,
        token::authority = vault
    )]
    pub vault_token_account: Account<'info, TokenAccount>,
    //ynSOL which is minted to User
    #[account(mut)]
    pub ynsol_mint: Account<'info, Mint>,
    //ata account user recieves ynsol
    #[account(
        mut,
        associated_token::mint = ynsol_mint,
        associated_token::authority = owner
    )]
    pub user_ynsol_ata: Account<'info, TokenAccount>,
    //token program
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
pub fn deposit(
    ctx: Context<Deposit>, 
    amount: u64,
) -> Result<()> {
    let user = &mut ctx.accounts.user; // user
    // let amount = ctx.accounts.user_token_account.amount; // user token account's amount( no of tokens user has sent)
    // cpi for transfer funds from user wallet to vault
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
    // creating pda seeds
    let bump = ctx.accounts.vault.bump;
    let seeds: &[&[u8]] = &[b"vault", &[bump]];
    let signer = &[&seeds[..]];
    // mint_to for ynsol
    let mint_cpi_accounts = MintTo {
        mint: ctx.accounts.ynsol_mint.to_account_info(),
        to: ctx.accounts.user_ynsol_ata.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            mint_cpi_accounts,
            signer,
        ),
        amount,
    )?;
    //update vault_state fields
    let vault = &mut ctx.accounts.vault;
    vault.total_shares += amount;
    vault.vault_balance += amount;
    user.total_deposited += amount;
    Ok(())
}
