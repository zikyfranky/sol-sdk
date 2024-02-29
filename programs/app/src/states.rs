use {
    crate::constants::{LAMPORTS_IN_SOL, MINT_SEED},
    crate::errors::ProgramError,
    crate::events::*,
    crate::utils::*,
    anchor_lang::{
        prelude::*,
        solana_program::{program::invoke, system_instruction::transfer},
        AnchorDeserialize, AnchorSerialize,
    },
    anchor_spl::token_2022::{burn, mint_to, Burn, MintTo},
};

#[account]
pub struct User {
    authority: Pubkey,
    balance: u128,
    referred_balance: u128,
    is_admin: bool,
    is_amb: bool,
    referred_by: Pubkey,
    payout: i128,
    usable_locked: u128,
    total_locked: u128,
    locked_starttime: i64,
    locked_endtime: i64,
}

// Helper functions
impl User {
    fn actually_locked(&mut self) -> u128 {
        let ten_days = 86400 * 10;
        let clock: Clock = Clock::get().unwrap();
        let now = clock.unix_timestamp;
        if now >= self.locked_endtime {
            self.usable_locked = self.total_locked;
        } else {
            let days_gone_in_tens = (now - self.locked_starttime) / ten_days;
            let amount = (self.total_locked / 10) * days_gone_in_tens as u128;
            if amount > self.usable_locked && amount <= self.total_locked {
                self.usable_locked = amount;
            }
        }

        self.total_locked - self.usable_locked
    }

    fn has_admin_rights(&mut self) -> bool {
        self.is_admin
    }

    fn has_balance(&mut self) -> bool {
        self.balance > 0
    }

    fn usable_balance(&mut self) -> u128 {
        self.balance - self.actually_locked()
    }

    fn has_balance_upto(&mut self, amount: u128, include_locked: bool) -> bool {
        if include_locked {
            return self.balance > amount;
        }
        self.balance - self.actually_locked() >= amount
    }

    fn increase_balance_by(&mut self, amount: u128) {
        self.balance += amount;
    }

    fn decrease_balance_by(&mut self, amount: u128) {
        self.balance -= amount;
    }

    fn increase_payout_by(&mut self, amount: i128) {
        self.payout += amount;
    }

    fn decrease_payout_by(&mut self, amount: i128) {
        self.payout -= amount;
    }

    fn increase_referred_balance_by(&mut self, amount: u128) {
        self.referred_balance += amount;
    }

    fn decrease_referred_balance_by(&mut self, amount: u128) {
        self.referred_balance -= amount;
    }

    fn update_admin_status(&mut self, status: bool) {
        self.is_admin = status;
    }

    fn update_amb_status(&mut self, status: bool) {
        self.is_amb = status;
    }
}

impl User {
    pub const MAXIMUM_SIZE: usize = 32 + 16 + 16 + 1 + 1 + 32 + 16 + 16 + 16 + 8 + 8;
}

#[account]
pub struct App {
    name: String,
    symbol: String,
    decimals: u8,
    dividend_fee: u8,
    token_initial_price: u128,
    token_incremental_price: u128,
    contract_balance: u128,
    token_supply: u128,
    magnitude: u64,
    staking_requirement: u128,
    profit_per_share: u128,
    is_initialized: bool,
    is_initial_phase: bool,
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
        -> change the PoS difficulty (How many tokens it costs to hold a skwizkey, in case it gets crazy high later)
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

    fn is_post_initial_phase_or_admin(&mut self, user: &mut Account<User>) -> Result<()> {
        require!(
            !self.is_initial_phase || (self.is_initial_phase && user.is_admin),
            ProgramError::IsInitialPhase
        );

        Ok(())
    }

