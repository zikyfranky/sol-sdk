import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  getAccount,
  TOKEN_2022_PROGRAM_ID,
  TokenAccountNotFoundError,
  TokenInvalidAccountOwnerError,
} from "@solana/spl-token";
import { Connection, PublicKey } from "@solana/web3.js";

export const tokenAccountInstruction = async (
  connection: Connection,
  mint: PublicKey,
  owner: PublicKey,
  associatedToken: PublicKey,
  programId = TOKEN_2022_PROGRAM_ID,
  associatedTokenProgramId = ASSOCIATED_TOKEN_PROGRAM_ID
) => {
  try {
    await getAccount(connection, associatedToken, undefined, programId);
    return null;
  } catch (error: unknown) {
    // TokenAccountNotFoundError can be possible if the associated address has already received some lamports,
    // becoming a system account. Assuming program derived addressing is safe, this is the only case for the
    // TokenInvalidAccountOwnerError in this code path.
    if (
      error instanceof TokenAccountNotFoundError ||
      error instanceof TokenInvalidAccountOwnerError
    ) {
      return createAssociatedTokenAccountInstruction(
        owner,
        associatedToken,
        owner,
        mint,
        programId,
        associatedTokenProgramId
      );
    } else {
      throw error;
    }
  }
};
