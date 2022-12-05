use anchor_lang::prelude::*;
use instructions::*;
use crate::errors::ErrorCode;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("Cm1A1eSrCVbKmpcM944cMgDMbWBsDq9c1tyD1sKwJD1G");

#[program]
pub mod commonmarket {
    use super::*;
    pub fn main_account(ctx: Context<Create>) -> Result<()> {
        instructions::main_account::main_account(ctx)
    }
    pub fn sell(
        ctx: Context<SellStruct>,
        product: String,
        description: String,
        supply: u64,
        price: u64,
        ipfs_url: String
    ) -> Result<()> {
        instructions::sell::sell(
            ctx,
            product,
            description,
            supply,
            price,
            ipfs_url
        )
    }
    pub fn exchange(
        ctx: Context<Exchange>,
        supply: u64
    ) -> Result<()> {
        instructions::exchange::exchange(
            ctx,
            supply
        )
    }
}