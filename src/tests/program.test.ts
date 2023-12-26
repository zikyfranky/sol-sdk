import { AnchorProvider, setProvider, Wallet } from "@coral-xyz/anchor";
import {
  Keypair,
  PartiallyDecodedInstruction,
  PublicKey,
} from "@solana/web3.js";
import parseInitializeIx from "parse/parseInitializeIx";
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

const sdk = new AppSdk(connection, wallet as Wallet, PROGRAM_ID);

async function getFirstAndOnlyIx(txid: string) {
  const parsedTx = await connection.getParsedTransaction(txid, "confirmed");
  const instructions = parsedTx!.transaction.message.instructions;
  expect(instructions.length).toEqual(1);
  return instructions[0];
}

describe("Program Data / Parse tests", () => {
  beforeAll(async () => {
    await requestAirdrops(connection, [USER]);

    [programDataAddress] = await sdk.findProgramPda();
    [userDataAddress] = await sdk.findUserPda(USER.publicKey);
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

    const { authority, balance, isAdmin, isAmbassador, referredBalance } = user;
    expect(userPda.toString()).toEqual(userPda.toString());
    expect(authority.toString()).toEqual(USER.publicKey.toString());
    expect(balance.toNumber()).toEqual(0);
    expect(referredBalance.toNumber()).toEqual(0);
    expect(isAdmin).toBeTruthy();
    expect(isAmbassador).toBeTruthy();
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
});
