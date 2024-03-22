// eslint-disable-next-line simple-import-sort/imports
import { Address, AnchorProvider, BN, Program } from "@coral-xyz/anchor";
import { IdlCoder } from "@coral-xyz/anchor/dist/cjs/coder/borsh/idl";
import { Wallet } from "@coral-xyz/anchor/dist/cjs/provider";
import { decode } from "@coral-xyz/anchor/dist/cjs/utils/bytes/base64";
import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import { APP_IDL, App, AppProgram } from "generated";
import { IDL } from "generated/App";
import {
  Metadata,
  createBuyIx,
  createDisableInitialStageIx,
  createDistributeTokenIxs,
  createExitIx,
  createInitializeIx,
  createReinvestIx,
  createSellIx,
  createSetAdministratorIx,
  createSetAmbassadorIx,
  createSetStakingRequirementIx,
  createTransferIx,
  createWithdrawIx,
} from "sdk/instructions/createIx";
import { findProgramPda, findUserPda } from "utils/pdas";
import ixToTx from "utils/solana/ixToTx";
import ixsToTx from "utils/solana/ixsToTx";
import {
  getBuyPrice,
  getCalculateLamportsReceived,
  getCalculateTokensReceived,
  getMyDividends,
  getSellPrice,
} from "./instructions/getters";

export default class AppSdk {
  private connection: Connection;
  public program: AppProgram;
  public test: boolean;
  public wallet: Wallet;

  constructor(
    connection: Connection,
    wallet: Wallet,
    programId: Address,
    test: boolean
  ) {
    this.connection = connection;
    this.wallet = wallet;
    const provider = new AnchorProvider(connection, wallet, {
      preflightCommitment: "recent",
    });

    this.test = test;
    this.program = new Program<App>(APP_IDL as any, programId, provider);
  }

