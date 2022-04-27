use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program; //dodaje se zbog constrainta za sys_prog::ID

//TODO: PROMENI PROGRAM ID treba 2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ
//Drask ID: 2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ Danilo ID: DYWdbcaqeXrWqvbTHeRVPZdEuUkm7YUDBErMkE7FajJS
declare_id!("DYWdbcaqeXrWqvbTHeRVPZdEuUkm7YUDBErMkE7FajJS");

#[program]
pub mod vesting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct vestment {
    pub author: Pubkey,
    pub ammount: i32, //DA: kolicina tokena koji se zakljucavaju
    pub cliff: i64, //DA: cliff izrazen u danima? mozda postoji bolje resenje
}
