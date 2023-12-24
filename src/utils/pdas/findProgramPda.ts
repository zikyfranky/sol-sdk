import { PublicKey } from "@solana/web3.js";
import { PROGRAM } from "constants/PdaConstants";

export default function findProgramPda(programId: PublicKey) {
  return PublicKey.findProgramAddressSync([Buffer.from(PROGRAM)], programId);
}
