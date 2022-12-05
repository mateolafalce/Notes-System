use anchor_lang::prelude::*;

#[account]
pub struct MainAccount { 
    pub bump_original: u8,       // 1
    pub transactions: u64,       // 8
    pub offers: u64,             // 8
}

#[account]
pub struct Sell { 
    pub bump_original: u8,       // 1
    pub pubkey: Pubkey,          // 32
    pub seed: u64,               // 8 
    pub product: String,         // 4 + 27
    pub description: String,     // 4 + 200
    pub supply: u64,             // 4 + 8
    pub price: u64,              // 4 + 8
    pub ipfs_url: String,        // 4 + 900
}

impl Sell {
    pub const SIZE: usize =  1 + 32 + 8 + 4 + 27 + 4 + 200 + 4 + 8 + 4 + 8 + 4 + 900;
}

impl MainAccount {
    pub const SIZE: usize =  1 + 8 + 8;
}
