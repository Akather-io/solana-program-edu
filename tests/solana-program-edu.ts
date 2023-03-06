import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaProgramEdu } from "../target/types/solana_program_edu";

describe("solana-program-edu", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaProgramEdu as Program<SolanaProgramEdu>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
