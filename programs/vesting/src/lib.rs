use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program; //add bcs of constraint for sys_prog::ID
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::mem::size_of;

declare_id!("2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ");

#[program]
pub mod vesting {
    use super::*;
    pub fn make_vestment(
        ctx: Context<MakeVestment>,
        amount: u64,
        cliff: Option<i64>,
        period: i64,
        num_of_periods: u32,
    ) -> Result<()> {
        let vestment: &mut Account<Vestment> = &mut ctx.accounts.vestment;
        let vestor: &Signer = &ctx.accounts.vestor;
        let vesting_start_at = Clock::get().unwrap().unix_timestamp;
        let ledger = &mut ctx.accounts.ledger;

        ledger.vestment_beneficiary = ctx.accounts.beneficiary.key();
        ledger.vestment_mint = ctx.accounts.vested_tokens_mint.key();
        ledger.vestment_count= ledger.vestment_count.checked_add(1).unwrap();

        if amount <=0 { 
            return Err(ErrorCode::InvalidAmount.into());
        }
        if let Some(c) = cliff {
            if c<=0 {
            return Err(ErrorCode::InvalidCliff.into());
            }
        }
        if period<=0 {
            return Err(ErrorCode::InvalidPeriod.into());
        }
        if num_of_periods <=0 { 
            return Err(ErrorCode::InvalidNumberOfPeriods.into());
        }


        vestment.vestor = vestor.key();
        vestment.vesting_start_at = vesting_start_at;
        vestment.amount_vested = amount*1000000000;
        vestment.amount_claimed = 0;
        vestment.period_length = period;
        vestment.num_of_periods = num_of_periods;
        vestment.beneficiary = ctx.accounts.beneficiary.key();
        vestment.last_claim_period = None;
        vestment.amount_per_period = vestment.amount_vested.checked_div(vestment.num_of_periods as u64).unwrap();
        vestment.is_active = true;

        if let Some(c) = cliff {
            vestment.cliff_end_at = vestment.vesting_start_at.checked_add(c);
            vestment.vesting_end_at = vestment.cliff_end_at.unwrap()
                .checked_add((num_of_periods as i64).checked_mul(period).unwrap())
                .unwrap();
        } else {
            vestment.cliff_end_at = None;
            vestment.vesting_end_at = vesting_start_at
                .checked_add((num_of_periods as i64).checked_mul(period).unwrap())
                .unwrap();
        }

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.vestor_token_account.to_account_info(),
                    to: ctx.accounts.vested_tokens.to_account_info(),
                    authority: ctx.accounts.vestor.to_account_info(),
                },
            ),
            amount*1000000000 as u64,
        )?;

        Ok(())
    }

    pub fn claim_vestment(ctx: Context<ClaimVestment>) -> Result<()> {
        let vestment = &mut ctx.accounts.vestment;
        let claim_time = Clock::get().unwrap().unix_timestamp;
        let mut amount_to_claim: Box<u64> = Box::new(0);


        if vestment.vesting_end_at >= claim_time {
       
        let mut num_of_claim_periods = Box::new(0);
        if let Some(last_claim_period) = vestment.last_claim_period {
            *num_of_claim_periods = claim_time.checked_sub(last_claim_period).unwrap().checked_div(vestment.period_length).unwrap();
            vestment.last_claim_period = (num_of_claim_periods.checked_mul(vestment.period_length).unwrap()).checked_add(last_claim_period);
        } else {
            if let Some(c) = vestment.cliff_end_at {
                *num_of_claim_periods = claim_time.checked_sub(c).unwrap().checked_div(vestment.period_length).unwrap();
                vestment.last_claim_period = num_of_claim_periods.checked_mul(vestment.period_length).unwrap().checked_add(c);

            } else {
                *num_of_claim_periods = (claim_time.checked_sub(vestment.vesting_start_at).unwrap()).checked_div(vestment.period_length).unwrap();
                vestment.last_claim_period = (num_of_claim_periods.checked_mul(vestment.period_length).unwrap()).checked_add(vestment.vesting_start_at);
            }
        }
        *amount_to_claim = (*num_of_claim_periods as u64).checked_mul(vestment.amount_per_period).unwrap();
    } else  {
        *amount_to_claim = vestment.amount_vested.checked_sub(vestment.amount_claimed).unwrap();
    }

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.vested_tokens.to_account_info(),
                    to: ctx.accounts.beneficiary_token_account.to_account_info(),
                    authority: ctx.accounts.vested_tokens.to_account_info(),
                },
                &[&[
                    b"vested-tokens",
                    ctx.accounts.beneficiary.key().as_ref(),
                    &[*ctx.bumps.get("vested_tokens").unwrap()],
                ]],
            ),
            *amount_to_claim,
        )?;

        vestment.amount_claimed = vestment.amount_claimed.checked_add(*amount_to_claim).unwrap();

        if vestment.amount_vested-vestment.amount_claimed=0 {
            vestment.is_active=false;
        }
       
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MakeVestment<'info> {
    #[account(
        init_if_needed,
        payer = vestor,
        seeds = [b"vestment", vested_tokens_mint.key().as_ref(), b"ledger", beneficiary.key().as_ref()],
        bump, 
        space = 8 + size_of::<Ledger>()
    )]
    pub ledger: Account<'info, Ledger>,

    #[account(
        init,
        payer = vestor,
        space = 8 + size_of::<Vestment>(),
        seeds = [b"vestment", ledger.key().as_ref(), &(ledger.vestment_count+1).to_le_bytes()],
        bump
    )]
    //inits acc of the right size
    pub vestment: Account<'info, Vestment>, //parses from bits to vestment struct

    #[account(mut)] //mut to make the amount he has LESS
    pub vestor: Signer<'info>, //=AccountInfo but has to sign it too

    #[account(mut)]
    pub vestor_token_account: Account<'info, TokenAccount>,

    #[account()]
    /// CHECK: TODO
    pub beneficiary: AccountInfo<'info>,

    #[account(
        init,
        payer = vestor,
        seeds = [b"vested-tokens", vestment.key().as_ref()],
        bump,
        token::mint = vested_tokens_mint,
        token::authority = vested_tokens,
    )]
    pub vested_tokens: Account<'info, TokenAccount>,
    pub vested_tokens_mint: Account<'info, Mint>, // mint

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,

    ///CHECK: Validated here.
    #[account(address=system_program::ID)] //so its valid
    pub system_program: AccountInfo<'info>, //accountInfo gives an accounts in BITS
}

