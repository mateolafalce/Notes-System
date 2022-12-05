use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;

pub fn main_account(
    ctx: Context<Create>
) -> Result<()> {
    let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
    let (_pda, bump) = Pubkey::find_program_address(&[b"Main Account"], ctx.program_id);
    main_account.bump_original = bump;
    main_account.transactions = 0;
    main_account.offers = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"Main Account"], bump, payer = user, space = MainAccount::SIZE + 8)]
    pub main_account: Account<'info, MainAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}