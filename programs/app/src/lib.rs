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

declare_id!("DiqPEso6EqoX62T9j9peBdWQYdPb86tzGE7yZ2rR6d5B");

#[program]
pub mod app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, metadata: InitTokenParams) -> Result<()> {
        _initialize(ctx, metadata)
    }

    pub fn buy(
        ctx: Context<Buy>,
        lamports_to_send: u128,
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

    pub fn transfer(ctx: Context<Transfer>, to: Pubkey, lamports_to_send: u128) -> Result<()> {
        _transfer(ctx, to, lamports_to_send)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        _withdraw(ctx)
    }

    pub fn sell(ctx: Context<Sell>, lamports_to_send: u128) -> Result<()> {
        _sell(ctx, lamports_to_send)
    }

    // Admin instructions
    pub fn disable_initial_stage(ctx: Context<Admin>) -> Result<()> {
        _disable_initial_stage(ctx)
    }

    pub fn distribute_token(
        ctx: Context<DistributeToken>,
        amount_of_tokens: u128,
        update_payout_by: i128,
        receipient: Pubkey,
    ) -> Result<()> {
        let clock: Clock = Clock::get()?;
        let a_day = 86400;
        let s_timestamp: i64 = clock.unix_timestamp;
        let e_timestamp: i64 = s_timestamp + (a_day * 100);
        _distribute_token(
            ctx,
            amount_of_tokens,
            update_payout_by,
            receipient,
            s_timestamp,
            e_timestamp,
        )
    }

    pub fn set_administrator(ctx: Context<AdminSetter>, user: Pubkey, status: bool) -> Result<()> {
        _set_administrator(ctx, user, status)
    }

    pub fn set_ambassador(ctx: Context<AdminSetter>, user: Pubkey, status: bool) -> Result<()> {
        _set_ambassador(ctx, user, status)
    }

    pub fn set_staking_requirement(ctx: Context<Admin>, amount_of_tokens: u128) -> Result<()> {
        _set_staking_requirement(ctx, amount_of_tokens)
    }

    // Read only instructions
    pub fn my_dividends(ctx: Context<ReadOnly>, including_ref: bool) -> Result<u128> {
        _my_dividends(ctx, including_ref)
    }

    pub fn sell_price(ctx: Context<ProgramReadOnly>) -> Result<u128> {
        _sell_price(ctx)
    }

    pub fn buy_price(ctx: Context<ProgramReadOnly>) -> Result<u128> {
        _buy_price(ctx)
    }

    pub fn calculate_lamports_received(
        ctx: Context<ProgramReadOnly>,
        tokens: u128,
    ) -> Result<u128> {
        _calculate_lamports_received(ctx, tokens)
    }

    pub fn calculate_tokens_received(
        ctx: Context<ProgramReadOnly>,
        lamports: u128,
    ) -> Result<u128> {
        _calculate_tokens_received(ctx, lamports)
    }
}
