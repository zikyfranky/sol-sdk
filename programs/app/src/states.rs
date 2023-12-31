use {
    crate::errors::ProgramError,
    crate::events::*,
    crate::utils::*,
    anchor_lang::{
        prelude::*,
        solana_program::{
            native_token::LAMPORTS_PER_SOL, program::invoke, system_instruction::transfer,
        },
        AnchorDeserialize, AnchorSerialize,
    },
};

#[account]
pub struct User {
    authority: Pubkey,
    balance: u64,
    referred_balance: u64,
    is_admin: bool,
    is_amb: bool,
    ambassador_quota: u64,
    payout: i64,
}

// Helper functions
impl User {
    fn has_admin_rights(&mut self) -> bool {
        self.is_admin
    }

    fn has_ambassador_rights(&mut self) -> bool {
        self.is_amb
    }

    fn has_balance(&mut self) -> bool {
        self.balance > 0
    }

    fn has_balance_upto(&mut self, amount: u64) -> bool {
        self.balance >= amount
    }

    fn increase_balance_by(&mut self, amount: u64) {
        self.balance += amount;
    }

    fn decrease_balance_by(&mut self, amount: u64) {
        self.balance -= amount;
    }

    fn increase_payout_by(&mut self, amount: i64) {
        self.payout += amount;
    }

    fn decrease_payout_by(&mut self, amount: i64) {
        self.payout -= amount;
    }

    fn increase_referred_balance_by(&mut self, amount: u64) {
        self.referred_balance += amount;
    }

    fn decrease_referred_balance_by(&mut self, amount: u64) {
        self.referred_balance -= amount;
    }

    fn increase_ambassador_quota_by(&mut self, amount: u64) {
        self.ambassador_quota += amount;
    }

    // fn decrease_ambassador_quota_by(&mut self, amount: u64) {
    // 	self.ambassador_quota -= amount;
    // }

    fn update_admin_status(&mut self, status: bool) {
        self.is_admin = status;
    }

    fn update_amb_status(&mut self, status: bool) {
        self.is_amb = status;
    }
}

impl User {
    pub const MAXIMUM_SIZE: usize = 32 + 8 + 8 + 1 + 1 + 8 + 8;
}

#[account]
pub struct App {
    name: String,
    symbol: String,
    decimals: u8,
    dividend_fee: u8,
    token_initial_price: u64,
    token_incremental_price: u64,
    contract_balance: u64,
    token_supply: u64,
    magnitude: u64,
    staking_requirement: u64,
    ambassador_max_purchase: u64,
    ambassador_quota: u64,
    profit_per_share: u64,
    only_ambassadors: bool,
    is_initialized: bool,
}

// Modifiers helper functions
impl App {
    // only people with tokens
    fn only_bagholders(&mut self, user: &mut Account<User>) -> Result<()> {
        require!(user.has_balance(), ProgramError::NotABagHolder);
        Ok(())
    }

    // only people with profits
    fn only_stronghands(program: &mut Account<App>, user: &mut Account<User>) -> Result<()> {
        require!(
            App::my_dividends(program, user, true) > 0,
            ProgramError::NoPofit
        );
        Ok(())
    }

    /*
        administrators can:
        -> change the name of the contract
        -> change the name of the token
        -> change the PoS difficulty (How many tokens it costs to hold a masternode, in case it gets crazy high later)
        they CANNOT:
        -> take funds
        -> disable withdrawals
        -> kill the contract
        -> change the price of tokens
    */
    fn check_admin_rights(&mut self, user: &mut Account<User>) -> Result<()> {
        require!(user.has_admin_rights(), ProgramError::NotAnAdmin);
        Ok(())
    }

    fn check_ambassador_rights(&mut self, user: &mut Account<User>) -> Result<()> {
        require!(user.has_ambassador_rights(), ProgramError::NotAnAmbassador);
        Ok(())
    }

    // ensures that the first tokens in the contract will be equally distributed
    // meaning, no divine dump will be ever possible
    // result: healthy longevity.
    fn anti_early_whale(&mut self, user: &mut Account<User>, amount_of_lamport: u64) -> Result<()> {
        // are we still in the vulnerable phase?
        // if so, enact anti early whale protocol
        if self.only_ambassadors
            && ((self.contract_balance + amount_of_lamport) <= self.ambassador_quota)
        {
            // is the customer in the ambassador list?
            self.check_ambassador_rights(user)?;

            // does the customer purchase exceed the max ambassador quota?
            require!(
                user.ambassador_quota + amount_of_lamport <= self.ambassador_max_purchase,
                ProgramError::LimitExceeded
            );

            // updated the accumulated quota
            user.increase_ambassador_quota_by(amount_of_lamport);
        } else {
            // in case the eth count drops low, the ambassador phase won't reinitiate
            if self.only_ambassadors {
                self.only_ambassadors = false;
            }
        }
        Ok(())
    }

