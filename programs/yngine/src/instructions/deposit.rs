#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::state::{Vault}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer= owner,
        space= Vault::SIZE
        seeds=[b"vault",creator.key().as_ref()], //pda seeds
        bump
    )]
};