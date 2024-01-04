pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

use {
    crate::instructions::*,
    anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize},
};

declare_id!("9eAsSSyUEkPwYcTAWpf9MSYboarjv1J3vKWz8q9q9VSe");

#[program]
pub mod app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        _initialize(ctx)
    }

    pub fn buy(
        ctx: Context<Buy>,
        lamports_to_send: u64,
        referred_by: Option<Pubkey>,
    ) -> Result<()> {
        _buy(ctx, lamports_to_send, referred_by)
    }

    pub fn reinvest(ctx: Context<Reinvest>) -> Result<()> {
        _reinvest(ctx)
    }

    pub fn exit(ctx: Context<Exit>) -> Result<()> {
        _exit(ctx)
    }

    pub fn transfer(ctx: Context<Transfer>, to: Pubkey, lamports_to_send: u64) -> Result<()> {
        _transfer(ctx, to, lamports_to_send)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        _withdraw(ctx)
    }

    pub fn sell(ctx: Context<Sell>, lamports_to_send: u64) -> Result<()> {
        _sell(ctx, lamports_to_send)
    }

    // Read only instructions
    pub fn my_dividends(ctx: Context<ReadOnly>, including_ref: bool) -> Result<u64> {
        _my_dividends(ctx, including_ref)
    }

    pub fn sell_price(ctx: Context<ProgramReadOnly>) -> Result<u64> {
        _sell_price(ctx)
    }

    pub fn buy_price(ctx: Context<ProgramReadOnly>) -> Result<u64> {
        _buy_price(ctx)
    }

    pub fn calculate_lamports_received(ctx: Context<ProgramReadOnly>, tokens: u64) -> Result<u64> {
        _calculate_lamports_received(ctx, tokens)
    }

    pub fn calculate_tokens_received(ctx: Context<ProgramReadOnly>, lamports: u64) -> Result<u64> {
        _calculate_tokens_received(ctx, lamports)
    }
}
