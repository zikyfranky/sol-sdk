import { BN } from "@coral-xyz/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { AppProgram } from "generated";
import { findProgramPda, findUserPda } from "utils/pdas";

export const getMyDividends = (
  user: PublicKey,
  withReferralBonus: boolean,
  program: AppProgram,
  test: boolean
): { ixName: string; value: Promise<BN | TransactionInstruction> } => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  const partials = program.methods.myDividends(withReferralBonus).accounts({
    programData: programInfo,
    userData: userInfo,
  });

  return {
    ixName: "myDividends",
    value: test ? partials.view() : partials.instruction(),
  };
};

export const getSellPrice = (
  program: AppProgram,
  test: boolean
): { ixName: string; value: Promise<BN | TransactionInstruction> } => {
  const [programInfo] = findProgramPda(program.programId);

  const partials = program.methods.sellPrice().accounts({
    programData: programInfo,
  });
  return {
    ixName: "sellPrice",
    value: test ? partials.view() : partials.instruction(),
  };
};

export const getBuyPrice = (
  program: AppProgram,
  test: boolean
): { ixName: string; value: Promise<BN | TransactionInstruction> } => {
  const [programInfo] = findProgramPda(program.programId);

  const partials = program.methods.buyPrice().accounts({
    programData: programInfo,
  });
  return {
    ixName: "buyPrice",
    value: test ? partials.view() : partials.instruction(),
  };
};

export const getCalculateLamportsReceived = (
  lamports: BN,
  program: AppProgram,
  test: boolean
): { ixName: string; value: Promise<BN | TransactionInstruction> } => {
  const [programInfo] = findProgramPda(program.programId);

  const partials = program.methods
    .calculateLamportsReceived(lamports)
    .accounts({
      programData: programInfo,
    });
  return {
    ixName: "calculateLamportsReceived",
    value: test ? partials.view() : partials.instruction(),
  };
};

export const getCalculateTokensReceived = (
  tokens: BN,
  program: AppProgram,
  test: boolean
): { ixName: string; value: Promise<BN | TransactionInstruction> } => {
  const [programInfo] = findProgramPda(program.programId);

  const partials = program.methods.calculateTokensReceived(tokens).accounts({
    programData: programInfo,
  });

  return {
    ixName: "calculateTokensReceived",
    value: test ? partials.view() : partials.instruction(),
  };
};