  //
  // TRANSACTIONS
  //
  async createInitializeTx(payer: PublicKey, meta: Metadata) {
    const ix = await createInitializeIx(payer, meta, this.program);

    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createBuyTx(payer: PublicKey, amount: BN, referral?: PublicKey) {
    if (referral && referral.equals(payer)) referral = undefined;
    const ix = await createBuyIx(payer, amount, this.program, referral);
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createReinvestTx(payer: PublicKey) {
    const ix = await createReinvestIx(payer, this.program);
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createDistributeTokenTx(
    payer: PublicKey,
    receipients: Array<PublicKey>,
    batchSize: number,
    amount: BN,
    payout: BN
  ) {
    const txs: Array<Transaction> = [];
    const ixs = await createDistributeTokenIxs(
      payer,
      receipients,
      amount,
      payout,
      this.program
    );

    const runs =
      Math.floor(ixs.length / batchSize) + (ixs.length % batchSize > 0 ? 1 : 0);
    let start = 0;
    let end = start + batchSize;
    for (let i = 0; i < runs; i++) {
      txs.push(
        await ixsToTx(this.connection, payer, ixs.slice(start, end), this.test)
      );

      start = end;
      end += batchSize;
    }

    return txs;
  }

  async createExitTx(payer: PublicKey) {
    const ix = await createExitIx(payer, this.program);
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createTransferTx(payer: PublicKey, to: PublicKey, amount: BN) {
    const ix = await createTransferIx(payer, to, amount, this.program);
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createWithdrawTx(payer: PublicKey) {
    const ix = await createWithdrawIx(payer, this.program);
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createSellTx(payer: PublicKey, amount: BN) {
    const ix = await createSellIx(payer, amount, this.program);
    return ixToTx(this.connection, payer, ix, this.test);
  }

  //
  // ADMIN
  //
  async createDisableInitialStageTx(payer: PublicKey) {
    const ix = await createDisableInitialStageIx(payer, this.program);
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createSetAdministratorTx(
    payer: PublicKey,
    user: PublicKey,
    status: boolean
  ) {
    const ix = await createSetAdministratorIx(
      payer,
      user,
      status,
      this.program
    );
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createSetAmbassadorTx(
    payer: PublicKey,
    user: PublicKey,
    status: boolean
  ) {
    const ix = await createSetAmbassadorIx(payer, user, status, this.program);
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createSetStakingRequirementTx(payer: PublicKey, amountOfTokens: BN) {
    const ix = await createSetStakingRequirementIx(
      payer,
      amountOfTokens,
      this.program
    );
    return ixToTx(this.connection, payer, ix, this.test);
  }

  //
  // FETCH ACCOUNTS
  //
  async fetchProgramInfo() {
    const [program] = this.findProgramPda();
    return {
      account: await this.program.account.app.fetch(program, "confirmed"),
      pubkey: program,
    };
  }

  async fetchUserInfo(userKey: PublicKey) {
    const [user] = this.findUserPda(userKey);
    return {
      account: await this.program.account.user.fetch(user, "confirmed"),
      pubkey: user,
    };
  }

  //
  // PDAS
  //
  findProgramPda() {
    return findProgramPda(this.program.programId);
  }

  findUserPda(user: PublicKey) {
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

  async readOnly(
    ix: TransactionInstruction,
    ixName: string
  ): Promise<{ logs: Array<string>; value: any }> {
    const ixx = IDL.instructions.find((i) => i.name == ixName);
    const isMut = ixx && [...ixx.accounts].find((a: any) => a.isMut);
    const returnType = ixx && (ixx as any).returns;
    if (isMut || !returnType) return { logs: [], value: null };

    const { blockhash } = await this.connection.getLatestBlockhash();

    const msg = new TransactionMessage({
      instructions: [ix],
      payerKey: this.wallet.publicKey,
      recentBlockhash: blockhash,
    }).compileToV0Message();
    const tx = new VersionedTransaction(msg);

    // Simulate tx
    const sim = await this.connection.simulateTransaction(tx);

    if (sim.value.err) {
      let err;
      if (sim.value.logs && sim.value.logs.length > 0) {
        for (let i = 0; i < sim.value.logs!.length; i++) {
          const log = sim.value.logs![i];
          if (log?.includes("Error")) {
            err = new Error(log.split("Error Message: ")[1]);
          }
        }
      } else {
        err = new Error(JSON.stringify(sim.value.err));
      }

      err = err || new Error("Unknown Error");
      (err as any).logs = sim.value.logs || [];
      throw err;
    }

    let base64: Buffer | null = null;

    if (sim.value.returnData?.data) {
      base64 = decode(sim.value.returnData.data[0]);
    } else {
      // Log all the transaction logs.
      const returnPrefix = `Program return: ${this.program.programId} `;
      const returnLogEntry = sim.value.logs!.find((log) =>
        log.startsWith(returnPrefix)
      );

      if (returnLogEntry) {
        base64 = decode(returnLogEntry.slice(returnPrefix.length));
      }
    }

    if (!base64) return { logs: sim.value.logs || [], value: null };

    const coder = IdlCoder.fieldLayout(
      { type: returnType },
      Array.from([...(IDL.accounts ?? [])])
    );

    return { logs: sim.value.logs || [], value: coder.decode(base64) };
  }

  //
  // READ ONLY INSTRUCTIONS
  //
  async myDividends(user: PublicKey, ref: boolean) {
    const { ixName, value } = getMyDividends(
      user,
      ref,
      this.program,
      this.test
    );
    const awaitedValue = await value;
    if (BN.isBN(awaitedValue)) return { logs: [], value: awaitedValue };

    return this.readOnly(awaitedValue, ixName);
  }

  async sellPrice() {
    const { ixName, value } = getSellPrice(this.program, this.test);
    const awaitedValue = await value;
    if (BN.isBN(awaitedValue)) return { logs: [], value: awaitedValue };

    return this.readOnly(awaitedValue, ixName);
  }

  async buyPrice() {
    const { ixName, value } = getBuyPrice(this.program, this.test);
    const awaitedValue = await value;
    if (BN.isBN(awaitedValue)) return { logs: [], value: awaitedValue };

    return this.readOnly(awaitedValue, ixName);
  }

  async calculateLamportsReceived(tokens: BN) {
    const { ixName, value } = getCalculateLamportsReceived(
      tokens,
      this.program,
      this.test
    );
    const awaitedValue = await value;
    if (BN.isBN(awaitedValue)) return { logs: [], value: awaitedValue };

    return this.readOnly(awaitedValue, ixName);
  }

  async calculateTokensReceived(lamports: BN) {
    const { ixName, value } = getCalculateTokensReceived(
      lamports,
      this.program,
      this.test
    );
    const awaitedValue = await value;

    if (BN.isBN(awaitedValue)) return { logs: [], value: awaitedValue };

    return this.readOnly(awaitedValue, ixName);
  }
}
