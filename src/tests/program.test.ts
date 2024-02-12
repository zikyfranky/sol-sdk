/* eslint-disable simple-import-sort/imports */
import { AnchorProvider, BN, setProvider } from "@coral-xyz/anchor";
import {
  TOKEN_2022_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PartiallyDecodedInstruction,
  PublicKey,
} from "@solana/web3.js";
import { parseInitializeIx } from "parse/parseIx";
import AppSdk from "sdk/AppSdk";
import requestAirdrops from "tests/utils/requestAirdrops";
import sendTransactionForTest from "tests/utils/sendTransactionForTest";
import invariant from "tiny-invariant";
import { findMintPda } from "utils/pdas";

import getKeyPair from "../utils/getKeypair";

const USER = Keypair.fromSecretKey(getKeyPair());
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

    const { onlyAmbassadors, isInitialized } = program;
    expect(programPda.toString()).toEqual(programPda.toString());
    expect(onlyAmbassadors).toBeTruthy();
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

  it("Can Sell Token", async () => {
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

    const balance = new BN(tokenAccount.amount.toString());
    const sellAmount = balance.div(new BN(2));
    const nativeOldBalance = await connection.getBalance(USER.publicKey);

    expect(balance.toNumber()).toBeGreaterThan(0);

    const tx = await sdk.createSellTx(USER.publicKey, sellAmount);

    await sendTransactionForTest(connection, tx, [USER]);

    const nativeNewBalance = await connection.getBalance(USER.publicKey);
    const { account: program } = await sdk.fetchProgramInfo();
    const { account: user } = await sdk.fetchUserInfo(USER.publicKey);
    const { tokenSupply, contractBalance } = program;

    expect(user.balance == balance.sub(sellAmount));
    expect(tokenSupply == user.balance);
    expect(
      new BN(LAMPORTS_PER_SOL).sub(
        new BN((nativeNewBalance - nativeOldBalance).toString())
      ) == contractBalance
    );
  });

  describe("Can call readonly functions", () => {
    it("My dividends", async () => {
      const { logs, value } = await sdk.myDividends(USER.publicKey, true);

      expect(value.toNumber()).toBeGreaterThan(0);
    });

    it("Sell Price", async () => {
      const { logs, value } = await sdk.sellPrice();

      expect(value.toNumber()).toBeGreaterThan(0);
    });

    it("Buy price", async () => {
      const { logs, value } = await sdk.buyPrice();

      expect(value.toNumber()).toBeGreaterThan(0);
    });

    it("calculate lamports to receive for x tokens", async () => {
      const { logs, value } = await sdk.calculateLamportsReceived(
        new BN(LAMPORTS_PER_SOL)
      );
      expect(value.toNumber()).toBeGreaterThan(0);
    });

    it("calculate tokens to receive for x lamports", async () => {
      const { logs, value } = await sdk.calculateTokensReceived(
        new BN(LAMPORTS_PER_SOL)
      );
      expect(value.toNumber()).toBeGreaterThan(0);
    });
  });
});
