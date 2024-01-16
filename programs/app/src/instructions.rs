use {
    crate::constants::*,
    crate::states::*,
    anchor_lang::{prelude::*, solana_program::program::invoke_signed},
    anchor_spl::{
        associated_token::{AssociatedToken, ID as ASSOCIATED_TOKEN_ID},
        token_2022::{Token2022, ID as TOKEN_2022_ID},
        token_interface::{
            spl_token_2022::{
                extension::{
                    // default_account_state::instruction::initialize_default_account_state,
                    ExtensionType,
                },
                instruction::{
                    initialize_mint2, initialize_non_transferable_mint,
                    initialize_permanent_delegate,
                },
                state::{
                    // AccountState,
                    Mint as MainMint,
                },
            },
            Mint, TokenAccount,
        },
    },
    mpl_token_metadata::{
        instructions::CreateCpiBuilder,
        types::{CreateArgs, TokenStandard},
        ID as MPL_TOKEN_METADATA_ID,
    },
    std::str::FromStr,
};

#[derive(Accounts)]
#[instruction(params: InitTokenParams)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, seeds=[USER_SEED, admin.key().as_ref()], bump, payer = admin, space = 8 + User::MAXIMUM_SIZE)]
    pub admin_data: Account<'info, User>,

    /// CHECK: It'd be Created by Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK gets created by the program
    #[account(
		init,
        seeds = [MINT_SEED],
        bump,
		payer = admin,
		space = ExtensionType::try_calculate_account_len::<MainMint>(&[
        ExtensionType::PermanentDelegate,
        ExtensionType::NonTransferable,
        // ExtensionType::DefaultAccountState,
    ])
    .unwrap(),
		owner = TOKEN_2022_ID,
    )]
    pub mint: UncheckedAccount<'info>,

    #[account(init, seeds=[PROGRAM_SEED], bump, payer = admin, space = 8 + App::MAXIMUM_SIZE + 10)]
    pub program_data: Account<'info, App>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: Sysvar instruction
    pub sysvar_instructions: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    #[account(address = TOKEN_2022_ID)]
    pub token_program: Program<'info, Token2022>,
    /// CHECK: account constraint checked in account trait
    #[account(address = MPL_TOKEN_METADATA_ID)]
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
#[instruction(lamports_to_send: u64, referred_by: Option<Pubkey>,)]
pub struct Buy<'info> {
    #[account(
    	mut,
        seeds = [MINT_SEED],
        bump,
        mint::authority = mint,
    	mint::freeze_authority = mint,
    	mint::token_program = TOKEN_2022_ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,
    #[account(
		init_if_needed,
		payer = user,
		associated_token::mint = mint,
		associated_token::authority = user,
		associated_token::token_program = token_program
	)]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(init_if_needed, seeds=[USER_SEED, referred_by.unwrap().key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub referred_by_data: Option<Account<'info, User>>,
    #[account(mut, seeds=[PROGRAM_SEED], bump)]
    pub program_data: Account<'info, App>,

    pub system_program: Program<'info, System>,
    #[account(address = TOKEN_2022_ID)]
    pub token_program: Program<'info, Token2022>,
    #[account(address = ASSOCIATED_TOKEN_ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Reinvest<'info> {
    #[account(
		mut,
        seeds = [MINT_SEED],
        bump,
        mint::authority = mint,
		mint::freeze_authority = mint,
		mint::token_program = TOKEN_2022_ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,
    #[account(
		init_if_needed,
		payer = user,
		associated_token::mint = mint,
		associated_token::authority = user,
	)]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds=[PROGRAM_SEED], bump)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,

    #[account(address = TOKEN_2022_ID)]
    pub token_program: Program<'info, Token2022>,
    #[account(address = ASSOCIATED_TOKEN_ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Exit<'info> {
    #[account(
		mut,
        seeds = [MINT_SEED],
        bump,
        mint::authority = mint,
		mint::freeze_authority = mint,
		mint::token_program = TOKEN_2022_ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,
    #[account(
		init_if_needed,
		payer = user,
		associated_token::mint = mint,
		associated_token::authority = user,
	)]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds=[PROGRAM_SEED], bump)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,

    #[account(address = TOKEN_2022_ID)]
    pub token_program: Program<'info, Token2022>,
    #[account(address = ASSOCIATED_TOKEN_ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(to: Pubkey)]
