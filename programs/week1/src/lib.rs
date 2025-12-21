#![allow(deprecated)]
use anchor_lang::prelude::*;

declare_id!("6GCjceBJe4CHsrLijFFii313DvuchRPjjzTNHbYCDrxa");

#[program]

pub mod week1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
