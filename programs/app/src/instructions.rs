use {crate::states::*, anchor_lang::prelude::*};

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let program = &mut ctx.accounts.program_data;
    let admin_account = &ctx.accounts.admin;
    let admin_data_account = &mut ctx.accounts.admin_data;
    // let program_info = program.to_account_info();
    program.initialize(admin_account, admin_data_account)
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, seeds=[b"users", admin.key().as_ref()], bump, payer = admin, space = 8 + User::MAXIMUM_SIZE)]
    pub admin_data: Account<'info, User>,
    #[account(init, seeds=[b"program"], bump, payer = admin, space = 8 + App::MAXIMUM_SIZE)]
    pub program_data: Account<'info, App>,
    pub system_program: Program<'info, System>,
}
