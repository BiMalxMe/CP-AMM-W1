use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("11111111111111111111111111111111"); // Replace with your actual program ID

#[program]
pub mod simple_amm {
    use super::*;

}
// accounts context for initializing a pool
#[derive(Accounts)]
pub struct InitializePool<'info> {

    // the user who pays for account creation
    #[account(mut)]
    pub user: Signer<'info>,

    // token a mint
    pub token_a_mint: Account<'info, Mint>,

    // token b mint
    pub token_b_mint: Account<'info, Mint>,

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