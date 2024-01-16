import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

import {
  METADATA_SEED,
  MINT,
  MPL_TOKEN_METADATA_PROGRAM_ID,
  PROGRAM,
  USER,
} from "../../constants";

export const findMintPda = (programId: PublicKey) =>
  PublicKey.findProgramAddressSync([Buffer.from(MINT)], programId);

export const findMetadataPda = (mint: PublicKey) =>
  PublicKey.findProgramAddressSync(
    [
      Buffer.from(METADATA_SEED),
      MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    MPL_TOKEN_METADATA_PROGRAM_ID
  );

export const findUserPda = (user: PublicKey, programId: PublicKey) =>
  PublicKey.findProgramAddressSync(
    [Buffer.from(USER), user.toBuffer()],
    programId
  );

export const findProgramPda = (programId: PublicKey) =>
  PublicKey.findProgramAddressSync([Buffer.from(PROGRAM)], programId);

export const findUserAtaPda = (mint: PublicKey, user: PublicKey) =>
  PublicKey.findProgramAddressSync(
    [user.toBuffer(), TOKEN_2022_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_TOKEN_PROGRAM_ID
  );