pub struct Transfer<'info> {
    #[account(
		mut,
		seeds = [MINT_SEED],
		bump,
		mint::authority = mint,
		mint::freeze_authority = mint,
		mint::token_program = TOKEN_2022_ID,
	)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,
    #[account(
    	mut,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = TOKEN_2022_ID,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(init_if_needed, seeds=[USER_SEED, to.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub to_data: Account<'info, User>,

    /// CHECK: To Authority
    #[account(address = to)]
    pub to_info: AccountInfo<'info>,

    #[account(
		init_if_needed,
		payer = user,
		associated_token::mint = mint,
		associated_token::authority = to_info,
	)]
    pub to_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds=[PROGRAM_SEED], bump)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,

    #[account(address = TOKEN_2022_ID)]
    pub token_program: Program<'info, Token2022>,
    #[account(address = ASSOCIATED_TOKEN_ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [MINT_SEED],
        bump,
        mint::authority = mint,
		mint::token_program = TOKEN_2022_ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,
    #[account(
		init_if_needed,
		payer = user,
		associated_token::mint = mint,
		associated_token::authority = user,
	)]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds=[PROGRAM_SEED], bump)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,

    #[account(address = TOKEN_2022_ID)]
    pub token_program: Program<'info, Token2022>,
    #[account(address = ASSOCIATED_TOKEN_ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(
        mut,
        seeds = [MINT_SEED],
        bump,
        mint::authority = mint,
		mint::token_program = TOKEN_2022_ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,
    #[account(
		init_if_needed,
		payer = user,
		associated_token::mint = mint,
		associated_token::authority = user,
	)]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds=[PROGRAM_SEED], bump)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,

    #[account(address = TOKEN_2022_ID)]
    pub token_program: Program<'info, Token2022>,
    #[account(address = ASSOCIATED_TOKEN_ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Admin<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,

    #[account(mut, seeds=[PROGRAM_SEED], bump)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct AdminSetter<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, admin.key().as_ref()], bump, payer = admin, space = 8 + User::MAXIMUM_SIZE)]
    pub admin_data: Account<'info, User>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = admin, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,

    #[account(mut, seeds=[PROGRAM_SEED], bump)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProgramReadOnly<'info> {
    pub program_data: Account<'info, App>,
}

#[derive(Accounts)]
pub struct ReadOnly<'info> {
    pub user_data: Account<'info, User>,
    pub program_data: Account<'info, App>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitTokenParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}