    // Check that user has enough funds to use
    fn has_enough(&mut self, user: &mut Account<User>, amount: u64) -> Result<()> {
        require!(
            user.has_balance_upto(amount),
            ProgramError::InsufficientBalance
        );
        Ok(())
    }

    // Check that user owns the user_data_account
    fn owns_account(&mut self, user: &Signer, user_data: &Account<User>) -> Result<()> {
        require_keys_eq!(
            user.key(),
            user_data.authority.key(),
            ProgramError::NotOwner
        );
        Ok(())
    }
}

// Private functions
impl App {
    fn purchase_tokens<'a>(
        program: &mut Account<'a, App>,
        buyer: &mut Signer<'a>,
        system: AccountInfo<'a>,
        buyer_data_account: &mut Account<'_, User>,
        referred_by_data_account: &mut Option<Account<'_, User>>,
        lamports: u64,
        referred_by: Option<Pubkey>,
        direct_buy: bool,
    ) -> Result<u64> {
        program.anti_early_whale(buyer_data_account, lamports)?;
        // data setup
        let buyer_key = buyer.key();
        let undivided_dividends = lamports / program.dividend_fee as u64;
        let referral_bonus = undivided_dividends / 3;
        let mut dividends = undivided_dividends - referral_bonus;
        let taxed_lamport: u64 = lamports - undivided_dividends;
        let amount_of_tokens = program.lamport_to_tokens(taxed_lamport);
        let mut fee = dividends * program.magnitude;
        let referred = referred_by.unwrap_or_default();

        // Lot of checks
        // prevents overflow in the case that the pyramid somehow magically starts being used by everyone in the world
        // (or hackers)
        require_gt!(amount_of_tokens, 0, ProgramError::SentLessToken);
        require_gte!(
            buyer.get_lamports(),
            lamports,
            ProgramError::InsufficientBalance
        );
        require_gt!(
            amount_of_tokens + program.token_supply,
            program.token_supply,
            ProgramError::SentLessToken
        );

        // is the user referred by a masternode?
        if
        // is this a referred purchase?
        !Pubkey::default().eq(&referred) &&
			// no cheating!
			!referred.eq(&buyer_key)
        {
            if let Some(referred_by_data) = referred_by_data_account {
                // does the referrer have at least X whole tokens?
                // i.e is the referrer a godly chad masternode
                if referred_by_data.has_balance_upto(program.staking_requirement) {
                    // wealth redistribution
                    referred_by_data.increase_referred_balance_by(referral_bonus);

                    // Emit an event
                    on_masternode(referred, buyer_key, lamports, referral_bonus);
                } else {
                    // no ref purchase
                    // add the referral bonus back to the global dividends cake
                    dividends += referral_bonus;
                    fee = dividends * program.magnitude;
                }
            } else {
                // no ref purchase
                // add the referral bonus back to the global dividends cake
                dividends += referral_bonus;
                fee = dividends * program.magnitude;
            }
        } else {
            // no ref purchase
            // add the referral bonus back to the global dividends cake
            dividends += referral_bonus;
            fee = dividends * program.magnitude;
        }

        // we can't give people infinite ethereum
        if program.token_supply > 0 {
            // add tokens to the pool
            program.token_supply += amount_of_tokens;

            // take the amount of dividends gained through this transaction, and allocates them evenly to each shareholder
            program.profit_per_share += (dividends * program.magnitude) / (program.token_supply);

            // calculate the amount of tokens the customer receives over his purchase
            fee = fee
                - (fee
                    - (amount_of_tokens
                        * ((dividends * program.magnitude) / (program.token_supply))));
        } else {
            // add tokens to the pool
            program.token_supply = amount_of_tokens;
        }

        // update circulating supply & the ledger address for the customer
        buyer_data_account.increase_balance_by(amount_of_tokens);

        // Tells the contract that the buyer doesn't deserve dividends for the tokens before they owned them;
        //really i know you think you do but you don't
        let updated_payouts =
            ((program.profit_per_share * amount_of_tokens) as i64 - fee as i64) as i64;

        buyer_data_account.increase_payout_by(updated_payouts);

        // fire event
        on_token_purchase(buyer_key, lamports, amount_of_tokens, referred);

        if direct_buy {
            App::transfer_sol_in(buyer, program, system, lamports)?;
        }

        Ok(amount_of_tokens)
    }

    /**
     * Calculate Token price based on an amount of incoming lamport
     * It's an algorithm, hopefully we gave you the whitepaper with it in scientific notation;
     * Some conversions occurred to prevent decimal errors or underflows / overflows in solidity code.
     */
    fn lamport_to_tokens(&mut self, lamport: u64) -> u64 {
        let token_price_initial = self.token_initial_price as u128; // 100
        let token_price_incremental = self.token_incremental_price as u128; // 1000
        let token_supply = self.token_supply as u128; // 0

        let token_price_initial_expanded = token_price_initial * LAMPORTS_PER_SOL as u128; //100 * 1e9  = 100e9
        let token_price_incremental_expanded = token_price_incremental * LAMPORTS_PER_SOL as u128; //1000 * 1e9  = 1000e9
        let lamport_expanded = lamport as u128 * LAMPORTS_PER_SOL as u128; //1e9*1e9 =  1e18

        let token_price_initial_to_power = token_price_initial_expanded.pow(2);
        let token_price_incremental_to_power = token_price_incremental.pow(2);
        let token_supply_to_power = token_supply.pow(2);

        let first = token_price_initial_to_power;
        let second = 2 * token_price_incremental_expanded * lamport_expanded;
        let third = token_price_incremental_to_power * token_supply_to_power;
        let fourth = 2 * token_price_incremental * token_price_initial_expanded * token_supply;

        let square_root = sqrt(first + second + third + fourth);
        let tokens_received = (((square_root - token_price_initial_expanded)
            / token_price_incremental)
            - token_supply) as u64;

        tokens_received
    }

    /**
     * Calculate token sell value.
     * It's an algorithm, hopefully we gave you the whitepaper with it in scientific notation;
     * Some conversions occurred to prevent decimal errors or underflows / overflows in solidity code.
     */
    fn tokens_to_lamport(&mut self, tokens: u64) -> u64 {
        let lam_to_sol = LAMPORTS_PER_SOL as u128;

        let tokens = tokens as u128 + lam_to_sol;
        let token_supply = self.token_supply as u128 + lam_to_sol;

        let token_initial_price = self.token_initial_price as u128;
        let token_incremental_price = self.token_incremental_price as u128;

        let first = (token_initial_price + (token_incremental_price * (token_supply / lam_to_sol)))
            - token_incremental_price;

        let second = tokens - lam_to_sol;
        let third = token_incremental_price * ((tokens.pow(2) - tokens) / lam_to_sol) / 2;

        // underflow attempts BTFO
        let lamport_to_receive = ((first * second) - (third)) / lam_to_sol;

        lamport_to_receive as u64
    }

    /**
     * Retrieve the dividend balance of any single address.
     */
    fn dividends_of(&mut self, user: &Account<User>) -> u64 {
        ((((self.profit_per_share * user.balance) as i64) - user.payout) as u64) / self.magnitude
    }

    fn transfer_sol_out(from: &mut Account<'_, App>, to: &mut Signer, amount: u64) -> Result<bool> {
        require!(
            amount <= from.contract_balance,
            ProgramError::InsufficientBalance
        );
        from.contract_balance -= amount;
        from.sub_lamports(amount)?;
        to.add_lamports(amount)?;
        Ok(true)
    }

    fn transfer_sol_in<'a>(
        from: &mut Signer<'a>,
        to: &mut Account<'a, App>,
        sys: AccountInfo<'a>,
        amount: u64,
    ) -> Result<bool> {
        require!(
            amount <= from.get_lamports(),
            ProgramError::InsufficientBalance
        );
        to.contract_balance += amount;

        let ix = transfer(&from.key(), &to.key(), amount);

        let _from = from.to_account_info();
        let _to = to.to_account_info();

        invoke(&ix, &[_from, _to, sys])?;

        // from.sub_lamports(amount)?;
        // to.add_lamports(amount)?;

        Ok(true)
    }
}

