use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program; //dodaje se zbog constrainta za sys_prog::ID

//TODO: PROMENI PROGRAM ID treba 2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ
declare_id!("2GxXeKFC6jL6eMj2a1dCn9XFesYp6WrGXq7HDBZtgcPZ");

#[program]
pub mod vesting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
