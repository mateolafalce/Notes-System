use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn sell(
        ctx: Context<SellStruct>,
        product: String,
        description: String,
        supply: u64,
        price: u64,
        ipfs_url: String,
    ) -> Result<()> {
        require!(product.len() <= 27, ErrorCode::LenghtError);
        require!(description.len() <= 200, ErrorCode::LenghtError);
        require!(supply > 0, ErrorCode::U16Error);
        require!(price > 0, ErrorCode::PriceError);
        require!(ipfs_url.len() <= 900, ErrorCode::LenghtError);
        let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.main_account.offers.to_be_bytes().as_ref()], ctx.program_id);
        let sell: &mut Account<Sell> = &mut ctx.accounts.sell;
        sell.bump_original = bump;
        sell.pubkey = ctx.accounts.user.key();
        sell.seed = ctx.accounts.main_account.offers;
        sell.product = product;
        sell.description = description;
        sell.supply = supply;
        sell.price = price;
        sell.ipfs_url = ipfs_url;
        let main_account: &mut Account<MainAccount> = &mut ctx.accounts.main_account;
        main_account.offers += 1;
        Ok(())
    }

#[derive(Accounts)]
pub struct SellStruct<'info> {
    #[account(mut, seeds = [b"Main Account"], bump = main_account.bump_original)]
    pub main_account: Account<'info, MainAccount>,
    #[account(init, seeds = [main_account.offers.to_be_bytes().as_ref()], bump, payer = user, space = Sell::SIZE + 8)]
    pub sell: Account<'info, Sell>,
    /// CHECK: This is not dangerous
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}