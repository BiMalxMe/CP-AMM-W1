# Simple AMM

Solana constant product AMM built with Anchor 0.31.1.

## Architecture

### Pool Account (PDA)
- **Seeds**: `["pool", token_a_mint, token_b_mint]`
- **Stores**: Token mint addresses, vault addresses, LP mint, LP supply, bump seed
- **Size**: 169 bytes (5 pubkeys + 1 u64 + 1 u8)

### Accounts Created on Initialization
1. **Pool PDA**: Stores pool state and acts as authority for vaults and LP mint
2. **Vault A**: Token account holding token A liquidity (authority: pool PDA)
3. **Vault B**: Token account holding token B liquidity (authority: pool PDA)
4. **LP Mint**: Mint for liquidity provider tokens (authority: pool PDA, decimals: 9)

## Functions

### `initialize_pool`
**Purpose**: Creates a new liquidity pool for two SPL tokens

**Accounts**:
- `user`: Signer who pays for account creation
- `token_a_mint`: Existing mint for token A (InterfaceAccount)
- `token_b_mint`: Existing mint for token B (InterfaceAccount)
- `pool`: Pool PDA to be created
- `lp_mint`: LP token mint to be created
- `vault_a`: Token A vault to be created
- `vault_b`: Token B vault to be created
- `system_program`: For account creation
- `token_program`: For token operations

**Logic**:
1. Derives pool PDA from token mints
2. Initializes pool account with 169 bytes
3. Creates LP mint with pool as authority
4. Creates two vaults with pool as authority
5. Stores all addresses in pool state
6. Sets initial LP supply to 0

**Constraints**:
- Pool PDA derived deterministically (same token pair = same pool)
- All token accounts owned by pool PDA for security
- User pays rent for all accounts

## Next Steps
- Add liquidity (deposit tokens, mint LP tokens)
- Remove liquidity (burn LP tokens, withdraw tokens)
- Swap tokens (constant product formula: x * y = k)