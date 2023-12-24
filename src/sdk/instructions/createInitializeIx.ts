import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { SkwizzProgram } from "generated";
import findProgramPda from "utils/pdas/findProgramPda";
import findUserPda from "utils/pdas/findUserPda";

type Args = {
  program: SkwizzProgram;
};

export default function createInitializeIx(
  user: PublicKey,
  { program }: Args
): Promise<TransactionInstruction> {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .initialize()
    .accounts({
      admin: user,
      adminData: userInfo,
      programData: programInfo,
    })
    .instruction();
}
