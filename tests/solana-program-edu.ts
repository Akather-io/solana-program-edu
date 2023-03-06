import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaProgramEdu } from "../target/types/solana_program_edu";

describe("solana-program-edu", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .SolanaProgramEdu as Program<SolanaProgramEdu>;

  const me = anchor.web3.Keypair.generate();
  const course_id = new anchor.BN("1");
  const course_name = "My course";
  const course_description = "My course description";
  const course_price = 100;
  const instructor = anchor.web3.Keypair.generate();
  const COURSE_SEED = "course";
  const myWallet = anchor.Wallet.local();
  describe("Course", () => {
    it("Should create a course", async () => {
      const [courseAccount, courseBump] =
        await anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from(COURSE_SEED),
            course_id.toArrayLike(Buffer, "le", 8),
            myWallet.publicKey.toBuffer(),
          ],
          program.programId
        );
      const tx = await program.methods
        .createCourse(
          course_id,
          course_name,
          course_description,
          instructor.publicKey
        )
        .accounts({
          course: courseAccount,
          payer: myWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })

        .rpc();
    });
  });
});
