import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

export default async function ixsToTx(
  connection: Connection,
  feePayer: PublicKey,
  ixs: Array<TransactionInstruction>,
  isTest: boolean
) {
  if (isTest) {
    const tx = new Transaction();
    ixs.forEach((ix) => tx.add(ix));
    return tx;
  }

  const { blockhash, lastValidBlockHeight } =
    await connection.getLatestBlockhash("confirmed");
  const tx = new Transaction({
    blockhash: blockhash,
    feePayer: feePayer,
    lastValidBlockHeight: lastValidBlockHeight,
  });
  ixs.forEach((ix) => tx.add(ix));

  return tx;
}