// Admin functions
impl App {
    /**
     * In case the ambassador quota is not met, the administrator can manually disable the ambassador phase.
     */
    pub fn disable_initial_stage(
        program: &mut Account<App>,
        admin: &mut Account<User>,
    ) -> Result<()> {
        program.check_admin_rights(admin)?;

        program.only_ambassadors = false;
        Ok(())
    }

    /**
     * In case one of us dies, we need to replace ourselves.
     */
    pub fn set_administrator(
        program: &mut Account<App>,
        admin: &mut Account<User>,
        user: &mut Account<User>,
        status: bool,
    ) -> Result<()> {
        program.check_admin_rights(admin)?;
        user.update_admin_status(status);

        Ok(())
    }

    /**
     * Can make people ambassador on the fly
     */
    pub fn set_ambassador(
        program: &mut Account<App>,
        admin: &mut Account<User>,
        user: &mut Account<User>,
        status: bool,
    ) -> Result<()> {
        program.check_admin_rights(admin)?;
        user.update_amb_status(status);

        Ok(())
    }

    /**
     * Precautionary measures in case we need to adjust the masternode rate.
     */
    pub fn set_staking_requirement(
        program: &mut Account<App>,
        admin: &mut Account<User>,
        amount_of_tokens: u64,
    ) -> Result<()> {
        program.check_admin_rights(admin)?;
        program.staking_requirement = amount_of_tokens;

        Ok(())
    }