    // Check that user has enough funds to use
    fn has_enough(&mut self, user: &mut Account<User>, amount: u128) -> Result<()> {
        require!(
            user.has_balance_upto(amount, false),
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
    fn mint<'a>(
        data_account: &mut Account<'_, User>,
        quantity: u128,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        user_ata: AccountInfo<'a>,
        bump: u8,
    ) -> Result<bool> {
        let seeds = &[MINT_SEED, &[bump]];
        let signer = [&seeds[..]];

        mint_to(
            CpiContext::new_with_signer(
                token_program,
                MintTo {
                    authority: mint.clone(),
                    to: user_ata,
                    mint: mint.clone(),
                },
                &signer,
            ),
            quantity as u64,
        )?;

        data_account.increase_balance_by(quantity);

        Ok(true)
    }

    fn burn<'a>(
        signer: AccountInfo<'a>,
        data_account: &mut Account<'_, User>,
        quantity: u128,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        user_ata: AccountInfo<'a>,
        _bump: u8,
    ) -> Result<bool> {
        // let seeds = &[MINT_SEED, &[bump]];
        // let seeds = &[signer.key().as_ref()];
        // let sigsner = [&seeds[..], &seeds[..]];

        burn(
            CpiContext::new(
                token_program,
                Burn {
                    authority: signer,
                    from: user_ata,
                    mint: mint.clone(),
                },
            ),
            quantity as u64,
        )?;

        data_account.decrease_balance_by(quantity);

        Ok(true)
    }

