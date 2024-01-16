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

declare_id!("8yCmCyhDsLzof54Pbuj2dBxAKSyLosdFcR7Aov894NJk");

#[program]
pub mod app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, metadata: InitTokenParams) -> Result<()> {
        _initialize(ctx, metadata)
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

    // Admin instructions
    pub fn disable_initial_stage(ctx: Context<Admin>) -> Result<()> {
        _disable_initial_stage(ctx)
    }

    pub fn set_administrator(ctx: Context<AdminSetter>, user: Pubkey, status: bool) -> Result<()> {
        _set_administrator(ctx, user, status)
    }

    pub fn set_ambassador(ctx: Context<AdminSetter>, user: Pubkey, status: bool) -> Result<()> {
        _set_ambassador(ctx, user, status)
    }

    pub fn set_staking_requirement(ctx: Context<Admin>, amount_of_tokens: u64) -> Result<()> {
        _set_staking_requirement(ctx, amount_of_tokens)
    }

    pub fn set_name(ctx: Context<Admin>, name: String) -> Result<()> {
        _set_name(ctx, name)
    }

    pub fn set_symbol(ctx: Context<Admin>, symbol: String) -> Result<()> {
        _set_symbol(ctx, symbol)
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
