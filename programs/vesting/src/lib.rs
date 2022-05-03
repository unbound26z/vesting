use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program; //dodaje se zbog constrainta za sys_prog::ID
//use anchor_spl::{associated_token::AssociatedToken, token::{CloseAccount, Mint, Token, TokenAccount, Transfer}};

//TODO: PROMENI PROGRAM ID treba 2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ
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
}

#[derive(Accounts)]
pub struct MakeVestment<'info> {
    #[account(init,payer=vestor,space=Vestment::LEN)] //inituje acc prave velicine
    pub vestment: Account<'info,Vestment>, //parsuje iz bitova u Vestment struct

    #[account(mut)] //mut da bi smanjili kolko para ima
    pub vestor: Signer<'info>, //=AccountInfo al mora i da potpise

    ///CHECK
    #[account(address=system_program::ID)] //da bude valid
    pub system_program: AccountInfo<'info>//accountInfo daje account u bitovima

}


#[account]
pub struct Vestment {
    pub vestor: Pubkey, // ciji vestment
    pub timestamp: i64, //pocetak cliffa npr.
    pub amount: u16, //vljd dovoljan uint8
    pub cliff: u16, //u danima?
    pub period: u8, //na kolko se otkljucava
    //pub procent: u8 //koliko posto investmenta se unlockuje po periodu (ne pise u notion)
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