pub fn _initialize(ctx: Context<Initialize>, params: InitTokenParams) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.sysvar_instructions.key(),
        Pubkey::from_str("Sysvar1nstructions1111111111111111111111111").unwrap_or_default()
    );

    let program = &mut ctx.accounts.program_data;
    let admin_account = &ctx.accounts.admin;
    let admin_data_account = &mut ctx.accounts.admin_data;

    let (mint, bump) = Pubkey::find_program_address(&[MINT_SEED], ctx.program_id);

    let seeds = &[MINT_SEED, &[bump]];
    let signer = [&seeds[..]];

    let non_tf_ix = initialize_non_transferable_mint(&TOKEN_2022_ID, &mint)?;

    let pd_ix = initialize_permanent_delegate(&TOKEN_2022_ID, &mint, &mint)?;

    // let das_ix = initialize_default_account_state(
    //     &TOKEN_2022_ID,
    //     &ctx.accounts.mint.key(),
    //     &AccountState::Frozen,
    // )?;

    let init = initialize_mint2(&TOKEN_2022_ID, &mint, &mint, Some(&mint), params.decimals)?;

    invoke_signed(&pd_ix, &[ctx.accounts.mint.to_account_info()], &signer)?;
    invoke_signed(&non_tf_ix, &[ctx.accounts.mint.to_account_info()], &signer)?;
    // invoke_signed(&das_ix, &[ctx.accounts.mint.to_account_info()], &signer)?;
    invoke_signed(&init, &[ctx.accounts.mint.to_account_info()], &signer)?;

    CreateCpiBuilder::new(&ctx.accounts.token_metadata_program)
        .metadata(&ctx.accounts.metadata.to_account_info())
        .mint(&ctx.accounts.mint.to_account_info(), false)
        .authority(&ctx.accounts.mint.to_account_info())
        .payer(&ctx.accounts.admin.to_account_info())
        // .update_authority(ctx.accounts.mint.key(), true)
        .update_authority(&ctx.accounts.mint.to_account_info(), false)
        .system_program(&ctx.accounts.system_program.to_account_info())
        .sysvar_instructions(&ctx.accounts.sysvar_instructions.to_account_info())
        .spl_token_program(Some(&ctx.accounts.token_program.to_account_info()))
        .create_args(CreateArgs::V1 {
            name: params.name.clone(),
            symbol: params.symbol.clone(),
            uri: params.uri,
            is_mutable: true,
            token_standard: TokenStandard::Fungible,
            decimals: Some(params.decimals),
            seller_fee_basis_points: 0,
            primary_sale_happened: false,
            // Optional
            creators: None,
            collection: None,
            uses: None,
            collection_details: None,
            rule_set: None,
            print_supply: None,
        })
        .invoke_signed(&signer)?;

    App::initialize(
        program,
        admin_account,
        admin_data_account,
        params.name,
        params.symbol,
        params.decimals,
    )?;
    Ok(())
}

pub fn _buy(ctx: Context<Buy>, lamports_to_send: u64, referred_by: Option<Pubkey>) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let buyer = &mut ctx.accounts.user;
    let buyer_data_account = &mut ctx.accounts.user_data;
    let referred_by_data_account = &mut ctx.accounts.referred_by_data;
    let system_program = ctx.accounts.system_program.to_account_info();
    let mint = ctx.accounts.mint.to_account_info();
    let user_ata = ctx.accounts.user_ata.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let bump = ctx.bumps.mint;

    App::buy(
        program,
        buyer,
        buyer_data_account,
        referred_by_data_account,
        lamports_to_send,
        referred_by,
        system_program,
        token_program,
        mint,
        user_ata,
        bump,
    )?;
    Ok(())
}

pub fn _reinvest(ctx: Context<Reinvest>) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let user = &mut ctx.accounts.user;
    let user_data_account = &mut ctx.accounts.user_data;
    let system_program = ctx.accounts.system_program.to_account_info();
    let mint = ctx.accounts.mint.to_account_info();
    let user_ata = ctx.accounts.user_ata.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let bump = ctx.bumps.mint;

    App::reinvest(
        program,
        user,
        user_data_account,
        system_program.clone(),
        token_program,
        mint,
        user_ata,
        bump,
    )?;
    Ok(())
}

pub fn _exit(ctx: Context<Exit>) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let user = &mut ctx.accounts.user;
    let user_data_account = &mut ctx.accounts.user_data;
    let mint = ctx.accounts.mint.to_account_info();
    let user_ata = ctx.accounts.user_ata.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let bump = ctx.bumps.mint;

    App::leave(
        program,
        user,
        user_data_account,
        token_program,
        mint,
        user_ata,
        bump,
    )?;
    Ok(())
}

pub fn _transfer(ctx: Context<Transfer>, to: Pubkey, tokens_to_send: u64) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let user = &mut ctx.accounts.user;
    let user_data_account = &mut ctx.accounts.user_data;
    let user_ata = ctx.accounts.user_ata.to_account_info();
    let to_data_account = &mut ctx.accounts.to_data;
    let to_ata = ctx.accounts.to_ata.to_account_info();
    let mint = ctx.accounts.mint.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let bump = ctx.bumps.mint;

    App::transfer(
        program,
        user,
        user_data_account,
        user_ata,
        to,
        to_data_account,
        to_ata,
        tokens_to_send,
        token_program,
        mint,
        bump,
    )?;
    Ok(())
}

