use {
    crate::errors::ProgramError,
    crate::events::*,
    crate::utils::*,
    anchor_lang::{
        prelude::*,
        solana_program::native_token::LAMPORTS_PER_SOL,
        AnchorDeserialize, AnchorSerialize,
    },
};

#[account]
pub struct User {
    authority: Pubkey,
    balance: u64,
    referred_balance: u64,
    is_admin: bool,
    is_ambassador: bool,
    my_ambassador_quota: u64,
    payout: i64,
    bump: u8,
}

impl User {
	pub const MAXIMUM_SIZE: usize = 32 + 8 + 8 + 1 + 1 + 8 + 8 + 1;
}

#[account]
pub struct Skwizz {
    name: String,
    symbol: String,
    decimals: u8,
    dividend_fee: u8,
    token_initial_price: u64,
    token_incremental_price: u64,
    token_supply: u64,
    magnitude: u64,
    staking_requirement: u64,
    ambassador_max_purchase: u64,
    ambassador_quota: u64,
    profit_per_share: u64,
    only_ambassadors: bool,
    is_initialized: bool,
    bump: u8,
}

// Modifiers helper functions
impl Skwizz {}

// Private functions
impl Skwizz {
    fn purchase_tokens(
        &mut self,
        user: &Signer,
        user_data: &mut Account<User>,
        lamports: u64,
        referred_by: Option<Pubkey>,
        referred_by_data: &mut Account<User>,
    ) -> Result<u64> {
        // data setup
        let customer_address = user.key();
        let undivided_dividends = lamports / self.dividend_fee as u64;
        let referral_bonus = undivided_dividends / 3;
        let mut dividends = undivided_dividends - referral_bonus;
        let taxed_lamport = lamports - undivided_dividends;
        let amount_of_tokens = self.lamport_to_tokens(taxed_lamport);
        let mut fee = dividends * self.magnitude;
        let referred = referred_by.unwrap_or_default();

        // no point in continuing execution if OP is a poorfag russian hacker
        // prevents overflow in the case that the pyramid somehow magically starts being used by everyone in the world
        // (or hackers)
        // and yes we know that the safemath function automatically rules out the "greater then" equasion.
        require_gt!(amount_of_tokens, 0);
        require_gt!(amount_of_tokens + self.token_supply, self.token_supply);

        // is the user referred by a masternode?
        if
        // is this a referred purchase?
        referred != Pubkey::default() &&
            // no cheating!
            referred != customer_address &&
            // does the referrer have at least X whole tokens?
            // i.e is the referrer a godly chad masternode
			referred_by_data.balance >= self.staking_requirement
        {
            // wealth redistribution
            referred_by_data.referred_balance += referral_bonus;

            // Emit an event
            on_masternode(referred, customer_address, lamports, referral_bonus);
        } else {
            // no ref purchase
            // add the referral bonus back to the global dividends cake
            dividends += referral_bonus;
            fee = dividends * self.magnitude;
        }

        // we can't give people infinite ethereum
        if self.token_supply > 0 {
            // add tokens to the pool
            self.token_supply += amount_of_tokens;

            // take the amount of dividends gained through this transaction, and allocates them evenly to each shareholder
            self.profit_per_share += (dividends * self.magnitude) / (self.token_supply);

            // calculate the amount of tokens the customer receives over his purchase
            fee = fee
                - (fee - (amount_of_tokens * ((dividends * self.magnitude) / (self.token_supply))));
        } else {
            // add tokens to the pool
            self.token_supply = amount_of_tokens;
        }

        // update circulating supply & the ledger address for the customer
        user_data.balance += amount_of_tokens;

        // Tells the contract that the buyer doesn't deserve dividends for the tokens before they owned them;
        //really i know you think you do but you don't
        let updated_payouts = ((self.profit_per_share * amount_of_tokens) - fee) as i64;

        user_data.payout += updated_payouts;

        // fire event
        on_token_purchase(customer_address, lamports, amount_of_tokens, referred);

        Ok(amount_of_tokens)
    }