    fn purchase_tokens<'a>(
        program: &mut Account<'a, App>,
        buyer: &mut Signer<'a>,
        system: AccountInfo<'a>,
        buyer_data_account: &mut Account<'_, User>,
        referred_by_data_account: &mut Option<Account<'_, User>>,
        lamports: u128,
        referred_by: Option<Pubkey>,
        direct_buy: bool,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        user_ata: AccountInfo<'a>,
        bump: u8,
    ) -> Result<u128> {
        program.is_post_initial_phase_or_admin(buyer_data_account)?;
        // data setup
        let buyer_key = buyer.key();
        let undivided_dividends = lamports / (program.dividend_fee as u128);
        let referral_bonus = undivided_dividends / 3;
        let mut dividends = undivided_dividends - referral_bonus;
        let taxed_lamport = lamports - undivided_dividends;
        let amount_of_tokens = program.lamport_to_tokens(taxed_lamport);
        let mut fee = dividends * program.magnitude as u128;
        let r_by = referred_by.unwrap_or_default();
        let referred = if Pubkey::default().eq(&buyer_data_account.referred_by) {
            buyer_data_account.referred_by = r_by;
            buyer_data_account.referred_by
        } else {
            buyer_data_account.referred_by
        };

        // Lot of checks
        // prevents overflow in the case that the pyramid somehow magically starts being used by everyone in the world
        // (or hackers)
        require_gt!(amount_of_tokens, 0, ProgramError::SentLessToken);
        require_gte!(
            buyer.get_lamports(),
            lamports as u64,
            ProgramError::InsufficientBalance
        );
        require_gt!(
            amount_of_tokens + program.token_supply,
            program.token_supply,
            ProgramError::SentLessToken
        );

        // is the user referred by a skwizkey?
        if
        // is this a referred purchase?
        !Pubkey::default().eq(&referred) &&
			// no cheating!
			!referred.eq(&buyer_key)
        {
            if let Some(referred_by_data) = referred_by_data_account {
                // does the referrer have at least X whole tokens?
                // i.e is the referrer a godly chad skwizkey
                if referred_by_data.has_balance_upto(program.staking_requirement, true) {
                    // wealth redistribution
                    referred_by_data.increase_referred_balance_by(referral_bonus);

                    // Emit an event
                    on_skwizkey(referred, buyer_key, lamports, referral_bonus);
                } else {
                    // no ref purchase
                    // add the referral bonus back to the global dividends cake
                    dividends += referral_bonus;
                    fee = dividends * program.magnitude as u128;
                }
            } else {
                // no ref purchase
                // add the referral bonus back to the global dividends cake
                dividends += referral_bonus;
                fee = dividends * program.magnitude as u128;
            }
        } else {
            // no ref purchase
            // add the referral bonus back to the global dividends cake
            dividends += referral_bonus;
            fee = dividends * program.magnitude as u128;
        }

        // we can't give people infinite ethereum
        if program.token_supply > 0 {
            // add tokens to the pool
            program.token_supply += amount_of_tokens;

            // take the amount of dividends gained through this transaction, and allocates them evenly to each shareholder
            program.profit_per_share +=
                (dividends * program.magnitude as u128) / (program.token_supply);

            // calculate the amount of tokens the customer receives over his purchase
            fee = fee
                - (fee
                    - (amount_of_tokens
                        * ((dividends * program.magnitude as u128) / (program.token_supply))));
        } else {
            // add tokens to the pool
            program.token_supply = amount_of_tokens;
        }

        // update circulating supply & the ledger address for the customer
        App::mint(
            buyer_data_account,
            amount_of_tokens,
            token_program,
            mint,
            user_ata,
            bump,
        )?;

        // Tells the contract that the buyer doesn't deserve dividends for the tokens before they owned them;
        //really i know you think you do but you don't
        let updated_payouts: i128 =
            ((program.profit_per_share * amount_of_tokens) as i128 - fee as i128) as i128;

        buyer_data_account.increase_payout_by(updated_payouts);

        // fire event
        on_token_purchase(buyer_key, lamports, amount_of_tokens, referred);

        if direct_buy {
            App::transfer_sol_in(buyer, program, system, lamports as u64)?;
        }

        Ok(amount_of_tokens)
    }

    /**
     * Calculate Token price based on an amount of incoming lamport
     * It's an algorithm, hopefully we gave you the whitepaper with it in scientific notation;
     * Some conversions occurred to prevent decimal errors or underflows / overflows in solidity code.
     */
    fn lamport_to_tokens(&mut self, lamport: u128) -> u128 {
        let token_price_initial = self.token_initial_price; // 100
        let token_price_incremental = self.token_incremental_price; // 1000
        let token_supply = self.token_supply; // 0

        let token_price_initial_expanded = token_price_initial * LAMPORTS_IN_SOL; //100 * 1e9  = 100e9
        let token_price_incremental_expanded = token_price_incremental * LAMPORTS_IN_SOL; //1000 * 1e9  = 1000e9
        let lamport_expanded = lamport * LAMPORTS_IN_SOL; //1e9*1e9 =  1e18

        let token_price_initial_to_power = token_price_initial_expanded.pow(2);
        let token_price_incremental_to_power = token_price_incremental.pow(2);
        let token_supply_to_power = token_supply.pow(2);

        let first = token_price_initial_to_power;
        let second = 2 * token_price_incremental_expanded * lamport_expanded;
        let third = token_price_incremental_to_power * token_supply_to_power;
        let fourth = 2 * token_price_incremental * token_price_initial_expanded * token_supply;

        let square_root = sqrt(first + second + third + fourth);
        let tokens_received =
            ((square_root - token_price_initial_expanded) / token_price_incremental) - token_supply;

        tokens_received
    }

    /**
     * Calculate token sell value.
     * It's an algorithm, hopefully we gave you the whitepaper with it in scientific notation;
     * Some conversions occurred to prevent decimal errors or underflows / overflows in solidity code.
     */
    fn tokens_to_lamport(&mut self, tokens: u128) -> u128 {
        let tokens = tokens + LAMPORTS_IN_SOL;
        let token_supply = self.token_supply + LAMPORTS_IN_SOL;

        let token_initial_price = self.token_initial_price;
        let token_incremental_price = self.token_incremental_price;

        let first = (token_initial_price
            + (token_incremental_price * (token_supply / LAMPORTS_IN_SOL)))
            - token_incremental_price;

        let second = tokens - LAMPORTS_IN_SOL;
        let third = token_incremental_price * ((tokens.pow(2) - tokens) / LAMPORTS_IN_SOL) / 2;

        // underflow attempts BTFO
        let lamport_to_receive = ((first * second) - (third)) / LAMPORTS_IN_SOL;

        lamport_to_receive
    }

    /**
     * Retrieve the dividend balance of any single address.
     */
    fn dividends_of(&mut self, user: &Account<User>) -> u128 {
        ((((self.profit_per_share * user.balance) as i128) - user.payout) / self.magnitude as i128)
            as u128
    }

    fn transfer_sol_out(
        from: &mut Account<'_, App>,
        to: &mut Signer,
        // sys: AccountInfo<'a>,
        amount: u64,
    ) -> Result<bool> {
        require!(
            amount <= from.contract_balance as u64,
            ProgramError::InsufficientBalance
        );
        from.contract_balance -= amount as u128;

        // let _from = from.to_account_info();
        // let _to = to.to_account_info();
        // let ix = transfer(&from.key(), &to.key(), amount);
        // invoke(&ix, &[_from, _to, sys])?;
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
        to.contract_balance += amount as u128;

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

        program.is_initial_phase = false;
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
     * Precautionary measures in case we need to adjust the skwizkey rate.
     */
    pub fn set_staking_requirement(
        program: &mut Account<App>,
        admin: &mut Account<User>,
        amount_of_tokens: u128,
    ) -> Result<()> {
        program.check_admin_rights(admin)?;
        program.staking_requirement = amount_of_tokens;

        Ok(())
    }

    /**
     * Updates information of the metadata of mint
     * @TODO: Update token information
     */
    pub fn update_metadata_account(
        program: &mut Account<App>,
        admin: &mut Account<User>,
    ) -> Result<()> {
        program.check_admin_rights(admin)?;
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
    ) -> u128 {
        if include_referral_bonus {
            program.dividends_of(user) + user.referred_balance
        } else {
            program.dividends_of(user)
        }
    }

    /**
     * Return the buy price of 1 individual token.
     */
    pub fn sell_price(&mut self) -> u128 {
        // our calculation relies on the token supply, so we need supply. Doh.
        if self.token_supply == 0 {
            self.token_initial_price - self.token_incremental_price
        } else {
            let lamport: u128 = self.tokens_to_lamport(u128::pow(10, self.decimals as u32));
            let dividends: u128 = lamport / (self.dividend_fee) as u128;
            let taxed_lamport: u128 = lamport - (dividends);
            taxed_lamport
        }
    }

    /**
     * Return the sell price of 1 individual token.
     */
    pub fn buy_price(&mut self) -> u128 {
        // our calculation relies on the token supply, so we need supply. Doh.
        if self.token_supply == 0 {
            self.token_initial_price + self.token_incremental_price
        } else {
            let lamport: u128 = self.tokens_to_lamport(u128::pow(10, self.decimals as u32));
            let dividends: u128 = lamport / (self.dividend_fee) as u128;
            let taxed_lamport: u128 = lamport - (dividends);
            taxed_lamport
        }
    }

    /**
     * Function for the frontend to dynamically retrieve the price scaling of buy orders.
     */
    pub fn calculate_tokens_received(&mut self, lamport_to_spend: u128) -> u128 {
        if lamport_to_spend == 0 {
            return 0;
        }
        let dividends: u128 = lamport_to_spend / (self.dividend_fee as u128);
        let taxed_lamport: u128 = lamport_to_spend - dividends;
        let amount_of_tokens: u128 = self.lamport_to_tokens(taxed_lamport);

        amount_of_tokens
    }

    /**
     * Function for the frontend to dynamically retrieve the price scaling of sell orders.
     */
    pub fn calculate_lamports_received(&mut self, token_to_sell: u128) -> u128 {
        if token_to_sell > self.token_supply || token_to_sell == 0 {
            return 0;
        }
        let lamport = self.tokens_to_lamport(token_to_sell);
        let dividends: u128 = lamport / (self.dividend_fee as u128);
        let taxed_lamport: u128 = lamport - (dividends);

        taxed_lamport
    }
}

