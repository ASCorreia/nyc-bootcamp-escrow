use anchor_lang::prelude::*;

pub mod contexts;
use contexts::*;

pub mod state;
pub use state::*;

declare_id!("HQon1LDGidqzgLCr9QAkEqjnvvkpCxnspB1vf9UpJbub");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    // pub fn take(ctx: Context<Take>) -> Result<()> {
    //     ctx.accounts.deposit()?;
    //     ctx.accounts.withdraw_and_close_vault()
    // }
}
