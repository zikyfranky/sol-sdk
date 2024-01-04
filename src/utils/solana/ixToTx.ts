import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

export default async function ixToTx(
  connection: Connection,
  feePayer: PublicKey,
  ix: TransactionInstruction,
  isTest: boolean
) {
  if (isTest) {
    return new Transaction().add(ix);
  }

  const { blockhash, lastValidBlockHeight } =
    await connection.getLatestBlockhash("confirmed");
  const tx = new Transaction({
    blockhash: blockhash,
    feePayer: feePayer,
    lastValidBlockHeight: lastValidBlockHeight,
  }).add(ix);

  return tx;
}
