use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program; //add bcs of constraint for sys_prog::ID
use anchor_spl::token::{Mint, Token, TokenAccount};

//Drask ID: 2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ Danilo ID: DYWdbcaqeXrWqvbTHeRVPZdEuUkm7YUDBErMkE7FajJS
declare_id!("2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ");

#[program]
pub mod vesting {
    use anchor_lang::solana_program::system_instruction;
    use anchor_spl::token::{self, Transfer};

    use super::*;

    pub fn make_vestment(
        ctx: Context<MakeVestment>,
        amount: u64,
        cliff: u16,
        period: u8,
        num_of_periods: u8,
    ) -> Result<()> {
        let vestment: &mut Account<Vestment> = &mut ctx.accounts.vestment;
        let vestor: &Signer = &ctx.accounts.vestor;
        let clock: Clock = Clock::get().unwrap();

        vestment.vestor = *vestor.key;
        //vestment.timestamp = clock.unix_timestamp + ((cliff * 24 * 60 * 60) as i64); //endtime calculation
        vestment.timestamp = clock.unix_timestamp; //trenutak pravljenja vestmenta
        vestment.amount = amount;
        vestment.period = period;
        vestment.num_of_periods = num_of_periods;
        vestment.beneficiary = ctx.accounts.beneficiary.key(); // is this ok?
        vestment.bump = *ctx.bumps.get("vestment").unwrap(); // for the bump ??

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.vestor_token_account.to_account_info(),
                    to: ctx.accounts.vested_tokens.to_account_info(),
                    authority: ctx.accounts.vestor.to_account_info(),
                },
            ),
            amount as u64,
        )?;

        // vesting::cpi::set_data(
        //     ctx.accounts.set_data_ctx().with_signer(&[&[bump][..]]),
        //     data,
        // );  //sta je ovo i jel treba

        Ok(())
    }

    //TODO
    pub fn claim_vestment(ctx: Context<ClaimVestment>, amount_per_period: u64) -> Result<()> {
        //TODO: HOW TO CONNECT THE CLAIM VESTMENT BUTTON TO THE CREATED VESTMENT PDA ACCOUNT
        //WHEN USING PUBKEY AS SEED

        let vestment: &mut Account<Vestment> = &mut ctx.accounts.vestment;
        // let clock: Clock = Clock::get().unwrap();
        // let amount_per_payment: u16 = vestment.amount/ (vestment.num_of_periods as u16 + 1 as u16);

        // if (vestment.claim_counter <= vestment.num_of_periods) {
        //     // check if claim counter is ok

        //     //calculate if and how much to claim

        //     if vestment.timestamp
        //         + ((vestment.period * 24 * 60 * 60 * vestment.claim_counter) as i64)
        //         < clock.unix_timestamp
        //     {
        //         //TODO
        //         //isplata

        //         vestment.claim_counter += 1; //if claimed will increment counter so that next time will move up a period in seconds
        //     } else {
        //         //obavesti da nije prosao period
        //     }
        // } else {
        //     //delete vestment??
        // }

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
            amount_per_period,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
//#[instruction(bump: u8)]    //needed?
pub struct MakeVestment<'info> {
    #[account(init,payer=vestor,space=Vestment::LEN,seeds=[b"vestment",vested_tokens.key().as_ref()],bump)]
    //inits acc of the right size
    pub vestment: Account<'info, Vestment>, //parses from bits to vestment struct

    #[account(mut)] //mut to make the amount he has LESS
    pub vestor: Signer<'info>, //=AccountInfo but has to sign it too

    #[account(mut)]
    pub vestor_token_account: Account<'info, TokenAccount>,
    #[account()]
    /// CHECK: TODO
    pub beneficiary: AccountInfo<'info>,

    // #[account(mut, constraint = token_account.mint == kind_of_token.key())]
    // pub token_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = vestor,
        seeds = [b"vested-tokens",beneficiary.key().as_ref()],
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
    #[account(mut, seeds = [b"vestment", vested_tokens.key().as_ref()], bump = vestment.bump)]
    pub vestment: Account<'info, Vestment>,

    #[account(mut)] //mut to make the amount he has HIGHER
    pub beneficiary: Signer<'info>, //=AccountInfo  has to sign it too

    #[account(mut)]
    pub beneficiary_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vested-tokens",beneficiary.key().as_ref()],
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
    pub vestor: Pubkey,      // whose vestment
    pub timestamp: i64,      //endtime
    pub amount: u64,         //amount?
    pub period: u8,          //when it unlocks the percent
    pub beneficiary: Pubkey, // who gets the money
    pub bump: u8,            // za pda
    pub num_of_periods: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const AMOUNT_LENGTH: usize = 8;
const PERIOD_LENGTH: usize = 1;

impl Vestment {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + TIMESTAMP_LENGTH
        + AMOUNT_LENGTH
        + PERIOD_LENGTH
        + PUBLIC_KEY_LENGTH
        + PERIOD_LENGTH
        + PERIOD_LENGTH;
}
