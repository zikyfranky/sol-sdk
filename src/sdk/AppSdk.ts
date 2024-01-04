// eslint-disable-next-line simple-import-sort/imports
import { Address, AnchorProvider, BN, Program } from "@coral-xyz/anchor";
import { IdlCoder } from "@coral-xyz/anchor/dist/cjs/coder/borsh/idl";
import { Wallet } from "@coral-xyz/anchor/dist/cjs/provider";
import { decode } from "@coral-xyz/anchor/dist/cjs/utils/bytes/base64";
import {
  Connection,
  PublicKey,
  TransactionInstruction,
  TransactionMessage,
  VersionedTransaction,
} from "@solana/web3.js";
import { APP_IDL, App, AppProgram } from "generated";
import { IDL } from "generated/App";
import {
  createBuyIx,
  createExitIx,
  createInitializeIx,
  createReinvestIx,
  createSellIx,
  createTransferIx,
  createWithdrawrIx,
} from "sdk/instructions/createIx";
import findProgramPda from "utils/pdas/findProgramPda";
import findUserPda from "utils/pdas/findUserPda";
import ixToTx from "utils/solana/ixToTx";
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
  async createInitializeTx(payer: PublicKey) {
    const ix = await createInitializeIx(payer, {
      program: this.program,
    });
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createBuyTx(payer: PublicKey, amount: BN, referral?: PublicKey) {
    const ix = await createBuyIx(
      payer,
      amount,
      {
        program: this.program,
      },
      referral
    );
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createReinvestTx(payer: PublicKey) {
    const ix = await createReinvestIx(payer, {
      program: this.program,
    });
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createExitTx(payer: PublicKey) {
    const ix = await createExitIx(payer, {
      program: this.program,
    });
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createTransferTx(payer: PublicKey, to: PublicKey, amount: BN) {
    const ix = await createTransferIx(payer, to, amount, {
      program: this.program,
    });
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createWithdrawTx(payer: PublicKey) {
    const ix = await createWithdrawrIx(payer, {
      program: this.program,
    });
    return ixToTx(this.connection, payer, ix, this.test);
  }

  async createSellTx(payer: PublicKey, amount: BN) {
    const ix = await createSellIx(payer, amount, {
      program: this.program,
    });
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
  ): Promise<any | null> {
    const ixx = IDL.instructions.find((i) => i.name == ixName);
    const isMut = ixx && [...ixx.accounts].find((a: any) => a.isMut);
    const returnType = ixx && (ixx as any).returns;
    if (isMut || !returnType) return null;

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
      console.log(sim.value);
      if (sim.value.logs) {
        for (let i = 0; i < sim.value.logs!.length; i++) {
          const log = sim.value.logs![i];
          if (log?.includes("Error")) {
            throw new Error(log.split("Error Message: ")[1]);
          }
        }
      }
      throw new Error(JSON.stringify(sim.value.err));
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

    if (!base64) return null;

    const coder = IdlCoder.fieldLayout(
      { type: returnType },
      Array.from([...(IDL.accounts ?? [])])
    );

    return coder.decode(base64);
  }

  //
  // READ ONLY INSTRUCTIONS
  //
  async myDividends(user: PublicKey, ref: boolean) {
    const { ixName, value } = getMyDividends(
      user,
      ref,
      {
        program: this.program,
      },
      this.test
    );
    const awaitedValue = await value;
    if (BN.isBN(awaitedValue)) return awaitedValue;

    return this.readOnly(awaitedValue, ixName);
  }

  async sellPrice() {
    const { ixName, value } = getSellPrice(
      {
        program: this.program,
      },
      this.test
    );
    const awaitedValue = await value;
    if (BN.isBN(awaitedValue)) return awaitedValue;

    return this.readOnly(awaitedValue, ixName);
  }

  async buyPrice() {
    const { ixName, value } = getBuyPrice(
      {
        program: this.program,
      },
      this.test
    );
    const awaitedValue = await value;
    if (BN.isBN(awaitedValue)) return awaitedValue;

    return this.readOnly(awaitedValue, ixName);
  }

  async calculateLamportsReceived(tokens: BN) {
    const { ixName, value } = getCalculateLamportsReceived(
      tokens,
      {
        program: this.program,
      },
      this.test
    );
    const awaitedValue = await value;
    if (BN.isBN(awaitedValue)) return awaitedValue;

    return this.readOnly(awaitedValue, ixName);
  }

  async calculateTokensReceived(lamports: BN) {
    const { ixName, value } = getCalculateTokensReceived(
      lamports,
      {
        program: this.program,
      },
      this.test
    );
    const awaitedValue = await value;

    if (BN.isBN(awaitedValue)) return awaitedValue;

    return this.readOnly(awaitedValue, ixName);
  }
}
