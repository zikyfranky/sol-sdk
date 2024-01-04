import { AnchorProvider, BN, setProvider } from "@coral-xyz/anchor";
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
    const tx = await sdk.createInitializeTx(USER.publicKey);

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
    expect(parsedIx).not.toBeNull();
    invariant(parsedIx != null);
    const ixAccounts = parsedIx.accounts;
    expect(ixAccounts.admin.toString()).toEqual(USER.publicKey.toString());
    expect(ixAccounts.admin_data.toString()).toEqual(
      userDataAddress.toString()
    );
    expect(ixAccounts.program_data.toString()).toEqual(
      programDataAddress.toString()
    );
  });

  describe("Can call readonly functions", () => {
    it("My dividends", async () => {
      const value = await sdk.myDividends(USER.publicKey, true);

      expect(value.toNumber()).toEqual(0);
    });

    it("Sell Price", async () => {
      const value = await sdk.sellPrice();

      expect(value.toNumber()).toEqual(0);
    });

    it("Buy price", async () => {
      const value = await sdk.buyPrice();

      expect(value.toNumber()).toBeGreaterThan(0);
    });

    it("calculate lamports to receive for x tokens", async () => {
      const value = await sdk.calculateLamportsReceived(
        new BN(LAMPORTS_PER_SOL)
      );

      expect(value.toNumber()).toEqual(0);
    });

    it("calculate tokens to receive for x lamports", async () => {
      const value = await sdk.calculateTokensReceived(new BN(LAMPORTS_PER_SOL));
      console.log(value.toString());

      expect(value.toNumber()).toEqual(0);
    });
  });
});
