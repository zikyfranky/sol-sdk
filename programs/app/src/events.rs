use anchor_lang::prelude::*;

/*==============================
=            EVENTS            =
==============================*/
pub fn on_token_purchase(
    customer_address: Pubkey,
    incoming_lamports: u64,
    tokens_minted: u64,
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

pub fn on_token_sell(customer_address: Pubkey, tokens_burned: u64, lamports_earned: u64) -> bool {
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
    lamports_reinvested: u64,
    tokens_minted: u64,
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

pub fn on_withdraw(customer_address: Pubkey, lamports_withdrawn: u64) -> bool {
    // Emit a message for Withdraw event
    msg!(
        "Withdraw: Customer={}, Lamports={}",
        customer_address,
        lamports_withdrawn
    );

    true
}

pub fn on_masternode(
    customer_address: Pubkey,
    consumer_address: Pubkey,
    lamport_used: u64,
    bonus_received: u64,
) -> bool {
    // Emit a message for Masternode event
    msg!(
        "Masternode: Customer={}, Consumer={}, Lamports={}, Bonus={}",
        customer_address,
        consumer_address,
        lamport_used,
        bonus_received
    );

    true
}

pub fn on_transfer(from: Pubkey, to: Pubkey, tokens: u64) -> bool {
    // Emit a message for Transfer event
    msg!("Transfer: From={}, To={}, Tokens={}", from, to, tokens);

    true
}
