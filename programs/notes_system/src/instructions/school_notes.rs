use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn school_notes(
    ctx: Context<SchoolNotes>,
    philosophy: i8,
    english: i8,
    biology: i8,
    physical: i8,
    chemistry: i8,
    mathematics: i8,
    work_and_citizenship: i8,
    deports: i8,
) -> Result<()> {
    require!(ctx.accounts.user.key() == ctx.accounts.school.admin.key(), ErrorCode::AuthorityError);
    let student: &mut Account<StudentAccount> = &mut ctx.accounts.student;
    let notes: &mut Account<NotesAccount> = &mut ctx.accounts.notes;
    notes.philosophy.push(philosophy);
    notes.english.push(english);
    notes.biology.push(biology);
    notes.physical.push(physical);
    notes.chemistry.push(chemistry);
    notes.mathematics.push(mathematics);
    notes.work_and_citizenship.push(work_and_citizenship);
    notes.deports.push(deports);
    student.trimester += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct SchoolNotes<'info> {
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