    /**
     * If we want to rebrand, we can.
     */
    pub fn set_name(
        program: &mut Account<App>,
        admin: &mut Account<User>,
        new_name: String,
    ) -> Result<()> {
        program.check_admin_rights(admin)?;
        program.name = new_name;

        Ok(())
    }

    /**
     * If we want to rebrand, we can.
     */
    pub fn set_symbol(
        program: &mut Account<App>,
        admin: &mut Account<User>,
        new_symbol: String,
    ) -> Result<()> {
        program.check_admin_rights(admin)?;
        program.symbol = new_symbol;

        Ok(())
    }
}

// Getters public functions read only
impl App {
    /**
     * Retrieve the dividends owned by the _caller.
     * If `_includeReferralBonus` is to to 1/true, the referral bonus will be included in the calculations.
     * The reason for this, is that in the frontend, we will want to get the total divs (global + ref)
     * But in the internal calculations, we want them separate.
     */
    pub fn my_dividends(
        program: &mut Account<App>,
        user: &Account<User>,
        include_referral_bonus: bool,
    ) -> u64 {
        if include_referral_bonus {
            program.dividends_of(user) + user.referred_balance
        } else {
            program.dividends_of(user)
        }
    }

    /**
     * Return the buy price of 1 individual token.
     */
    pub fn sell_price(&mut self) -> u64 {
        // our calculation relies on the token supply, so we need supply. Doh.
        if self.token_supply == 0 {
            return 0;
        } else {
            let lamport: u64 = self.tokens_to_lamport(u64::pow(10, self.decimals as u32));
            let dividends: u64 = lamport / (self.dividend_fee) as u64;
            let taxed_lamport: u64 = lamport - (dividends);
            taxed_lamport
        }
    }

    /**
     * Return the sell price of 1 individual token.
     */
    pub fn buy_price(&mut self) -> u64 {
        // our calculation relies on the token supply, so we need supply. Doh.
        if self.token_supply == 0 {
            self.token_initial_price + self.token_incremental_price
        } else {
            let lamport: u64 = self.tokens_to_lamport(u64::pow(10, self.decimals as u32));
            let dividends: u64 = lamport / (self.dividend_fee) as u64;
            let taxed_lamport: u64 = lamport - (dividends);
            taxed_lamport
        }
    }

    /**
     * Function for the frontend to dynamically retrieve the price scaling of buy orders.
     */
    pub fn calculate_tokens_received(&mut self, lamport_to_spend: u64) -> u64 {
        if lamport_to_spend == 0 {
            return 0;
        }
        let dividends: u64 = lamport_to_spend / (self.dividend_fee as u64);
        let taxed_lamport: u64 = lamport_to_spend - dividends;
        let amount_of_tokens: u64 = self.lamport_to_tokens(taxed_lamport);

        amount_of_tokens
    }

    /**
     * Function for the frontend to dynamically retrieve the price scaling of sell orders.
     */
    pub fn calculate_lamports_received(&mut self, token_to_sell: u64) -> u64 {
        if token_to_sell > self.token_supply || token_to_sell == 0 {
            return 0;
        }
        let lamport = self.tokens_to_lamport(token_to_sell);
        let dividends: u64 = lamport / (self.dividend_fee as u64);
        let taxed_lamport: u64 = lamport - (dividends);

        taxed_lamport
    }
}

