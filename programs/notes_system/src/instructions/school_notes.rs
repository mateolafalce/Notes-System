use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn school_notes(
    ctx: Context<StudentRegister>,
    note: i8,
) -> Result<()> {
    require!(ctx.accounts.user.key() == ctx.accounts.school.admin.key(), ErrorCode::AuthorityError);
    let student: &mut Account<StudentAccount> = &mut ctx.accounts.student;
    let school: &mut Account<SchoolAccount> = &mut ctx.accounts.school;
    let main_account: &mut Account<SchoolMainAccount> = &mut ctx.accounts.main_account;
    if (student.notes.len() >= 2 && student.average) {
        student.year += 1;
    }
    student.notes.push(note);
    Ok(())
}

#[derive(Accounts)]
pub struct StudentRegister<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, SchoolMainAccount>,
    #[account(mut, seeds = [school.seed.to_be_bytes().as_ref()], bump = school.bump_original)]
    pub school: Account<'info, SchoolAccount>,
    #[account(mut, seeds = [student.number.to_be_bytes().as_ref()], bump = student.bump_original)]
    pub student: Account<'info, StudentAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}