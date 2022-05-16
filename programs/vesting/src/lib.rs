use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program; //add bcs of constraint for sys_prog::ID
use anchor_spl::token::{Mint, Token, TokenAccount};

//Drask ID: 2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ Danilo ID: DYWdbcaqeXrWqvbTHeRVPZdEuUkm7YUDBErMkE7FajJS
declare_id!("2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ");

#[program]
pub mod vesting {
    use super::*;

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


    //     Ok(())
    // }
}

#[derive(Accounts)]
// #[instruction(escrow_bump: u8)]
pub struct MakeVestment<'info> {
    #[account(init,payer=vestor,space=Vestment::LEN)] //inits acc of the right size
    pub vestment: Account<'info,Vestment>, //parses from bits to vestment struct

    #[account(mut)] //mut to make the amount he has LESS
    pub vestor: Signer<'info>, //=AccountInfo but has to sign it too

    // #[account(mut, constraint = token_account.mint ==  kind_of_token.key())]
    // pub token_account: Account<'info, TokenAccount>,

    // #[account(
    //     init,
    //     payer = vestor,
    //     seeds = [vestment.key().as_ref()],
    //     bump = escrow_bump,
    //     token::mint = kind_of_token,
    //     token::authority = escrowed_tokens_of_vestor,
    // )]

    // pub escrowed_tokens_of_vestor: Account<'info, TokenAccount>,
    // pub kind_of_token: Account<'info, Mint>,

    // pub token_program: Program<'info, Token>,
    // pub rent: Sysvar<'info, Rent>,

    ///CHECK: Validated here.
    #[account(address=system_program::ID)] //so its valid
    pub system_program: AccountInfo<'info>,//accountInfo gives an accounts in BITS
    


}

// //TODO  
// #[derive(Accounts)]
// pub struct ClaimVestment<'info> {
//     #[account(mut)] //mut to make the amount he has HIGHER
//     pub vestor: Signer<'info>, //=AccountInfo  has to sign it too

//     #[account(mut)] //mut to make the amount he has HIGHER
//     pub target: AccountInfo<'info>, //=AccountInfo  has to sign it too

//      ///CHECK
//      #[account(address=system_program::ID)] //so its valid
//      pub system_program: AccountInfo<'info>//accountInfo returns account in bits
// }


#[account]
pub struct Vestment {
    pub vestor: Pubkey, // whose vestment
    pub timestamp: i64, //beginning of the cliff fi.
    pub amount: u16, //amount?
    pub cliff: u16, //in days? //drugi timestamp umesto ovoga
    pub period: u8, //when it unlocks the percent
    pub beneficiary: Pubkey // who gets the money

}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const AMOUNT_LENGTH: usize = 2;
const CLIFF_LENGTH: usize = 2;
const PERIOD_LENGTH: usize= 1;

impl Vestment {
    const LEN: usize=DISCRIMINATOR_LENGTH+PUBLIC_KEY_LENGTH+TIMESTAMP_LENGTH+
    AMOUNT_LENGTH+CLIFF_LENGTH+PERIOD_LENGTH+PUBLIC_KEY_LENGTH;
}

