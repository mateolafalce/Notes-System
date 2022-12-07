use anchor_lang::prelude::*;

#[account]
pub struct SchoolMainAccount { 
    pub bump_original: u8,         // 1
    pub total_schools: u64,        // 8
    pub total_students: u64,       // 8
}

#[account]
pub struct SchoolAccount { 
    pub bump_original: u8,          // 1
    pub admin: Pubkey,              // 32
    pub name: String,               // 4 + 50
    pub seed: u64,                  // 8
    pub student_number: u64,        // 8
    pub total_students: u64,        // 8
}

#[account]
pub struct StudentAccount { 
    pub bump_original: u8,          // 1
    pub name: String,               // 4 + 50
    pub lastname: String,           // 4 + 50
    pub number: u64,                // 8
    pub year: u8                    // 1
    pub notes: Vec<i8>              // 4 + 1
}

impl StudentAccount {
    pub const SIZE: usize =  1 + 32 + 4 + 50 + 4 + 50 + 8 + 1 + 4 + 1;
}

impl SchoolAccount {
    pub const SIZE: usize =  1 + 32 + 4 + 50 + 8 + 8 + 8;
}

impl SchoolMainAccount {
    pub const SIZE: usize =  1 + 8 + 8;
}
