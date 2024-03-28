use anchor_lang::prelude::*;

declare_id!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");

#[program]
pub mod hello_world {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let num_account = &mut ctx.accounts.num_account;
        num_account.num = 420;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    num_account: Account<'info, NumAccount>,
    rent: Sysvar<'info, Rent>,
}

#[account]
pub struct NumAccount {
    pub num: u64,
}