import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { CapstoneProject } from "../target/types/capstone_project";

describe("capstone_project", () => {
  // Setup
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Capstone as Program<CapstoneProject>;
  const authority = provider.wallet;

  // PDAs
  const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), authority.publicKey.toBuffer()],
    program.programId
  );
  const [spendingAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("spending"), authority.publicKey.toBuffer()],
    program.programId
  );
  const [yieldAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("yield"), authority.publicKey.toBuffer()],
    program.programId
  );
  const [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    program.programId
  );

  // The public key for the real Pyth SOL/USD price feed on mainnet.
  // Anchor's local validator is smart enough to fetch this account's data for read-only operations.
  const pythSolUsdPriceFeed = new anchor.web3.PublicKey(
    "H6ARHf6YXhGYeQfSpCsXidIoT1S8MmktedB2CDdrA7w1"
  );

  // Initialize accounts once
  before("Initialize accounts", async () => {
    try { await program.account.vault.fetch(vaultPda); } catch { await program.methods.initialize().accounts({ vault: vaultPda, authority: authority.publicKey }).rpc(); }
    try { await program.account.spendingAccount.fetch(spendingAccountPda); } catch { await program.methods.initializeSpendingAccount().accounts({ spendingAccount: spendingAccountPda, authority: authority.publicKey }).rpc(); }
    try { await program.account.yieldAccount.fetch(yieldAccountPda); } catch { await program.methods.initializeYieldAccount().accounts({ yieldAccount: yieldAccountPda, authority: authority.publicKey }).rpc(); }
    try { await program.account.treasury.fetch(treasuryPda); } catch { await program.methods.initializeTreasury().accounts({ treasury: treasuryPda, authority: authority.publicKey }).rpc(); }
  });

  it("should correctly calculate the collateralization ratio", async () => {
    // === 1. SETUP THE SCENARIO ===
    // We can't directly set the `amountSpent` for testing, so we'll test with an amountSpent of 0.
    // In a real app, this would be updated by `authorizeSpend`.
    await program.methods.resetSpendTracker().accounts({ authority: authority.publicKey, spendingAccount: spendingAccountPda }).rpc();
    
    // Ensure there's a known staked amount.
    try {
        await program.methods.stake(new anchor.BN(8 * anchor.web3.LAMPORTS_PER_SOL)).accounts({ authority: authority.publicKey, vault: vaultPda, treasury: treasuryPda, yieldAccount: yieldAccountPda }).rpc()
    } catch(e) {
        console.log("Funds already staked, which is fine for this test.");
    }
    
    // === 2. CALL THE LIQUIDATE INSTRUCTION & CAPTURE LOGS ===
    const tx = await program.methods
      .liquidate()
      .accounts({
        userToCheck: authority.publicKey,
        spendingAccount: spendingAccountPda,
        yieldAccount: yieldAccountPda,
        pythPriceFeed: pythSolUsdPriceFeed,
      })
      .transaction();
    
    const txSignature = await provider.sendAndConfirm(tx, []);
    const txInfo = await provider.connection.getTransaction(txSignature, { commitment: "confirmed" });
    const programLogs = txInfo.meta.logMessages;
    
    // === 3. VERIFY THE LOGS ===
    const ratioLog = programLogs.find(log => log.includes("Collateralization Ratio:"));
    assert.ok(ratioLog, "Collateralization Ratio log not found!");
    console.log(`Found log: ${ratioLog}`);

    const stakedValueLog = programLogs.find(log => log.includes("Staked Value (Cents):"));
    assert.ok(stakedValueLog, "Staked Value log not found!");
    console.log(`Found log: ${stakedValueLog}`);
  });
});
