use {crate::constants::*, crate::states::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, seeds=[USER_SEED, admin.key().as_ref()], bump, payer = admin, space = 8 + User::MAXIMUM_SIZE)]
    pub admin_data: Account<'info, User>,
    #[account(init, seeds=[PROGRAM_SEED], bump, payer = admin, space = 8 + App::MAXIMUM_SIZE)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(lamports_to_send: u64, referred_by: Option<Pubkey>,)]
pub struct Buy<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,

    #[account(init_if_needed, seeds=[USER_SEED, referred_by.unwrap().key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub referred_by_data: Option<Account<'info, User>>,

    #[account(mut)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Reinvest<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,

    #[account(mut)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Exit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,

    #[account(mut)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(to: Pubkey)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,

    #[account(init_if_needed, seeds=[USER_SEED, to.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub to_data: Account<'info, User>,
    #[account(mut)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,

    #[account(mut)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,

    #[account(mut)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Admin<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, seeds=[USER_SEED, user.key().as_ref()], bump, payer = user, space = 8 + User::MAXIMUM_SIZE)]
    pub user_data: Account<'info, User>,

    #[account(mut)]
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

    #[account(mut)]
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

pub fn _initialize(ctx: Context<Initialize>) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let admin_account = &ctx.accounts.admin;
    let admin_data_account = &mut ctx.accounts.admin_data;

    App::initialize(program, admin_account, admin_data_account)?;
    Ok(())
}

pub fn _buy(ctx: Context<Buy>, lamports_to_send: u64, referred_by: Option<Pubkey>) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let buyer = &mut ctx.accounts.user;
    let buyer_data_account = &mut ctx.accounts.user_data;
    let referred_by_data_account = &mut ctx.accounts.referred_by_data;
    let system_program = ctx.accounts.system_program.to_account_info();

    App::buy(
        program,
        buyer,
        buyer_data_account,
        referred_by_data_account,
        lamports_to_send,
        referred_by,
        system_program,
    )?;
    Ok(())
}

pub fn _reinvest(ctx: Context<Reinvest>) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let user = &mut ctx.accounts.user;
    let user_data_account = &mut ctx.accounts.user_data;
    let system_program = ctx.accounts.system_program.to_account_info();

    App::reinvest(program, user, user_data_account, system_program.clone())?;
    Ok(())
}

pub fn _exit(ctx: Context<Exit>) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let user = &mut ctx.accounts.user;
    let user_data_account = &mut ctx.accounts.user_data;

    App::leave(program, user, user_data_account)?;
    Ok(())
}

pub fn _transfer(ctx: Context<Transfer>, to: Pubkey, lamports_to_send: u64) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let user = &mut ctx.accounts.user;
    let user_data_account = &mut ctx.accounts.user_data;
    let to_data_account = &mut ctx.accounts.to_data;
    App::transfer(
        program,
        user,
        user_data_account,
        to,
        to_data_account,
        lamports_to_send,
    )?;
    Ok(())
}

pub fn _withdraw(ctx: Context<Withdraw>) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let user = &mut ctx.accounts.user;
    let user_data_account = &mut ctx.accounts.user_data;

    App::withdraw(program, user, user_data_account)?;
    Ok(())
}

pub fn _sell(ctx: Context<Sell>, lamports_to_send: u64) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let user = &ctx.accounts.user;
    let user_data_account = &mut ctx.accounts.user_data;

    App::sell(program, user, user_data_account, lamports_to_send)?;
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
