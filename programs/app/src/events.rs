use anchor_lang::prelude::*;

/*==============================
=            EVENTS            =
==============================*/
pub fn on_token_purchase(
    customer_address: Pubkey,
    incoming_eth: u64,
    tokens_minted: u64,
    referred_by: Pubkey,
) -> bool {
    // Emit a message for TokenPurchase event
    msg!(
        "TokenPurchase: Customer={}, ETH={}, Tokens={}, ReferredBy={}",
        customer_address,
        incoming_eth,
        tokens_minted,
        referred_by
    );

    true
}

pub fn on_token_sell(customer_address: Pubkey, tokens_burned: u64, eth_earned: u64) -> bool {
    // Emit a message for TokenSell event
    msg!(
        "TokenSell: Customer={}, Tokens={}, ETH={}",
        customer_address,
        tokens_burned,
        eth_earned
    );

    true
}

pub fn on_reinvestment(customer_address: Pubkey, eth_reinvested: u64, tokens_minted: u64) -> bool {
    // Emit a message for Reinvestment event
    msg!(
        "Reinvestment: Customer={}, ETH={}, Tokens={}",
        customer_address,
        eth_reinvested,
        tokens_minted
    );

    true
}

pub fn on_withdraw(customer_address: Pubkey, eth_withdrawn: u64) -> bool {
    // Emit a message for Withdraw event
    msg!(
        "Withdraw: Customer={}, ETH={}",
        customer_address,
        eth_withdrawn
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
        "Masternode: Customer={}, Consumer={}, ETH={}, Bonus={}",
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
