/* eslint-disable simple-import-sort/imports */
import { AnchorProvider, BN, setProvider } from "@coral-xyz/anchor";
import {
  Account,
  TOKEN_2022_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PartiallyDecodedInstruction,
  PublicKey,
  SendTransactionError,
} from "@solana/web3.js";
import { parseInitializeIx } from "parse/parseIx";
import AppSdk from "sdk/AppSdk";
import requestAirdrops from "tests/utils/requestAirdrops";
import sendTransactionForTest from "tests/utils/sendTransactionForTest";
import invariant from "tiny-invariant";
import { findMintPda } from "utils/pdas";

import getKeyPair from "../utils/getKeypair";

const USER = Keypair.fromSecretKey(getKeyPair());
const user1 = Keypair.generate();
const user2 = Keypair.generate();
const PROGRAM_ID = Keypair.fromSecretKey(
  getKeyPair(`${__dirname}/../../target/deploy/app-keypair.json`)
).publicKey;

let programDataAddress: PublicKey;
let userDataAddress: PublicKey;
let initialTx = "";

// Configure the client to use the local cluster.
const provider = AnchorProvider.env();
const { wallet, connection } = provider;
setProvider(provider);

const sdk = new AppSdk(connection, wallet, PROGRAM_ID, true);

async function getFirstAndOnlyIx(txid: string) {
  const parsedTx = await connection.getParsedTransaction(txid, "confirmed");
  const instructions = parsedTx!.transaction.message.instructions;
  expect(instructions.length).toEqual(1);
  return instructions[0];
}

