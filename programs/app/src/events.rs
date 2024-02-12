use anchor_lang::prelude::*;

/*==============================
=            EVENTS            =
==============================*/
pub fn on_token_purchase(
    customer_address: Pubkey,
    incoming_lamports: u128,
    tokens_minted: u128,
    referred_by: Pubkey,
) -> bool {
    // Emit a message for TokenPurchase event
    msg!(
        "TokenPurchase: Customer={}, Lamports={}, Tokens={}, ReferredBy={}",
        customer_address,
        incoming_lamports,
        tokens_minted,
        referred_by
    );

    true
}

pub fn on_token_sell(customer_address: Pubkey, tokens_burned: u128, lamports_earned: u128) -> bool {
    // Emit a message for TokenSell event
    msg!(
        "TokenSell: Customer={}, Tokens={}, Lamports={}",
        customer_address,
        tokens_burned,
        lamports_earned
    );

    true
}

pub fn on_reinvestment(
    customer_address: Pubkey,
    lamports_reinvested: u128,
    tokens_minted: u128,
) -> bool {
    // Emit a message for Reinvestment event
    msg!(
        "Reinvestment: Customer={}, Lamports={}, Tokens={}",
        customer_address,
        lamports_reinvested,
        tokens_minted
    );

    true
}

pub fn on_withdraw(customer_address: Pubkey, lamports_withdrawn: u128) -> bool {
    // Emit a message for Withdraw event
    msg!(
        "Withdraw: Customer={}, Lamports={}",
        customer_address,
        lamports_withdrawn
    );

    true
}

pub fn on_skwizkey(
    customer_address: Pubkey,
    consumer_address: Pubkey,
    lamport_used: u128,
    bonus_received: u128,
) -> bool {
    // Emit a message for Skwizkey event
    msg!(
        "Skwizkey: Customer={}, Consumer={}, Lamports={}, Bonus={}",
        customer_address,
        consumer_address,
        lamport_used,
        bonus_received
    );

    true
}

pub fn on_transfer(from: Pubkey, to: Pubkey, tokens: u128) -> bool {
    // Emit a message for Transfer event
    msg!("Transfer: From={}, To={}, Tokens={}", from, to, tokens);

    true
}