    /**
     * Calculate Token price based on an amount of incoming lamport
     * It's an algorithm, hopefully we gave you the whitepaper with it in scientific notation;
     * Some conversions occurred to prevent decimal errors or underflows / overflows in solidity code.
     */
    fn lamport_to_tokens(&mut self, lamport: u64) -> u64 {
        let token_initial_price = self.token_initial_price * LAMPORTS_PER_SOL;
        let tokens_to_receive = ((
            // underflow attempts BTFO
            sqrt(
                (token_initial_price.pow(2))
                    + (2 * (self.token_incremental_price * LAMPORTS_PER_SOL)
                        * (lamport * LAMPORTS_PER_SOL))
                    + ((self.token_incremental_price).pow(2) * (self.token_supply.pow(2)))
                    + (2 * (self.token_incremental_price)
                        * token_initial_price
                        * self.token_supply),
            ) - token_initial_price
        ) / (self.token_incremental_price))
            - (self.token_supply);

        tokens_to_receive
    }

    /**
     * Calculate token sell value.
     * It's an algorithm, hopefully we gave you the whitepaper with it in scientific notation;
     * Some conversions occurred to prevent decimal errors or underflows / overflows in solidity code.
     */
    fn tokens_to_lamport(&mut self, tokens: u64) -> u64 {
        let tokens = tokens + LAMPORTS_PER_SOL;
        let token_supply = self.token_supply + LAMPORTS_PER_SOL;

        let lamport_to_receive = 
            // underflow attempts BTFO
            ((((self.token_initial_price
                + (self.token_incremental_price * (token_supply / LAMPORTS_PER_SOL)))
                - self.token_incremental_price)
                * (tokens - LAMPORTS_PER_SOL))
                - (self.token_incremental_price * ((tokens.pow(2) - tokens) / LAMPORTS_PER_SOL)) / 2)
                / LAMPORTS_PER_SOL;
        
		lamport_to_receive
    }
}

// Public functions
impl Skwizz {
    pub const MAXIMUM_SIZE: usize = 1 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 1 + 20; //  20 bytes for token name and symbol

    // initialize the state
    pub fn initialize(
        &mut self,
        admin_account: &Signer,
        // program_id: &Pubkey,
        admin_data_account: &mut Account<User>,
        // ambassadors_accounts: Vec<&Account<User>>,
        // ambassadors: Vec<&Pubkey>,
    ) -> Result<()> {
		require_eq!(self.is_initialized, false, ProgramError::AlreadyInitialized);
        self.name = String::from("Skwizz");
        self.symbol = String::from("SKWIZZ");
        self.decimals = 9;
        self.dividend_fee = 10;
        self.token_initial_price = 100_000;
        self.token_incremental_price = 100_000;
        self.token_supply = 0;
        self.magnitude = u64::pow(2,16);
        self.staking_requirement = 2_000_000_000_000_000_000;
        self.ambassador_max_purchase = LAMPORTS_PER_SOL;
        self.ambassador_quota = LAMPORTS_PER_SOL;
        self.profit_per_share = 0;
        self.only_ambassadors = true;
        self.is_initialized = true;

        admin_data_account.authority = admin_account.key();
        admin_data_account.balance = 0;
        admin_data_account.is_admin = true;
        admin_data_account.is_ambassador = true;

        // for (index, user_key) in ambassadors.iter().enumerate() {
        //     let user = ambassadors_accounts[index];
        //     let (pda, _bump) = Pubkey::find_program_address(
        //         &["users".as_bytes(), &user_key.as_ref()],
        //         &program_id,
        //     );
        //     require_keys_eq!(user.key(), pda.key());
        //     if user.get_lamports() == 0 {};

        //     let mut user = User::try_deserialize(&pda)?;
        //     let user = user.authority = user.key();
        //     user.balance = 0;
        //     user.is_admin = true;
        //     user.is_ambassador = true;
        // }

        Ok(())
    }

    /**
     * Converts all incoming lamports to tokens for the signer, and passes down the referral addy (if any)
     */
    pub fn buy(
        &mut self,
        referred_by: Option<Pubkey>,
        user_account: &Signer,
        user_data_account: &mut Account<User>,
        lamports: u64,
        referred_by_data_account: &mut Account<User>,
    ) -> Result<u64> {
        self.purchase_tokens(user_account, user_data_account, lamports, referred_by, referred_by_data_account)
    }
}
