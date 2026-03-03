import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleAmm } from "../target/types/simple_amm";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { assert } from "chai";

describe("simple_amm", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SimpleAmm as Program<SimpleAmm>;
  const payer = provider.wallet as anchor.Wallet;

  let tokenAMint: PublicKey;
  let tokenBMint: PublicKey;
  let poolPda: PublicKey;
  let lpMint: Keypair;
  let vaultA: Keypair;
  let vaultB: Keypair;

  before(async () => {
    // Create token A mint
    tokenAMint = await createMint(
      provider.connection,
      payer.payer,
      payer.publicKey,
      null,
      9
    );

    // Create token B mint
    tokenBMint = await createMint(
      provider.connection,
      payer.payer,
      payer.publicKey,
      null,
      9
    );

    // Derive pool PDA
    [poolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), tokenAMint.toBuffer(), tokenBMint.toBuffer()],
      program.programId
    );

    // Generate keypairs for accounts
    lpMint = Keypair.generate();
    vaultA = Keypair.generate();
    vaultB = Keypair.generate();
  });

  it("Initializes pool successfully", async () => {
    const tx = await program.methods
      .initializePool()
      .accounts({
        user: payer.publicKey,
        tokenAMint: tokenAMint,
        tokenBMint: tokenBMint,
        lpMint: lpMint.publicKey,
        vaultA: vaultA.publicKey,
        vaultB: vaultB.publicKey,
      })
      .signers([lpMint, vaultA, vaultB])
      .rpc();

    console.log("Initialize pool tx:", tx);

    // Fetch pool account
    const poolAccount = await program.account.pool.fetch(poolPda);

    // Verify pool state
    assert.ok(poolAccount.tokenAMint.equals(tokenAMint));
    assert.ok(poolAccount.tokenBMint.equals(tokenBMint));
    assert.ok(poolAccount.vaultA.equals(vaultA.publicKey));
    assert.ok(poolAccount.vaultB.equals(vaultB.publicKey));
    assert.ok(poolAccount.lpMint.equals(lpMint.publicKey));
    assert.equal(poolAccount.lpSupply.toNumber(), 0);
  });

  it("Pool PDA is deterministic", async () => {
    // Derive PDA again
    const [derivedPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), tokenAMint.toBuffer(), tokenBMint.toBuffer()],
      program.programId
    );

    assert.ok(derivedPda.equals(poolPda));
  });

  it("Cannot initialize same pool twice", async () => {
    const newLpMint = Keypair.generate();
    const newVaultA = Keypair.generate();
    const newVaultB = Keypair.generate();

    try {
      await program.methods
        .initializePool()
        .accounts({
          user: payer.publicKey,
          tokenAMint: tokenAMint,
          tokenBMint: tokenBMint,
          lpMint: newLpMint.publicKey,
          vaultA: newVaultA.publicKey,
          vaultB: newVaultB.publicKey,
        })
        .signers([newLpMint, newVaultA, newVaultB])
        .rpc();

      assert.fail("Should have failed to initialize pool twice");
    } catch (err) {
      assert.ok(err.toString().includes("already in use"));
    }
  });

  it("Pool stores correct addresses", async () => {
    const poolAccount = await program.account.pool.fetch(poolPda);

    // Verify all stored addresses match
    assert.ok(poolAccount.tokenAMint.equals(tokenAMint), "Token A mint mismatch");
    assert.ok(poolAccount.tokenBMint.equals(tokenBMint), "Token B mint mismatch");
    assert.ok(poolAccount.vaultA.equals(vaultA.publicKey), "Vault A mismatch");
    assert.ok(poolAccount.vaultB.equals(vaultB.publicKey), "Vault B mismatch");
    assert.ok(poolAccount.lpMint.equals(lpMint.publicKey), "LP mint mismatch");
  });
});