// CONSTANTS
impl App {
    pub const MAXIMUM_SIZE: usize = 1 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 1 + 20; //  20 bytes for token name and symbol
}

// Public functions
impl App {
    // initialize the state
    pub fn initialize(
        program: &mut Account<'_, App>,
        admin_account: &Signer,
        admin_data_account: &mut Account<'_, User>,
    ) -> Result<()> {
        require!(!program.is_initialized, ProgramError::AlreadyInitialized);

        // Initialize main project state
        program.name = String::from("App");
        program.symbol = String::from("APP");
        program.decimals = 9;
        program.dividend_fee = 10;
        program.token_initial_price = 100000;
        program.token_incremental_price = 10000;
        program.token_supply = 0;
        program.magnitude = u64::pow(2, 32);
        program.staking_requirement = 2000_000_000_000;
        program.ambassador_max_purchase = LAMPORTS_PER_SOL;
        program.ambassador_quota = LAMPORTS_PER_SOL * 20;
        program.profit_per_share = 0;
        program.only_ambassadors = true;
        program.is_initialized = true;

        // Initialize the signer as the admin and ambassador
        admin_data_account.authority = admin_account.key();
        admin_data_account.balance = 0;
        admin_data_account.is_admin = true;
        admin_data_account.is_amb = true;

        Ok(())
    }

