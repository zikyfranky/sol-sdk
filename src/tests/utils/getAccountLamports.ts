import { Connection, PublicKey } from "@solana/web3.js";

export default async function getAccountLamports(
  connection: Connection,
  pubkey: PublicKey
): Promise<number> {
  return (await connection.getAccountInfo(pubkey))!.lamports;
}
