import { Address } from "@coral-xyz/anchor";
import { PartiallyDecodedInstruction } from "@solana/web3.js";
import decodeIx from "parse/decodeIx";

import { INSTRUCTIONS } from "../constants";

const { IX_INITIALIZE } = INSTRUCTIONS;

export const parseInitializeIx = (
  ix: PartiallyDecodedInstruction,
  programId: Address
) => {
  const decodedIx = decodeIx(ix, programId);
  if (decodedIx == null || decodedIx.name !== IX_INITIALIZE) {
    return null;
  }

  return {
    accounts: {
      admin: ix.accounts[0],
      admin_data: ix.accounts[1],
      metadata: ix.accounts[2],
      mint: ix.accounts[3],
      program_data: ix.accounts[4],
      rent: ix.accounts[5],
      system_program: ix.accounts[7],
      sysvar_instructions: ix.accounts[6],
      token_metadata: ix.accounts[9],
      token_program: ix.accounts[8],
    },
  };
};
