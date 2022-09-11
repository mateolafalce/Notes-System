use anchor_lang::{
    prelude::*,
    solana_program::account_info::AccountInfo,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;
use oorandom;

declare_id!("CEBJ4Yj5z4dB3ahZBVTbo6kUmykh3HqibCjyNzRugUmj");

#[program]
pub mod solotery {
    use super::*;
    pub fn create_stake(
        ctx: Context<Create>
    ) -> Result<()> {
        require!(ctx.accounts.user.key() == Pubkey::from_str("NqXx91Lk2qn9V1W3kEvBVmp1fzXLEGTSGkd9yungtk8").unwrap(), ErrorCode::YouAreNotSOLotery);
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        let (_stake_pda, bump) = Pubkey::find_program_address(&[b"SOLotery", ctx.accounts.user.key().as_ref()], &Pubkey::from_str("CEBJ4Yj5z4dB3ahZBVTbo6kUmykh3HqibCjyNzRugUmj").unwrap());
        solotery.authority = ctx.accounts.user.key();
        solotery.choose_winner_only_one_time = 0;
        solotery.bump_original = bump;
        solotery.secure_check = 1662260159;
        solotery.players = [].to_vec();
        solotery.best_proposal1 = 5000;
        solotery.best_proposal2 = 5000;
        solotery.best_proposal3 = 5000;
        solotery.best_proposal4 = 5000;
        Ok(())
    }
    pub fn ticket(
        ctx: Context<Ticket>
    ) -> Result<()> {
        require!(ctx.accounts.solotery.players.len() != 300, ErrorCode::Limit);
        require!(ctx.accounts.stake.key() == ctx.accounts.solotery.key(), ErrorCode::WrongStake);
        require!(ctx.accounts.solotery.choose_winner_only_one_time != 1, ErrorCode::JustOnce);
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.solotery.key(), 7777777),
            &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],).expect("Error");
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        solotery.players.push(ctx.accounts.from.key());
        Ok(())
    }
    pub fn buy_share(
        ctx: Context<BuyShare>,
        share_number: u8,
        amount: u64
    ) -> Result<()> {
        let creator: Pubkey = Pubkey::from_str("HiHyNHnwSzFprUdMERFbUJNTcuoD375oE6qDfuytmUYL").unwrap();
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        require!(ctx.accounts.creator.key() == creator, ErrorCode::WrongStake);
        if share_number == 1 {
            require!(amount > solotery.best_proposal1, ErrorCode::AmountError);
            anchor_lang::solana_program::program::invoke(
                &system_instruction::transfer(&ctx.accounts.from.key(), &creator, amount),
                &[ctx.accounts.from.to_account_info(), ctx.accounts.creator.to_account_info().clone()],).expect("Error");
                solotery.best_proposal1 = amount;
                solotery.owner1 = ctx.accounts.from.key();
        }
        if share_number == 2 {
            require!(amount > solotery.best_proposal2, ErrorCode::AmountError);
            anchor_lang::solana_program::program::invoke(
                &system_instruction::transfer(&ctx.accounts.from.key(), &creator, amount),
                &[ctx.accounts.from.to_account_info(), ctx.accounts.creator.to_account_info().clone()],).expect("Error");
                solotery.best_proposal2 = amount;
                solotery.owner2 = ctx.accounts.from.key();
        }
        if share_number == 3 {
            require!(amount > solotery.best_proposal3, ErrorCode::AmountError);
            anchor_lang::solana_program::program::invoke(
                &system_instruction::transfer(&ctx.accounts.from.key(), &creator, amount),
                &[ctx.accounts.from.to_account_info(), ctx.accounts.creator.to_account_info().clone()],).expect("Error");
                solotery.best_proposal3 = amount;
                solotery.owner3 = ctx.accounts.from.key();
        }
        if share_number == 4 {
            require!(amount > solotery.best_proposal4, ErrorCode::AmountError);
            anchor_lang::solana_program::program::invoke(
                &system_instruction::transfer(&ctx.accounts.from.key(), &creator, amount),
                &[ctx.accounts.from.to_account_info(), ctx.accounts.creator.to_account_info().clone()],).expect("Error");
                solotery.best_proposal4 = amount;
                solotery.owner4 = ctx.accounts.from.key();
        }
        Ok(())
    }
    pub fn choose_winner(
        ctx: Context<Winner>
    ) -> Result<()> {
        require!(ctx.accounts.solotery_authority.key() == Pubkey::from_str("NqXx91Lk2qn9V1W3kEvBVmp1fzXLEGTSGkd9yungtk8").unwrap(), ErrorCode::YouAreNotSOLotery); 
        require!(ctx.accounts.solotery.choose_winner_only_one_time != 1, ErrorCode::JustOnce);
        require!(Clock::get().unwrap().unix_timestamp > ctx.accounts.solotery.secure_check, IncorrectTimestamp);
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
        solotery.choose_winner_only_one_time += 1;
        if solotery.players.len() == 0 {
            solotery.secure_check += 86398;
            solotery.choose_winner_only_one_time -= 1;
        }
        if solotery.players.len() > 0 {
            let mut rng: oorandom::Rand64 = oorandom::Rand64::new((Clock::get().unwrap().unix_timestamp as u64).into());
            let winner: usize = rng.rand_range(1..(solotery.players.len() as u64)).try_into().unwrap();
            solotery.winner_publickey =  solotery.players[winner - 1];
        }
        Ok(())
    }
    pub fn send_amount_to_winner(
        ctx: Context<SendAmountToWinner>
    ) -> Result<()> {
        require!(ctx.accounts.solotery.choose_winner_only_one_time != 0, ErrorCode::JustOnce);
        require!(ctx.accounts.winner_publickey.key() == ctx.accounts.solotery.winner_publickey, ErrorCode::ThisIsNotTheWinner);
        require!(ctx.accounts.solotery_authority.key() == Pubkey::from_str("NqXx91Lk2qn9V1W3kEvBVmp1fzXLEGTSGkd9yungtk8").unwrap(), ErrorCode::YouAreNotSOLotery);
        require!(Clock::get().unwrap().unix_timestamp > ctx.accounts.solotery.secure_check, ErrorCode::IncorrectTimestamp);
        let solotery: &mut Account<SoLotery> = &mut ctx.accounts.solotery;
    
        let winner: &mut AccountInfo = &mut ctx.accounts.winner_publickey;
        fn to_f64(amount: u64) -> f64 {return amount as f64}
        fn percent(amount: f64) -> u64 {((amount / 100.0)* 2.0).round() as u64}  
        let owners_amount = percent(to_f64(AccountInfo::lamports(&solotery.to_account_info()) - 69419050));
        let owners_dividend: u64 = owners_amount / 4; 
        let winner_reward: u64 = AccountInfo::lamports(&solotery.to_account_info()) - 69419050 - owners_amount; 
        **solotery.to_account_info().try_borrow_mut_lamports()? -= owners_dividend;
        **ctx.accounts.owner1.try_borrow_mut_lamports()? += owners_dividend;
        **solotery.to_account_info().try_borrow_mut_lamports()? -= owners_dividend;
        **ctx.accounts.owner2.try_borrow_mut_lamports()? += owners_dividend;
        **solotery.to_account_info().try_borrow_mut_lamports()? -= owners_dividend;
        **ctx.accounts.owner3.try_borrow_mut_lamports()? += owners_dividend;
        **solotery.to_account_info().try_borrow_mut_lamports()? -= owners_dividend;
        **ctx.accounts.owner4.try_borrow_mut_lamports()? += owners_dividend;
        **solotery.to_account_info().try_borrow_mut_lamports()? -= winner_reward;
        **winner.to_account_info().try_borrow_mut_lamports()? += winner_reward;
        solotery.choose_winner_only_one_time = solotery.choose_winner_only_one_time - 1;
        solotery.players = [].to_vec();
        solotery.winner_publickey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        solotery.secure_check += 86398;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"SOLotery", user.key().as_ref()], bump, payer = user, space = 9846)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Ticket<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct BuyShare<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut, signer)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub creator: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Winner<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    #[account(mut)]
    pub solotery_authority: Signer<'info>
}
#[derive(Accounts)]
pub struct SendAmountToWinner<'info> {
    #[account(mut, seeds = [b"SOLotery", solotery.authority.key().as_ref()], bump = solotery.bump_original)]
    pub solotery: Account<'info, SoLotery>,
    /// CHECK: This is not dangerous because we don't read or write from this accoun
    #[account(mut)]
    pub owner1: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this accoun
    #[account(mut)]
    pub owner2: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this accoun
    #[account(mut)]
    pub owner3: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this accoun
    #[account(mut)]
    pub owner4: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this accoun
    #[account(mut)]
    pub winner_publickey: AccountInfo<'info>,
    #[account(mut)]
    pub solotery_authority: Signer<'info>,
}
#[account]
pub struct SoLotery {
    pub authority: Pubkey, 
    pub bump_original: u8, 
    pub winner_publickey: Pubkey, 
    pub choose_winner_only_one_time: u8, 
    pub secure_check: i64, 
    pub players: Vec<Pubkey>, // 300 players
    pub owner1: Pubkey,
    pub best_proposal1: u64,
    pub owner2: Pubkey,
    pub best_proposal2: u64,
    pub owner3: Pubkey,
    pub best_proposal3: u64,
    pub owner4: Pubkey,
    pub best_proposal4: u64
}
#[error_code]
pub enum ErrorCode {
    #[msg("The winner can only be chosen once")]JustOnce, #[msg("You are not SOLotery key")]YouAreNotSOLotery, 
    #[msg("This is not the winner")]ThisIsNotTheWinner, #[msg("This is not the stake account")]WrongStake, 
    #[msg("No winner has been chosen")]NoWinner, #[msg("The player limit is 300")]Limit,
    #[msg("19:00 Argentine time")]IncorrectTimestamp, #[msg("You are not a owner")]OwnerError, 
    #[msg("Your proposal is not superior to the existing one")]AmountError, 
}