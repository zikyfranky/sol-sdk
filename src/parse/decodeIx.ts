import { Instruction } from "@coral-xyz/anchor";
import { PartiallyDecodedInstruction } from "@solana/web3.js";
import { SKWIZZ_PROGRAM_ID } from "constants/ProgramIds";
import { SKWIZZ_IDL } from "generated";
import decodeIxWithIdl from "parse/decodeIxWithIdl";
import { Maybe } from "types/UtilityTypes";
import arePublicKeysEqual from "utils/solana/arePublicKeysEqual";

export default function decodeIx(
  ix: PartiallyDecodedInstruction
): Maybe<Instruction> {
  if (!arePublicKeysEqual(ix.programId, SKWIZZ_PROGRAM_ID)) {
    return null;
  }

  return decodeIxWithIdl(ix, SKWIZZ_IDL);
}