// CONSTANTS
impl App {
    pub const MAXIMUM_SIZE: usize = 1 + 1 + 16 + 16 + 16 + 16 + 8 + 16 + 16 + 1 + 1 + 20; //  20 bytes for token name and symbol
}

// Public functions
impl App {
    // initialize the state
    pub fn initialize(
        program: &mut Account<'_, App>,
        admin_account: &Signer,
        admin_data_account: &mut Account<'_, User>,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<()> {
        require!(!program.is_initialized, ProgramError::AlreadyInitialized);

        // Initialize main project state
        program.name = name;
        program.symbol = symbol;
        program.decimals = decimals;
        program.dividend_fee = 10;
        program.token_initial_price = 100000;
        program.token_incremental_price = 100;
        program.magnitude = u64::pow(2, 32);
        program.staking_requirement = LAMPORTS_IN_SOL * 2000; // 2000 coins
        program.is_initialized = true;
        program.is_initial_phase = true;

        // Initialize the signer as the admin and ambassador
        admin_data_account.authority = admin_account.key();
        admin_data_account.balance = 0;
        admin_data_account.is_admin = true;
        admin_data_account.is_amb = true;
        admin_data_account.referred_by = Pubkey::default();

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
        lamports: u128,
        referred_by: Option<Pubkey>,
        sys_info: AccountInfo<'a>,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        user_ata: AccountInfo<'a>,
        bump: u8,
    ) -> Result<u128> {
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
            token_program,
            mint,
            user_ata,
            bump,
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
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        user_ata: AccountInfo<'a>,
        bump: u8,
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
            token_program,
            mint,
            user_ata,
            bump,
        )?;

