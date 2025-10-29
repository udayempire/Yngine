#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;
use crate::instructions::*;

pub mod instructions;
pub mod state;

declare_id!("3kLpfgTzRrMDANpQQ9gFnENqvFFQGyU7apooz426YWty");

#[program]
pub mod yngine {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
