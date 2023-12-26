import { Address, Instruction } from "@coral-xyz/anchor";
import { PartiallyDecodedInstruction } from "@solana/web3.js";
import { APP_IDL } from "generated";
import decodeIxWithIdl from "parse/decodeIxWithIdl";
import { Maybe } from "types/UtilityTypes";
import arePublicKeysEqual from "utils/solana/arePublicKeysEqual";

export default function decodeIx(
  ix: PartiallyDecodedInstruction,
  programId: Address
): Maybe<Instruction> {
  if (!arePublicKeysEqual(ix.programId, programId)) {
    return null;
  }

  return decodeIxWithIdl(ix, APP_IDL);
}
