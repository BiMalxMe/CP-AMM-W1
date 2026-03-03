use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint as InterfaceMint;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("11111111111111111111111111111111"); // Replace with your actual program ID

#[program]
pub mod simple_amm {
    use super::*;

    // function to initialize the pool
pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {

    // get mutable reference to pool account
    let pool = &mut ctx.accounts.pool;

    // store token a mint in pool
    pool.token_a_mint = ctx.accounts.token_a_mint.key();

    // store token b mint in pool
    pool.token_b_mint = ctx.accounts.token_b_mint.key();

    // store vault a address in pool
    pool.vault_a = ctx.accounts.vault_a.key();

    // store vault b address in pool
    pool.vault_b = ctx.accounts.vault_b.key();

    // store lp token mint address in pool
    pool.lp_mint = ctx.accounts.lp_mint.key();

    // set initial lp supply to zero
    pool.lp_supply = 0;

    // log success message
    msg!("pool initialized successfully!");

    // return ok
    Ok(())
}

}
// accounts context for initializing a pool
#[derive(Accounts)]
pub struct InitializePool<'info> {

    // the user who pays for account creation
    #[account(mut)]
    pub user: Signer<'info>,

    // token a mint
    pub token_a_mint: InterfaceAccount<'info, InterfaceMint>,

    // token b mint
    pub token_b_mint: InterfaceAccount<'info, InterfaceMint>,

    // pool account (PDA), stores pool state
    #[account(
        init,
        payer = user,
        seeds = [
            b"pool", 
            token_a_mint.key().as_ref(), 
            token_b_mint.key().as_ref()
        ],
        bump,
        space = 8 + Pool::LEN,
    )]
    pub pool: Account<'info, Pool>,

    // lp token mint, authority is pool PDA
    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = pool,
    )]
    pub lp_mint: Account<'info, Mint>,

    // token a vault owned by pool
    #[account(
        init,
        payer = user,
        token::mint = token_a_mint,
        token::authority = pool,
    )]
    pub vault_a: Account<'info, TokenAccount>,

    // token b vault owned by pool
    #[account(
        init,
        payer = user,
        token::mint = token_b_mint,
        token::authority = pool,
    )]
    pub vault_b: Account<'info, TokenAccount>,

    // system program for account creation
    pub system_program: Program<'info, System>,

    // token program to mint and transfer tokens
    pub token_program: Program<'info, Token>,

}


// pool account stores all information about the liquidity pool
#[account]
pub struct Pool {

    // token a mint address
    pub token_a_mint: Pubkey,

    // token b mint address
    pub token_b_mint: Pubkey,

    // vault a account (token a custody)
    pub vault_a: Pubkey,

    // vault b account (token b custody)
    pub vault_b: Pubkey,

    // lp token mint address
    pub lp_mint: Pubkey,

    // total lp tokens minted
    pub lp_supply: u64,

    // bump seed for pool pda
    pub bump: u8,
}

// implementation for pool
impl Pool {

    // length of the pool account (5 pubkeys + 1 u64 + 1 u8)
    pub const LEN: usize = 32 * 5 + 8 + 1;
}