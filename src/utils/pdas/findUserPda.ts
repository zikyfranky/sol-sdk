import { PublicKey } from "@solana/web3.js";

import { USER } from "../../constants";

export default function findUserPda(user: PublicKey, programId: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(USER), user.toBuffer()],
    programId
  );
}