pub fn _withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let user = &mut ctx.accounts.user;
    let user_data_account = &mut ctx.accounts.user_data;

    App::withdraw(program, user, user_data_account, true)?;
    Ok(())
}

pub fn _sell(ctx: Context<Sell>, lamports_to_send: u64) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let user = &mut ctx.accounts.user;
    let user_data_account = &mut ctx.accounts.user_data;
    let mint = ctx.accounts.mint.to_account_info();
    let user_ata = ctx.accounts.user_ata.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let bump = ctx.bumps.mint;

    App::sell(
        program,
        user,
        user_data_account,
        lamports_to_send,
        token_program,
        mint,
        user_ata,
        bump,
    )?;
    Ok(())
}

// ADMIN FUNCTIONS
pub fn _disable_initial_stage(ctx: Context<Admin>) -> Result<()> {
    let user_data_account = &mut ctx.accounts.user_data;
    let program = &mut ctx.accounts.program_data;
    App::disable_initial_stage(program, user_data_account)?;
    Ok(())
}

pub fn _set_administrator(ctx: Context<AdminSetter>, _user: Pubkey, status: bool) -> Result<()> {
    let admin_data_account = &mut ctx.accounts.admin_data;
    let user_data_account = &mut ctx.accounts.user_data;
    let program = &mut ctx.accounts.program_data;
    App::set_administrator(program, admin_data_account, user_data_account, status)?;
    Ok(())
}

pub fn _set_ambassador(ctx: Context<AdminSetter>, _user: Pubkey, status: bool) -> Result<()> {
    let admin_data_account = &mut ctx.accounts.admin_data;
    let user_data_account = &mut ctx.accounts.user_data;
    let program = &mut ctx.accounts.program_data;
    App::set_ambassador(program, admin_data_account, user_data_account, status)?;
    Ok(())
}

pub fn _set_staking_requirement(ctx: Context<Admin>, amount_of_tokens: u64) -> Result<()> {
    let user_data_account = &mut ctx.accounts.user_data;
    let program = &mut ctx.accounts.program_data;
    App::set_staking_requirement(program, user_data_account, amount_of_tokens)?;
    Ok(())
}

pub fn _set_name(ctx: Context<Admin>, new_name: String) -> Result<()> {
    let user_data_account = &mut ctx.accounts.user_data;
    let program = &mut ctx.accounts.program_data;
    App::set_name(program, user_data_account, new_name)?;
    Ok(())
}

pub fn _set_symbol(ctx: Context<Admin>, new_symbol: String) -> Result<()> {
    let user_data_account = &mut ctx.accounts.user_data;
    let program = &mut ctx.accounts.program_data;
    App::set_symbol(program, user_data_account, new_symbol)?;
    Ok(())
}

// READ ONLY INSTRUCTIONS
pub fn _my_dividends(ctx: Context<ReadOnly>, including_ref: bool) -> Result<u64> {
    let user_data_account = &ctx.accounts.user_data;
    let program = &mut ctx.accounts.program_data;
    let value = App::my_dividends(program, user_data_account, including_ref);
    Ok(value)
}

pub fn _sell_price(ctx: Context<ProgramReadOnly>) -> Result<u64> {
    let value = ctx.accounts.program_data.sell_price();
    Ok(value)
}

pub fn _buy_price(ctx: Context<ProgramReadOnly>) -> Result<u64> {
    let value = ctx.accounts.program_data.buy_price();
    Ok(value)
}

pub fn _calculate_lamports_received(ctx: Context<ProgramReadOnly>, tokens: u64) -> Result<u64> {
    let value = ctx
        .accounts
        .program_data
        .calculate_lamports_received(tokens);

    Ok(value)
}

pub fn _calculate_tokens_received(ctx: Context<ProgramReadOnly>, lamports: u64) -> Result<u64> {
    let value = ctx
        .accounts
        .program_data
        .calculate_tokens_received(lamports);
    Ok(value)
}
