use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program; //add bcs of constraint for sys_prog::ID
use anchor_spl::token::{Mint, Token, TokenAccount};

//Drask ID: 2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ Danilo ID: DYWdbcaqeXrWqvbTHeRVPZdEuUkm7YUDBErMkE7FajJS
declare_id!("2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ");

#[program]
pub mod vesting {
    use anchor_lang::solana_program::system_instruction;

    use super::*;

    pub fn make_vestment(ctx: Context<MakeVestment>,amount:u16,cliff:u16,period:u8,beneficiary:Pubkey,num_of_periods:u8) -> Result<()> {
        let vestment: &mut Account<Vestment> = &mut ctx.accounts.vestment;
        let vestor: &Signer = &ctx.accounts.vestor;
        let clock: Clock = Clock::get().unwrap();
        
    
        vestment.vestor = *vestor.key;
        vestment.timestamp = clock.unix_timestamp + ((cliff*24*60*60) as i64); //endtime calculation
        vestment.amount = amount;
        vestment.period = period;
        vestment.num_of_periods=num_of_periods;
        vestment.beneficiary=beneficiary.key(); // is this ok?
        vestment.bump =*ctx.bumps.get("vestment").unwrap(); // for the bump ??
        vestment.claim_counter=0;



        // vesting::cpi::set_data(
        //     ctx.accounts.set_data_ctx().with_signer(&[&[bump][..]]),
        //     data,
        // );  //sta je ovo i jel treba 

        Ok(())
    }


    // //TODO  
    pub fn claim_vestment(ctx: Context<ClaimVestment>) -> Result<()> {
       //TODO: HOW TO CONNECT THE CLAIM VESTMENT BUTTON TO THE CREATED VESTMENT PDA ACCOUNT
       //WHEN USING PUBKEY AS SEED
        let vestment: &mut Account<Vestment> = &mut ctx.accounts.vestment;
        let clock: Clock = Clock::get().unwrap();
        let amount_per_payment: u16 = vestment.amount/ (vestment.num_of_periods as u16 + 1 as u16);

        if (vestment.claim_counter <= vestment.num_of_periods) { // check if claim counter is ok
             
            //calculate if and how much to claim

            if vestment.timestamp + ((vestment.period*24*60*60 * vestment.claim_counter) as i64 ) < clock.unix_timestamp {
                //TODO
                //isplata
                system_instruction::transfer(&vestment.vestor,&ctx.accounts.target.key,amount_per_payment as u64);
    
                vestment.claim_counter+=1; //if claimed will increment counter so that next time will move up a period in seconds
    
    
            } else {
                //obavesti da nije prosao period
            }


        } else {
            //delete vestment??
        }
        
        
       

        Ok(())
    }
}


#[derive(Accounts)]
//#[instruction(bump: u8)]    //needed?
 pub struct MakeVestment<'info> {
    #[account(init,payer=vestor,space=Vestment::LEN,seeds=[b"vestment",vestor.key().as_ref()],bump)] //inits acc of the right size
    pub vestment: Account<'info,Vestment>, //parses from bits to vestment struct

    #[account(mut)] //mut to make the amount he has LESS
    pub vestor: Signer<'info>, //=AccountInfo but has to sign it too

    #[account(mut, constraint = token_account.mint == kind_of_token.key())]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = vestor,
        seeds = [b"token_account",vestor.key().as_ref()],
        bump,
        token::mint = kind_of_token,
        token::authority = tokens_in_vestment,
    )]

    pub tokens_in_vestment: Account<'info, TokenAccount>,
    pub kind_of_token: Account<'info, Mint>, // mint

    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,

    ///CHECK: Validated here.
    #[account(address=system_program::ID)] //so its valid
    pub system_program: AccountInfo<'info>,//accountInfo gives an accounts in BITS
    


}


//TODO  
#[derive(Accounts)] 
pub struct ClaimVestment<'info> {
    #[account(mut, seeds = [b"vestment", vestor.key().as_ref()], bump = vestment.bump)]
    pub vestment: Account<'info,Vestment>,
   
    #[account(mut)] //mut to make the amount he has HIGHER
    pub vestor: Signer<'info>, //=AccountInfo  has to sign it too

    #[account(mut)] //mut to make the amount he has HIGHER
    pub target: AccountInfo<'info>, //=AccountInfo  has to sign it too

    ///CHECK
    #[account(address=system_program::ID)] //so its valid
    pub system_program: AccountInfo<'info>//accountInfo returns account in bits
}


#[account]
#[derive(Default)] //needed?
pub struct Vestment {
    pub vestor: Pubkey, // whose vestment
    pub timestamp: i64, //endtime
    pub amount: u16, //amount?
    pub period: u8, //when it unlocks the percent
    pub beneficiary: Pubkey, // who gets the money
    pub bump: u8, // za pda
    pub num_of_periods: u8,
    pub claim_counter: u8
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const AMOUNT_LENGTH: usize = 2;
const PERIOD_LENGTH: usize= 1;

impl Vestment {
    const LEN: usize=DISCRIMINATOR_LENGTH+PUBLIC_KEY_LENGTH+ TIMESTAMP_LENGTH +
    AMOUNT_LENGTH+PERIOD_LENGTH+PUBLIC_KEY_LENGTH+PERIOD_LENGTH + PERIOD_LENGTH + PERIOD_LENGTH;
}

