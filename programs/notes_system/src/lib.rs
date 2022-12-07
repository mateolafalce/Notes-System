use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("Cm1A1eSrCVbKmpcM944cMgDMbWBsDq9c1tyD1sKwJD1G");

#[program]
pub mod notes_system {
    use super::*;
    pub fn school_main_account(ctx: Context<InitSchoolMainAccount>) -> Result<()> {
        instructions::school_main_account::school_main_account(ctx)
    }
    pub fn school_register(
        ctx: Context<SchoolRegister>,
        name: String,
        student_number: u64
    ) -> Result<()> {
        instructions::school_register::school_register(
            ctx,
            name,
            student_number
        )
    }
    pub fn student_register(
        ctx: Context<StudentRegister>,
        name: String,
        lastname: String,
        trimester: u8
    ) -> Result<()> {
        instructions::student_register::student_register(
            ctx,
            name,
            lastname,
            trimester
        )
    }
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
        instructions::school_notes::school_notes(
            ctx,
            philosophy,
            english,
            biology,
            physical,
            chemistry,
            mathematics,
            work_and_citizenship,
            deports,
        )
    }
}