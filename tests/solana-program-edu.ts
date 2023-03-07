import * as anchor from "@project-serum/anchor";
import { AnchorProvider, Program } from "@project-serum/anchor";
import { SolanaProgramEdu } from "../target/types/solana_program_edu";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

import { airdrop } from "./utils";

describe("solana-program-edu", () => {
  const provider = AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace
    .SolanaProgramEdu as Program<SolanaProgramEdu>;

  const me = anchor.web3.Keypair.generate();
  const course_id = new anchor.BN("1");
  const course_id2 = new anchor.BN("2");
  const course_name = "My course";
  const course_description = "My course description";
  const course_price = new anchor.BN(1 * LAMPORTS_PER_SOL);
  const instructor = anchor.web3.Keypair.generate();
  const COURSE_SEED = "course";
  const ENROLLMENT_SEED = "enrollment";
  const myWallet = anchor.Wallet.local();
  const delay = (ms) => new Promise((res) => setTimeout(res, ms));

  let courseAccount1: anchor.web3.PublicKey;
  let courseAccount2: anchor.web3.PublicKey;

  before(async () => {
    const [course1] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(COURSE_SEED), course_id.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    courseAccount1 = course1;
    const [course2] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(COURSE_SEED), course_id2.toArrayLike(Buffer, "le", 8)],
      program.programId
    );
    courseAccount2 = course2;
  });

  describe("Course", () => {
    it("Should create a course", async () => {
      const tx = await program.methods
        .createCourse(
          course_id,
          course_name,
          course_description,
          instructor.publicKey,
          course_price
        )
        .accounts({
          course: courseAccount1,
          payer: myWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await program.methods
        .createCourse(
          course_id2,
          course_name + course_id2,
          course_description + course_id2,
          instructor.publicKey,
          course_price
        )
        .accounts({
          course: courseAccount2,
          payer: myWallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
    });
    it("Should get a course", async () => {
      const course = await program.account.course.fetch(courseAccount1);
      expect(course.name).to.equal(course_name);
      expect(course.description).to.equal(course_description);
      expect(course.price.toNumber()).to.equal(course_price.toNumber());
    });
    it("Should enroll a student", async () => {
      const student = anchor.web3.Keypair.generate();
      await airdrop(provider, student.publicKey, 1);

      const [enrollmentAccount] =
        await anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from(ENROLLMENT_SEED),
            course_id.toArrayLike(Buffer, "le", 8),
            Buffer.from(student.publicKey.toBytes()),
          ],
          program.programId
        );

      await program.methods
        .enroll(course_id)
        .accounts({
          enrollment: enrollmentAccount,
          course: courseAccount1,
          student: student.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([student])
        .rpc();

      const student2 = anchor.web3.Keypair.generate();
      await airdrop(provider, student2.publicKey, 1);

      const [enrollmentAccount2] =
        await anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from(ENROLLMENT_SEED),
            course_id2.toArrayLike(Buffer, "le", 8),
            Buffer.from(student2.publicKey.toBytes()),
          ],
          program.programId
        );

      await program.methods
        .enroll(course_id2)
        .accounts({
          enrollment: enrollmentAccount2,
          course: courseAccount2,
          student: student2.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([student2])
        .rpc();
    });
    it("Should get all enrollment", async () => {
      const enrollments = await program.account.enrollment.all();
      console.log("Enrollments count:", enrollments.length);
    });
    it("Should get all student of course", async () => {
      const enrollments = await program.account.enrollment.all([
        {
          memcmp: {
            offset: 8,
            bytes: courseAccount2.toBase58(),
          },
        },
      ]);
      // console.log("Enrollments count:", enrollments.length);
      // console.log("Enrollments:", enrollments);
      const students = enrollments.map((enrollment) => ({
        student: enrollment.account.student.toBase58(),
        course: enrollment.account.course.toBase58(),
        startDateTime: enrollment.account.startDate.toNumber(),
      }));

      console.log("Students:", students);
    });
  });
});