    /**
     * Converts all incoming lamports to tokens for the signer, and passes down the referral addy (if any)
     */
    pub fn buy<'a>(
        program: &mut Account<'a, App>,
        buyer: &mut Signer<'a>,
        buyer_data_account: &mut Account<'_, User>,
        referred_by_data_account: &mut Option<Account<'_, User>>,
        lamports: u64,
        referred_by: Option<Pubkey>,
        sys_info: AccountInfo<'a>,
    ) -> Result<u64> {
        if buyer_data_account
            .authority
            .key()
            .eq(&Pubkey::default().key())
        {
            buyer_data_account.authority = buyer.key();
        }

        program.owns_account(buyer, buyer_data_account)?;
        App::purchase_tokens(
            program,
            buyer,
            sys_info,
            buyer_data_account,
            referred_by_data_account,
            lamports,
            referred_by,
            true,
        )
    }

    /**
     * Converts all of _caller's dividends to tokens.
     */
    pub fn reinvest<'a>(
        program: &mut Account<'a, App>,
        user: &mut Signer<'a>,
        user_data_account: &mut Account<'_, User>,
        sys_info: AccountInfo<'a>,
    ) -> Result<()> {
        if user_data_account
            .authority
            .key()
            .eq(&Pubkey::default().key())
        {
            user_data_account.authority = user.key();
        }
        program.owns_account(user, user_data_account)?;
        let dividends = App::my_dividends(program, user_data_account, false);
        let mut none: Option<Account<User>> = None;

        let tokens = App::purchase_tokens(
            program,
            user,
            sys_info,
            user_data_account,
            &mut none,
            dividends,
            None,
            false,
        )?;

        on_reinvestment(user.key(), dividends, tokens);
        Ok(())
    }

    /**
     * Alias of sell() and withdraw().
     */
    pub fn leave(
        program: &mut Account<'_, App>,
        user: &mut Signer,
        user_data_account: &mut Account<'_, User>,
    ) -> Result<()> {
        if user_data_account
            .authority
            .key()
            .eq(&Pubkey::default().key())
        {
            user_data_account.authority = user.key();
        }
        program.owns_account(user, user_data_account)?;
        let tokens = user_data_account.balance;
        if tokens > 0 {
            App::sell(program, user, user_data_account, tokens)?;
        }

        // lambo delivery service
        App::withdraw(program, user, user_data_account)?;
        Ok(())
    }

    /**
     * Transfer tokens from the _caller to a new holder.
     * Remember, there's a 10% fee here as well.
     */
    pub fn transfer(
        program: &mut Account<'_, App>,
        user: &mut Signer,
        user_data_account: &mut Account<'_, User>,
        to: Pubkey,
        to_data_account: &mut Account<'_, User>,
        amount_of_tokens: u64,
    ) -> Result<bool> {
        if user_data_account
            .authority
            .key()
            .eq(&Pubkey::default().key())
        {
            user_data_account.authority = user.key();
        }
        if to_data_account.authority.key().eq(&Pubkey::default().key()) {
            to_data_account.authority = to.key();
        } else {
            require_keys_eq!(
                to_data_account.authority.key(),
                to.key(),
                ProgramError::InvalidToAccount
            )
        }
        program.owns_account(user, user_data_account)?;
        program.only_bagholders(user_data_account)?;

        // make sure we have the requested tokens
        program.has_enough(user_data_account, amount_of_tokens)?;

        // also disables transfers until ambassador phase is over
        // ( we dont want whale premines )
        require!(!program.only_ambassadors, ProgramError::AmbassadorPhase);

        // withdraw all outstanding dividends first
        if App::my_dividends(program, user_data_account, true) > 0 {
            App::withdraw(program, user, user_data_account)?;
        }

        // liquify 10% of the tokens that are transfered
        // these are dispersed to shareholders
        let token_fee = amount_of_tokens / (program.dividend_fee) as u64;
        let taxed_tokens = amount_of_tokens - token_fee;
        let _dividends = App::tokens_to_lamport(program, token_fee);

        // burn the fee tokens
        program.token_supply -= token_fee;

        // exchange tokens
        user_data_account.decrease_balance_by(amount_of_tokens);
        to_data_account.increase_balance_by(taxed_tokens);

        // update dividend trackers
        let from_payouts = (program.profit_per_share * amount_of_tokens) as i64;
        let to_payouts = (program.profit_per_share * taxed_tokens) as i64;

        user_data_account.decrease_payout_by(from_payouts);
        to_data_account.increase_payout_by(to_payouts);

        // disperse dividends among holders
        program.profit_per_share += (_dividends * program.magnitude) / program.token_supply;

        // fire event
        on_transfer(
            user_data_account.authority,
            to_data_account.authority,
            amount_of_tokens,
        );

        Ok(true)
    }

    /**
     * Withdraws all of the _callers earnings.
     */
    pub fn withdraw(
        program: &mut Account<'_, App>,
        user: &mut Signer,
        user_data_account: &mut Account<'_, User>,
    ) -> Result<()> {
        if user_data_account
            .authority
            .key()
            .eq(&Pubkey::default().key())
        {
            user_data_account.authority = user.key();
        }
        program.owns_account(user, user_data_account)?;
        App::only_stronghands(program, user_data_account)?;

        let mut dividends = App::my_dividends(program, user_data_account, false); // get ref. bonus later in the code

        let updated_payouts = (dividends * program.magnitude) as i64;
        // update dividend tracker
        user_data_account.increase_payout_by(updated_payouts);

        // add ref. bonus
        let ref_amount = user_data_account.referred_balance;
        dividends += ref_amount;
        user_data_account.decrease_referred_balance_by(ref_amount);

        // lambo delivery service
        // Transfer from the app state
        App::transfer_sol_out(program, user, dividends)?;

        // fire event
        on_withdraw(user_data_account.authority, dividends);

        Ok(())
    }

    /**
     * Liquifies tokens to ether.
     */
    pub fn sell(
        program: &mut Account<'_, App>,
        user: &Signer,
        user_data_account: &mut Account<'_, User>,
        amount_of_tokens: u64,
    ) -> Result<()> {
        if user_data_account
            .authority
            .key()
            .eq(&Pubkey::default().key())
        {
            user_data_account.authority = user.key();
        }
        program.owns_account(user, user_data_account)?;
        program.only_bagholders(user_data_account)?;

        // russian hackers BTFO
        program.has_enough(user_data_account, amount_of_tokens)?;

        let tokens = amount_of_tokens;
        let _lamport = App::tokens_to_lamport(program, tokens);
        let dividends = _lamport / program.dividend_fee as u64;
        let taxed_lamport = _lamport - dividends;

        // burn the sold tokens
        program.token_supply -= tokens;
        user_data_account.decrease_balance_by(tokens);

        // update dividends tracker
        let updated_payouts =
            (program.profit_per_share * tokens + (taxed_lamport * program.magnitude)) as i64;
        user_data_account.decrease_payout_by(updated_payouts);

        // dividing by zero is a bad idea
        if program.token_supply > 0 {
            // update the amount of dividends per token
            program.profit_per_share += (dividends * program.magnitude) / program.token_supply;
        }

        // fire event
        on_token_sell(user_data_account.authority, tokens, taxed_lamport);

        Ok(())
    }
}