describe("Program", () => {
  beforeAll(async () => {
    await requestAirdrops(connection, [USER]);

    [programDataAddress] = sdk.findProgramPda();
    [userDataAddress] = sdk.findUserPda(USER.publicKey);
  });

  it("Create Program Account and User Account", async () => {
    const tx = await sdk.createInitializeTx(USER.publicKey, {
      decimals: 9,
      name: "app",
      symbol: "APP",
      uri: "",
    });

    initialTx = await sendTransactionForTest(connection, tx, [USER]);

    const { account: program, pubkey: programPda } =
      await sdk.fetchProgramInfo();

    const { account: user, pubkey: userPda } = await sdk.fetchUserInfo(
      USER.publicKey
    );

    const { isInitialPhase, isInitialized } = program;
    expect(programPda.toString()).toEqual(programPda.toString());
    expect(isInitialPhase).toBeTruthy();
    expect(isInitialized).toBeTruthy();

    const { authority, balance, isAdmin, isAmb, referredBalance } = user;
    expect(userPda.toString()).toEqual(userPda.toString());
    expect(authority.toString()).toEqual(USER.publicKey.toString());
    expect(balance.toNumber()).toEqual(0);
    expect(referredBalance.toNumber()).toEqual(0);
    expect(isAdmin).toBeTruthy();
    expect(isAmb).toBeTruthy();
  });

  it("Parse initialize ix", async () => {
    const ix = await getFirstAndOnlyIx(initialTx);
    const parsedIx = parseInitializeIx(
      ix as PartiallyDecodedInstruction,
      PROGRAM_ID
    );

    // const deets = await connection.getParsedTransaction(initialTx, {
    //   commitment: "confirmed",
    // });
    // console.log(deets?.meta?.logMessages);
    expect(parsedIx).not.toBeNull();
    invariant(parsedIx != null);
    const ixAccounts = parsedIx.accounts as { [key: string]: PublicKey };
    expect(ixAccounts.admin.toString()).toEqual(USER.publicKey.toString());
    expect(ixAccounts.admin_data.toString()).toEqual(
      userDataAddress.toString()
    );
    expect(ixAccounts.program_data.toString()).toEqual(
      programDataAddress.toString()
    );
  });

  it("Can Buy Token", async () => {
    const tx = await sdk.createBuyTx(USER.publicKey, new BN(LAMPORTS_PER_SOL));

    await sendTransactionForTest(connection, tx, [USER]);

    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      USER,
      findMintPda(sdk.program.programId)[0],
      USER.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const { account: program } = await sdk.fetchProgramInfo();
    const { account: user } = await sdk.fetchUserInfo(USER.publicKey);
    const { tokenSupply, contractBalance } = program;

    expect(new BN(tokenAccount.amount.toString()) == user.balance);
    expect(tokenSupply == user.balance);
    expect(new BN(LAMPORTS_PER_SOL) == contractBalance);
  });

  it("Can Distribute Tokens", async () => {
    await requestAirdrops(connection, [user1, user2]);

    const { account: user } = await sdk.fetchUserInfo(USER.publicKey);

    const amount = user.balance.div(new BN(2));
    const payout = user.payout.div(new BN(2));

    const txs = await sdk.createDistributeTokenTx(
      USER.publicKey,
      [user1.publicKey, user2.publicKey],
      50,
      amount,
      payout
    );

    for (let i = 0; i < txs.length; i++) {
      const tx = txs[i];
      await sendTransactionForTest(connection, tx, [USER]);
    }

    const token1Account = await getOrCreateAssociatedTokenAccount(
      connection,
      USER,
      findMintPda(sdk.program.programId)[0],
      user1.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const token2Account = await getOrCreateAssociatedTokenAccount(
      connection,
      USER,
      findMintPda(sdk.program.programId)[0],
      user2.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    const { account: newAccount } = await sdk.fetchUserInfo(USER.publicKey);
    const { account: user1Account } = await sdk.fetchUserInfo(user1.publicKey);
    const { account: user2Account } = await sdk.fetchUserInfo(user2.publicKey);

    expect(new BN(token1Account.amount.toString()) == user1Account.balance);
    expect(new BN(token2Account.amount.toString()) == user2Account.balance);
    expect(user1Account.balance == amount);
    expect(user2Account.balance == amount);
  });

  describe("Sell Tokens", () => {
    let tokenAccount: Account;
    beforeAll(async () => {
      tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        user1,
        findMintPda(sdk.program.programId)[0],
        user1.publicKey,
        false,
        undefined,
        undefined,
        TOKEN_2022_PROGRAM_ID
      );
    });
    it("Can't Sell Token when in initial phase", async () => {
      const balance = new BN(tokenAccount.amount.toString());
      const sellAmount = balance.div(new BN(2));
      //   const nativeOldBalance = await connection.getBalance(user1.publicKey);

      expect(balance.toNumber()).toBeGreaterThan(0);

      const tx = await sdk.createSellTx(user1.publicKey, sellAmount);

      try {
        await sendTransactionForTest(connection, tx, [user1]);
      } catch (err: any) {
        expect(err).toBeInstanceOf(SendTransactionError);
        expect(
          (err.logs as Array<string>).join().includes("It is in initial phase")
        ).toBeTruthy();
      }
    });
    it("Can't Sell Locked Token", async () => {
      const balance = new BN(tokenAccount.amount.toString());
      const sellAmount = balance.div(new BN(2));
      const nativeOldBalance = await connection.getBalance(user1.publicKey);

      expect(balance.toNumber()).toBeGreaterThan(0);

      const txDisable = await sdk.createDisableInitialStageTx(USER.publicKey);

      await sendTransactionForTest(connection, txDisable, [USER]);

      const tx = await sdk.createSellTx(user1.publicKey, sellAmount);

      try {
        await sendTransactionForTest(connection, tx, [user1]);
      } catch (err: any) {
        expect(err).toBeInstanceOf(SendTransactionError);
        console.log(Object.keys(err));
        expect(
          (err.logs as Array<string>)
            .join()
            .includes("You do not have enough funds")
        ).toBeTruthy();
      }
    });
    it("Can Sell Token when token is unlocked @TODO", async () => {
      //   const balance = new BN(tokenAccount.amount.toString());
      //   const sellAmount = balance.div(new BN(2));
      //   const nativeOldBalance = await connection.getBalance(user1.publicKey);

      //   expect(balance.toNumber()).toBeGreaterThan(0);

      //   const tx = await sdk.createSellTx(user1.publicKey, sellAmount);

      //   await sendTransactionForTest(connection, tx, [user1]);

      //   const nativeNewBalance = await connection.getBalance(user1.publicKey);
      //   const { account: program } = await sdk.fetchProgramInfo();
      //   const { account: user } = await sdk.fetchUserInfo(user1.publicKey);
      //   const { tokenSupply, contractBalance } = program;

      //   expect(user.balance == balance.sub(sellAmount));
      //   expect(tokenSupply == user.balance);
      //   expect(
      //     new BN(LAMPORTS_PER_SOL).sub(
      //       new BN((nativeNewBalance - nativeOldBalance).toString())
      //     ) == contractBalance
      //   );

      expect(true);
    });
  });

  describe("Can call readonly functions", () => {
    it("My dividends", async () => {
      const { value } = await sdk.myDividends(user1.publicKey, true);

      expect(value.toNumber()).toBeGreaterThan(0);
    });

    it("Sell Price", async () => {
      const { value } = await sdk.sellPrice();

      expect(value.toNumber()).toBeGreaterThan(0);
    });

    it("Buy price", async () => {
      const { value } = await sdk.buyPrice();

      expect(value.toNumber()).toBeGreaterThan(0);
    });

    it("calculate lamports to receive for x tokens", async () => {
      const { value } = await sdk.calculateLamportsReceived(
        new BN(LAMPORTS_PER_SOL)
      );
      expect(value.toNumber()).toBeGreaterThan(0);
    });

    it("calculate tokens to receive for x lamports", async () => {
      const { value } = await sdk.calculateTokensReceived(
        new BN(LAMPORTS_PER_SOL)
      );
      expect(value.toNumber()).toBeGreaterThan(0);
    });
  });
});
