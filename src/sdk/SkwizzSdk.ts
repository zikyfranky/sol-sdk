// eslint-disable-next-line simple-import-sort/imports
import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import { SKWIZZ_PROGRAM_ID } from "constants/ProgramIds";
import { SKWIZZ_IDL, Skwizz, SkwizzProgram } from "generated";
import createInitializeIx from "sdk/instructions/createInitializeIx";
import findProgramPda from "utils/pdas/findProgramPda";
import findUserPda from "utils/pdas/findUserPda";
import ixToTx from "utils/solana/ixToTx";

export default class SkwizzSdk {
  private connection: Connection;
  public program: SkwizzProgram;

  constructor(connection: Connection, wallet: Wallet) {
    this.connection = connection;
    const provider = new AnchorProvider(connection, wallet, {
      preflightCommitment: "recent",
    });

    this.program = new Program<Skwizz>(
      SKWIZZ_IDL as any,
      SKWIZZ_PROGRAM_ID,
      provider
    );
  }

  //
  // TRANSACTIONS
  //

  async createInitializeTx(payer: PublicKey) {
    const ix = await createInitializeIx(payer, {
      program: this.program,
    });
    return ixToTx(ix);
  }

  //
  // FETCH ACCOUNTS
  //

  async fetchProgramInfo() {
    const [program] = await this.findProgramPda();
    return {
      account: await this.program.account.skwizz.fetch(program, "confirmed"),
      pubkey: program,
    };
  }

  async fetchUserInfo(userKey: PublicKey) {
    const [user] = await this.findUserPda(userKey);
    return {
      account: await this.program.account.user.fetch(user, "confirmed"),
      pubkey: user,
    };
  }

  //
  // PDAS
  //

  async findProgramPda() {
    return findProgramPda(this.program.programId);
  }

  async findUserPda(user: PublicKey) {
    return findUserPda(user, this.program.programId);
  }

  //
  // HELPER
  //
  async isInitialized(pda?: PublicKey) {
    const [program] = pda ? [pda] : findProgramPda(this.program.programId);
    const programAccountInfo = await this.connection.getAccountInfo(program);
    return programAccountInfo != null;
  }
}
