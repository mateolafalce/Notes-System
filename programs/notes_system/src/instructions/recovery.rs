use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn school_notes(
    ctx: Context<SchoolNotes>,
    subject: u8,
) -> Result<()> {
    require!(ctx.accounts.user.key() == ctx.accounts.school.admin.key(), ErrorCode::AuthorityError);
    let student: &mut Account<StudentAccount> = &mut ctx.accounts.student;
    let notes: &mut Account<NotesAccount> = &mut ctx.accounts.notes;
    notes.philosophy.push(philosophy);
    Ok(())
}

#[derive(Accounts)]
pub struct SchoolNotes<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, SchoolMainAccount>,
    #[account(mut, seeds = [school.seed.to_be_bytes().as_ref()], bump = school.bump_original)]
    pub school: Account<'info, SchoolAccount>,
    #[account(mut, seeds = [student.number.to_be_bytes().as_ref()], bump = student.bump_original)]
    pub student: Account<'info, StudentAccount>,
    #[account(init, seeds = [school.student_number.to_be_bytes().as_ref(), student.trimester.to_be_bytes().as_ref()], bump, payer = user, space = NotesAccount::SIZE + 8)]
    pub notes: Account<'info, NotesAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}