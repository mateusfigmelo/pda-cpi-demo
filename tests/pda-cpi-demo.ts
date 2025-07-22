import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Caller } from "../target/types/caller";
import { Callee } from "../target/types/callee";

describe("PDA as signer CPI demo", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const caller = anchor.workspace.Caller as Program<Caller>;
  const callee = anchor.workspace.Callee as Program<Callee>;

  let callerPda: anchor.web3.PublicKey;
  let bump: number;

  it("Derives the caller's PDA", async () => {
    [callerPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("caller_pda")],
      caller.programId
    );
    
    console.log("✅ Caller PDA:", callerPda.toString());
  });

  it("Calls hello via CPI using caller's PDA as signer", async () => {
    await caller.methods
      .callHello()
      .accounts({
        callerPda: callerPda,
        calleeProgram: callee.programId,
      })
      .rpc();

    console.log("✅ CPI call successful!");
  });
});
