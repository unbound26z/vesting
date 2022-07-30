use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program; //add bcs of constraint for sys_prog::ID
//use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, Mint, Token, TokenAccount, Transfer}};

<<<<<<< Updated upstream
//Drask ID: 2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ Danilo ID: DYWdbcaqeXrWqvbTHeRVPZdEuUkm7YUDBErMkE7FajJS
declare_id!("2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ");
=======
declare_id!("6d7MWPKaMnpFkPwU8trzvojcKF4bHm32WXzJiqhQuFXG");
>>>>>>> Stashed changes

#[program]
pub mod vesting {
    use super::*;
<<<<<<< Updated upstream

    pub fn make_vestment(ctx: Context<MakeVestment>,amount:u16,cliff:u16,period:u8) -> Result<()> {
        let vestment: &mut Account<Vestment> = &mut ctx.accounts.vestment;
        let vestor: &Signer = &ctx.accounts.vestor;
        let clock: Clock = Clock::get().unwrap();
    
        vestment.vestor = *vestor.key;
        vestment.timestamp = clock.unix_timestamp;
        vestment.amount = amount;
        vestment.cliff = cliff;
        vestment.period = period;
       
        Ok(())
    }


    // //TODO  
    // pub fn claim_vestment(ctx: Context<ClaimVestment>) -> Result<()> {
=======
    pub fn make_vestment(
        ctx: Context<MakeVestment>,
        amount: u64,
        cliff: Option<i64>,
        period: i64,
        num_of_periods: u32,
    ) -> Result<()> {
        let vestment_ledger = &mut ctx.accounts.vestment_ledger;
        let vestment: &mut Account<Vestment> = &mut ctx.accounts.vestment;
        let vestor: &Signer = &ctx.accounts.vestor;
        let vesting_start_at = Clock::get().unwrap().unix_timestamp;

        vestment_ledger.vestment_beneficiary = ctx.accounts.beneficiary.key();
        vestment_ledger.vestment_mint = ctx.accounts.vested_tokens_mint.key();
        vestment_ledger.vestment_count = vestment_ledger.vestment_count.checked_add(1).unwrap();
        vestment_ledger.active_vestment_count = vestment_ledger.active_vestment_count.checked_add(1).unwrap();

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



        if let Some(c) = cliff {
            vestment.cliff_end_at = Some(c);
            vestment.vesting_end_at = c
                .checked_add((num_of_periods as i64).checked_mul(period).unwrap())
                .unwrap();
        } else {
            vestment.cliff_end_at = None;
            vestment.vesting_end_at = vesting_start_at
                .checked_add((num_of_periods as i64).checked_mul(period).unwrap())
                .unwrap();
        } // Racuna kada je kraj vestmenta u odnosu na to da li postoji cliff

        

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
        let vestment_ledger = &mut ctx.accounts.vestment_ledger; 

        vestment_ledger.active_vestment_count = vestment_ledger.active_vestment_count.checked_sub(1).unwrap();
>>>>>>> Stashed changes


    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct MakeVestment<'info> {
<<<<<<< Updated upstream
    #[account(init,payer=vestor,space=Vestment::LEN)] //inits acc of the right size
    pub vestment: Account<'info,Vestment>, //parses from bits to vestment struct
=======
    #[account(
        init_if_needed,
        payer = vestor,
        seeds = [b"vestment", vested_tokens_mint.key().as_ref(), b"vestment-ledger", beneficiary.key().as_ref()],
        bump, 
        space = 8 + size_of::<VestmentLedger>()
    )]
    pub vestment_ledger: Account<'info, VestmentLedger>,

    #[account(
        init,
        payer = vestor,
        space = 8 + size_of::<Vestment>(),
        seeds = [b"vestment", vestment_ledger.key().as_ref(), &(vestment_ledger.vestment_count + 1).to_le_bytes()],
        bump
    )]
    //inits acc of the right size
    pub vestment: Account<'info, Vestment>, //parses from bits to vestment struct
>>>>>>> Stashed changes

    #[account(mut)] //mut to make the amount he has LESS
    pub vestor: Signer<'info>, //=AccountInfo but has to sign it too

<<<<<<< Updated upstream
    ///CHECK
=======
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
>>>>>>> Stashed changes
    #[account(address=system_program::ID)] //so its valid
    pub system_program: AccountInfo<'info>//accountInfo gives an accounts in BITS

}

<<<<<<< Updated upstream
// //TODO  
// #[derive(Accounts)]
// pub struct ClaimVestment<'info> {
//     #[account(mut)] //mut to make the amount he has HIGHER
//     pub vestor: Signer<'info>, //=AccountInfo  has to sign it too

//      ///CHECK
//      #[account(address=system_program::ID)] //so its valid
//      pub system_program: AccountInfo<'info>//accountInfo returns account in bits
// }
=======
//TODO
#[derive(Accounts)]
pub struct ClaimVestment<'info> {
    #[account(
        mut,
        seeds = [b"vestment", vested_tokens_mint.key().as_ref(), b"vestment-ledger", beneficiary.key().as_ref()],
        bump, 
    )]
    pub vestment_ledger: Account<'info, VestmentLedger>,
    
    #[account(
        mut, 
        seeds = [b"vestment", vested_tokens.key().as_ref()], 
        bump
    )]
    pub vestment: Account<'info, Vestment>,

    #[account(mut)] //mut to make the amount he has HIGHER
    pub beneficiary: Signer<'info>, //=AccountInfo has to sign it too

    #[account(mut)]
    pub beneficiary_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vested-tokens", beneficiary.key().as_ref()],
        bump,
    )]
    pub vested_tokens: Account<'info, TokenAccount>,
    pub vested_tokens_mint: Account<'info, Mint>, // mint
    pub token_program: Program<'info, Token>,
>>>>>>> Stashed changes


#[account]
<<<<<<< Updated upstream
pub struct Vestment {
    pub vestor: Pubkey, // whose vestment
    pub timestamp: i64, //beginning of the cliff fi.
    pub amount: u16, //enough?
    pub cliff: u16, //in days?
    pub period: u8, //when it unlocks the percent
    //pub procent: u8 //maybe
=======
pub struct VestmentLedger {
    pub vestment_count: u32,
    pub active_vestment_count: u32,
    pub vestment_mint: Pubkey,
    pub vestment_beneficiary: Pubkey,
   
}


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
    pub is_active: bool,
>>>>>>> Stashed changes
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const AMOUNT_LENGTH: usize = 2;
const CLIFF_LENGTH: usize = 2;
const PERIOD_LENGTH: usize= 1;

impl Vestment {
    const LEN: usize=DISCRIMINATOR_LENGTH+PUBLIC_KEY_LENGTH+TIMESTAMP_LENGTH+
    AMOUNT_LENGTH+CLIFF_LENGTH+PERIOD_LENGTH;
}

