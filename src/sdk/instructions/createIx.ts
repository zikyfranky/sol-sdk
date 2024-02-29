/* eslint-disable simple-import-sort/imports */
import { BN } from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
} from "@solana/spl-token";
import {
  PublicKey,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  SYSVAR_RENT_PUBKEY,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { AppProgram } from "generated";
import {
  findMetadataPda,
  findMintPda,
  findProgramPda,
  findUserAtaPda,
  findUserPda,
} from "utils/pdas";
import { MPL_TOKEN_METADATA_PROGRAM_ID } from "../../constants";

export type Metadata = {
  decimals: number;
  name: string;
  symbol: string;
  uri: string;
};

export const createInitializeIx = async (
  user: PublicKey,
  meta: Metadata,
  program: AppProgram
): Promise<TransactionInstruction> => {
  const { programId } = program;
  const [programInfo] = findProgramPda(programId);
  const [userInfo] = findUserPda(user, programId);
  const [mint] = findMintPda(programId);
  const [metadata] = findMetadataPda(mint);

  return program.methods
    .initialize(meta)
    .accounts({
      admin: user,
      adminData: userInfo,
      metadata,
      mint,
      programData: programInfo,
      rent: SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId,
      sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    })
    .instruction();
};

export const createBuyIx = async (
  user: PublicKey,
  amount: BN,
  program: AppProgram,
  referral?: PublicKey
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);
  const [referralInfo] = referral
    ? findUserPda(referral, program.programId)
    : [null];
  const [mint] = findMintPda(program.programId);
  const [userAta] = findUserAtaPda(mint, user);

  return program.methods
    .buy(amount, referral ? referral : null)
    .accounts({
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      mint,
      programData: programInfo,
      referredByData: referralInfo,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      user: user,
      userAta: userAta,
      userData: userInfo,
    })
    .instruction();
};

export const createReinvestIx = async (
  user: PublicKey,
  program: AppProgram
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);
  const [mint] = findMintPda(program.programId);
  const [userAta] = findUserAtaPda(mint, user);

  return program.methods
    .reinvest()
    .accounts({
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      mint,
      programData: programInfo,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      user: user,
      userAta,
      userData: userInfo,
    })
    .instruction();
};

export const createExitIx = async (
  user: PublicKey,
  program: AppProgram
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);
  const [mint] = findMintPda(program.programId);
  const [userAta] = findUserAtaPda(mint, user);

  return program.methods
    .exit()
    .accounts({
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      mint,
      programData: programInfo,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      user: user,
      userAta: userAta,
      userData: userInfo,
    })
    .instruction();
};

export const createTransferIx = async (
  user: PublicKey,
  to: PublicKey,
  amount: BN,
  program: AppProgram
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);
  const [toInfo] = findUserPda(to, program.programId);
  const [mint] = findMintPda(program.programId);
  const [userAta] = findUserAtaPda(mint, user);
  const [toAta] = findUserAtaPda(mint, to);

  return program.methods
    .transfer(to, amount)
    .accounts({
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      mint,
      programData: programInfo,
      systemProgram: SystemProgram.programId,
      toAta: toAta,
      toData: toInfo,
      toInfo: to,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      user: user,
      userAta: userAta,
      userData: userInfo,
    })
    .instruction();
};

export const createWithdrawIx = async (
  user: PublicKey,
  program: AppProgram
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);
  const [mint] = findMintPda(program.programId);
  const [userAta] = findUserAtaPda(mint, user);

  return program.methods
    .withdraw()
    .accounts({
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      mint,
      programData: programInfo,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      user: user,
      userAta: userAta,
      userData: userInfo,
    })
    .instruction();
};

export const createSellIx = async (
  user: PublicKey,
  amount: BN,
  program: AppProgram
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);
  const [mint] = findMintPda(program.programId);
  const [userAta] = findUserAtaPda(mint, user);

  return program.methods
    .sell(amount)
    .accounts({
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      mint,
      programData: programInfo,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      user: user,
      userAta: userAta,
      userData: userInfo,
    })
    .instruction();
};

// Admin functions
export const createDistributeTokenIxs = async (
  user: PublicKey,
  receipients: Array<PublicKey>,
  amount: BN,
  payout: BN,
  program: AppProgram
): Promise<Array<TransactionInstruction>> => {
  const [programInfo] = findProgramPda(program.programId);
  const [mint] = findMintPda(program.programId);
  const [fromInfo] = findUserPda(user, program.programId);
  const [fromAta] = findUserAtaPda(mint, user);

  const instructions = [];

  for (let i = 0; i < receipients.length; i++) {
    const receipient = receipients[i];
    const [receipientInfo] = findUserPda(receipient, program.programId);
    const [receipientAta] = findUserAtaPda(mint, receipient);
    const ix = await program.methods
      .distributeToken(amount, payout, receipient)
      .accounts({
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        fromAta: fromAta,
        fromData: fromInfo,
        mint,
        programData: programInfo,
        receipientAta: receipientAta,
        receipientData: receipientInfo,
        receipientInfo: receipient,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        user: user,
      })
      .instruction();

    instructions.push(ix);
  }

  return instructions;
};

export const createDisableInitialStageIx = (
  user: PublicKey,
  program: AppProgram
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .disableInitialStage()
    .accounts({
      programData: programInfo,
      systemProgram: SystemProgram.programId,
      user: user,
      userData: userInfo,
    })
    .instruction();
};

export const createSetAdministratorIx = (
  admin: PublicKey,
  user: PublicKey,
  status: boolean,
  program: AppProgram
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
      systemProgram: SystemProgram.programId,
      userData: userInfo,
    })
    .instruction();
};

export const createSetAmbassadorIx = (
  admin: PublicKey,
  user: PublicKey,
  status: boolean,
  program: AppProgram
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [adminInfo] = findUserPda(admin, program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .setAmbassador(user, status)
    .accounts({
      admin: admin,
      adminData: adminInfo,
      programData: programInfo,
      systemProgram: SystemProgram.programId,
      userData: userInfo,
    })
    .instruction();
};

export const createSetStakingRequirementIx = (
  user: PublicKey,
  amountOfTokens: BN,
  program: AppProgram
): Promise<TransactionInstruction> => {
  const [programInfo] = findProgramPda(program.programId);
  const [userInfo] = findUserPda(user, program.programId);

  return program.methods
    .setStakingRequirement(amountOfTokens)
    .accounts({
      programData: programInfo,
      systemProgram: SystemProgram.programId,
      user: user,
      userData: userInfo,
    })
    .instruction();
};
