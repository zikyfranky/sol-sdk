import { BN } from "@coral-xyz/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { AppProgram } from "generated";
import findProgramPda from "utils/pdas/findProgramPda";
import findUserPda from "utils/pdas/findUserPda";

type Args = {
  program: AppProgram;
};

export const createInitializeIx = (
  user: PublicKey,
  { program }: Args
): Promise<TransactionInstruction> => {
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
};

export const createBuyIx = (
  user: PublicKey,
  amount: BN,
  { program }: Args,
  referral?: PublicKey
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);
  const [referralInfo] = referral
    ? findUserPda(referral, program.programId)
    : [null];

  return program.methods
    .buy(amount, referral ? referral : null)
    .accounts({
      programData: programInfo,
      referredByData: referralInfo,
      user: user,
      userData: userInfo,
    })
    .instruction();
};

export const createReinvestIx = (
  user: PublicKey,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .reinvest()
    .accounts({
      programData: programInfo,
      user: user,
      userData: userInfo,
    })
    .instruction();
};

export const createExitIx = (
  user: PublicKey,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .exit()
    .accounts({
      programData: programInfo,
      user: user,
      userData: userInfo,
    })
    .instruction();
};

export const createTransferIx = (
  user: PublicKey,
  to: PublicKey,
  amount: BN,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);
  const [toInfo] = findUserPda(to, program.programId);

  return program.methods
    .transfer(to, amount)
    .accounts({
      programData: programInfo,
      toData: toInfo,
      user: user,
      userData: userInfo,
    })
    .instruction();
};

export const createWithdrawrIx = (
  user: PublicKey,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .withdraw()
    .accounts({
      programData: programInfo,
      user: user,
      userData: userInfo,
    })
    .instruction();
};

export const createSellIx = (
  user: PublicKey,
  amount: BN,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .sell(amount)
    .accounts({
      programData: programInfo,
      user: user,
      userData: userInfo,
    })
    .instruction();
};

// Admin functions
export const createDisableInitialStageIx = (
  user: PublicKey,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .disableInitialStage()
    .accounts({
      programData: programInfo,
      user: user,
      userData: userInfo,
    })
    .instruction();
};

export const createSetAdministratorIx = (
  admin: PublicKey,
  user: PublicKey,
  status: boolean,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [adminInfo] = findUserPda(admin, program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .setAdministrator(user, status)
    .accounts({
      admin: admin,
      adminData: adminInfo,
      programData: programInfo,
      userData: userInfo,
    })
    .instruction();
};

export const createSetAmbassadorIx = (
  admin: PublicKey,
  user: PublicKey,
  status: boolean,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [adminInfo] = findUserPda(admin, program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .setAdministrator(user, status)
    .accounts({
      admin: admin,
      adminData: adminInfo,
      programData: programInfo,
      userData: userInfo,
    })
    .instruction();
};

export const createSetStakingRequirementIx = (
  user: PublicKey,
  amountOfTokens: BN,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .setStakingRequirement(amountOfTokens)
    .accounts({
      programData: programInfo,
      user: user,
      userData: userInfo,
    })
    .instruction();
};

export const createSetNameIx = (
  user: PublicKey,
  name: string,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .setName(name)
    .accounts({
      programData: programInfo,
      user: user,
      userData: userInfo,
    })
    .instruction();
};

export const createSetSymbolIx = (
  user: PublicKey,
  symbol: string,
  { program }: Args
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .setSymbol(symbol)
    .accounts({
      programData: programInfo,
      user: user,
      userData: userInfo,
    })
    .instruction();
};