        on_reinvestment(user.key(), dividends, tokens);
        Ok(())
    }

    /**
     * Alias of sell() and withdraw().
     */
    pub fn leave<'a>(
        program: &mut Account<'a, App>,
        user: &mut Signer<'a>,
        user_data_account: &mut Account<'a, User>,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        user_ata: AccountInfo<'a>,
        bump: u8,
    ) -> Result<()> {
        require!(!program.is_initial_phase, ProgramError::IsInitialPhase);

        if user_data_account
            .authority
            .key()
            .eq(&Pubkey::default().key())
        {
            user_data_account.authority = user.key();
        }
        program.owns_account(user, user_data_account)?;
        let tokens = user_data_account.usable_balance();
        if tokens > 0 {
            App::sell(
                program,
                user,
                user_data_account,
                tokens,
                token_program,
                mint,
                user_ata,
                bump,
            )?;
        }

        // lambo delivery service
        App::withdraw(program, user, user_data_account, true)?;
        Ok(())
    }

    /**
     * Transfer tokens from the _caller to a new holder.
     * Remember, there's a 10% fee here as well.
     */
    pub fn transfer<'a>(
        program: &mut Account<'a, App>,
        user: &mut Signer<'a>,
        user_data_account: &mut Account<'a, User>,
        user_ata: AccountInfo<'a>,
        to: Pubkey,
        to_data_account: &mut Account<'a, User>,
        to_ata: AccountInfo<'a>,
        amount_of_tokens: u128,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        bump: u8,
    ) -> Result<bool> {
        require!(!program.is_initial_phase, ProgramError::IsInitialPhase);
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

        let mut trans_amount: u128 = 0;
        // withdraw all outstanding dividends first
        if App::my_dividends(program, user_data_account, true) > 0 {
            trans_amount = App::withdraw(program, user, user_data_account, false)?;
        }

        // liquify 10% of the tokens that are transfered
        // these are dispersed to shareholders
        let token_fee = amount_of_tokens / (program.dividend_fee as u128);
        let taxed_tokens = amount_of_tokens - token_fee;
        let _dividends = App::tokens_to_lamport(program, token_fee);

        // burn the fee tokens
        program.token_supply -= token_fee;
        // exchange tokens
        App::burn(
            user.to_account_info(),
            user_data_account,
            amount_of_tokens,
            token_program.clone(),
            mint.clone(),
            user_ata,
            bump,
        )?;
        App::mint(
            to_data_account,
            taxed_tokens,
            token_program,
            mint,
            to_ata,
            bump,
        )?;

        // update dividend trackers
        let from_payouts = (program.profit_per_share * amount_of_tokens) as i128;
        let to_payouts = (program.profit_per_share * taxed_tokens) as i128;

        user_data_account.decrease_payout_by(from_payouts);
        to_data_account.increase_payout_by(to_payouts);

        // disperse dividends among holders
        program.profit_per_share += (_dividends * program.magnitude as u128) / program.token_supply;

        // fire event
        on_transfer(
            user_data_account.authority,
            to_data_account.authority,
            amount_of_tokens,
        );

        App::transfer_sol_out(program, user, trans_amount as u64)?;

        Ok(true)
    }

    /**
     * Withdraws all of the _callers earnings.
     */
    pub fn withdraw(
        program: &mut Account<'_, App>,
        user: &mut Signer,
        user_data_account: &mut Account<'_, User>,
        direct_call: bool,
    ) -> Result<u128> {
        require!(!program.is_initial_phase, ProgramError::IsInitialPhase);

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

        let updated_payouts = (dividends * program.magnitude as u128) as i128;
        // update dividend tracker
        user_data_account.increase_payout_by(updated_payouts);

        // add ref. bonus
        let ref_amount = user_data_account.referred_balance;
        dividends += ref_amount;
        user_data_account.decrease_referred_balance_by(ref_amount);

        // lambo delivery service
        // Transfer from the app state
        if direct_call {
            App::transfer_sol_out(program, user, dividends as u64)?;
        }

        // fire event
        on_withdraw(user_data_account.authority, dividends);

        Ok(dividends)
    }

    /**
     * Liquifies tokens to ether.
     */
    pub fn sell<'a>(
        program: &mut Account<'a, App>,
        user: &Signer<'a>,
        user_data_account: &mut Account<'a, User>,
        amount_of_tokens: u128,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        user_ata: AccountInfo<'a>,
        bump: u8,
    ) -> Result<()> {
        require!(!program.is_initial_phase, ProgramError::IsInitialPhase);

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
        let dividends = _lamport / program.dividend_fee as u128;
        let taxed_lamport = _lamport - dividends;

        // burn the sold tokens
        program.token_supply -= tokens;

        App::burn(
            user.to_account_info(),
            user_data_account,
            tokens,
            token_program,
            mint,
            user_ata,
            bump,
        )?;

        // update dividends tracker
        let updated_payouts = (program.profit_per_share as i128 * tokens as i128
            + (taxed_lamport as i128 * program.magnitude as i128))
            as i128;
        user_data_account.decrease_payout_by(updated_payouts);

        // dividing by zero is a bad idea
        if program.token_supply > 0 {
            // update the amount of dividends per token
            program.profit_per_share +=
                (dividends * program.magnitude as u128) / program.token_supply;
        }

        // fire event
        on_token_sell(user_data_account.authority, tokens, taxed_lamport);

        Ok(())
    }

    /**
     * Transfer to whitelisted wallets.
     */
    pub fn distribute_token<'a>(
        program: &mut Account<'a, App>,
        user: &Signer<'a>,
        receipient: Pubkey,
        from_data_account: &mut Account<'a, User>,
        from_ata: AccountInfo<'a>,
        receipient_data_account: &mut Account<'a, User>,
        receipient_ata: AccountInfo<'a>,
        amount_of_tokens: u128,
        payout: i128,
        token_program: AccountInfo<'a>,
        mint: AccountInfo<'a>,
        bump: u8,
        s_timestamp: i64,
        e_timestamp: i64,
    ) -> Result<()> {
        require!(program.is_initial_phase, ProgramError::IsPostInitialPhase);
        require!(
            from_data_account.has_admin_rights(),
            ProgramError::NotAnAdmin
        );

        if receipient_data_account
            .authority
            .key()
            .eq(&Pubkey::default().key())
        {
            receipient_data_account.authority = receipient;
        }

        App::burn(
            user.to_account_info(),
            from_data_account,
            amount_of_tokens,
            token_program.clone(),
            mint.clone(),
            from_ata,
            bump,
        )?;

        App::mint(
            receipient_data_account,
            amount_of_tokens,
            token_program,
            mint,
            receipient_ata,
            bump,
        )?;

        // update dividends tracke
        from_data_account.decrease_payout_by(payout);
        receipient_data_account.increase_payout_by(payout);

        //
        receipient_data_account.total_locked = amount_of_tokens;
        receipient_data_account.usable_locked = 0;
        receipient_data_account.locked_starttime = s_timestamp;
        receipient_data_account.locked_endtime = e_timestamp;

        Ok(())
    }
}
