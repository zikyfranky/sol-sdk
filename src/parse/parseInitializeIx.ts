import { Address } from "@coral-xyz/anchor";
import { PartiallyDecodedInstruction } from "@solana/web3.js";
import decodeIx from "parse/decodeIx";

const IX_NAME = "initialize";

export default function parseInitializeIx(
  ix: PartiallyDecodedInstruction,
  programId: Address
) {
  const decodedIx = decodeIx(ix, programId);
  if (decodedIx == null || decodedIx.name !== IX_NAME) {
    return null;
  }

  return {
    accounts: {
      admin: ix.accounts[0],
      admin_data: ix.accounts[1],
      program_data: ix.accounts[2],
      system_program: ix.accounts[3],
    },
  };
}