//TODO
#[derive(Accounts)]
pub struct ClaimVestment<'info> {
    #[account(
        mut,
        seeds = [b"vestment", vested_tokens_mint.key().as_ref(), b"ledger", beneficiary.key().as_ref()],
        bump
    )]
    pub ledger: Account<'info, Ledger>,

    #[account(
        mut, 
        seeds = [b"vestment", ledger.key().as_ref(), &(ledger.vestment_count).to_le_bytes()], 
        bump
    )]
    pub vestment: Account<'info, Vestment>,

    #[account(mut)] //mut to make the amount he has HIGHER
    pub beneficiary: Signer<'info>, //=AccountInfo has to sign it too

    #[account(mut)]
    pub beneficiary_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vested-tokens", vestment.key().as_ref()],
        bump,
    )]
    pub vested_tokens: Account<'info, TokenAccount>,
    // pub vested_tokens_mint: Account<'info, Mint>, // mint
    pub token_program: Program<'info, Token>,

    ///CHECK
    #[account(address=system_program::ID)] //so its valid
    pub system_program: AccountInfo<'info>, //accountInfo returns account in bits
}

///c
#[account]
#[derive(Default)] //needed?
pub struct Vestment {
    pub vestor: Pubkey,        // whose vestment
    pub vesting_start_at: i64, //start time
    pub amount_vested: u64, // amount
    pub amount_claimed: u64, //amount to claim
    pub period_length: i64, //when it unlocks the percent
    pub num_of_periods: u32, 
    pub beneficiary: Pubkey,       // who gets the money
    pub cliff_end_at: Option<i64>, // if entered it represents the amount of days for cliff to end
    pub last_claim_period: Option<i64>,
    pub vesting_end_at: i64,
    pub amount_per_period: u64,
    pub is_active: bool
}

#[account]
pub struct Ledger {
    pub vestment_count: u32, 
    pub vestment_mint: Pubkey,
    pub vestment_beneficiary: Pubkey
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InvalidAmount,
    #[msg("The cliff length provided is not valid")]
    InvalidCliff,
    #[msg("The period length provided is not valid")]
    InvalidPeriod,
    #[msg("The number of periods is not valid")]
    InvalidNumberOfPeriods,
    
}

// const DISCRIMINATOR_LENGTH: usize = 8;
// const PUBLIC_KEY_LENGTH: usize = 32;
// const TIMESTAMP_LENGTH: usize = 8;
// const AMOUNT_LENGTH: usize = 8;
// const PERIOD_LENGTH: usize = 1;

// impl Vestment {
//     const LEN: usize = DISCRIMINATOR_LENGTH
//         + PUBLIC_KEY_LENGTH
//         + TIMESTAMP_LENGTH
//         + TIMESTAMP_LENGTH //ZA OPTION<I64> ??????
//         + AMOUNT_LENGTH
//         + AMOUNT_LENGTH
//         + PERIOD_LENGTH
//         + PUBLIC_KEY_LENGTH
//         + PERIOD_LENGTH
//         + AMOUNT_LENGTH+2;
// }
