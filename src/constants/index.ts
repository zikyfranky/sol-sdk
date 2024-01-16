import { PublicKey } from "@solana/web3.js";

export const USER = "users";
export const PROGRAM = "program";
export const MINT = "mint";
export const METADATA_SEED = "metadata";
export const MPL_TOKEN_METADATA_PROGRAM_ID: PublicKey = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

export const INSTRUCTIONS = {
  IX_BUY: "buy",
  IX_EXIT: "exit",
  IX_INITIALIZE: "initialize",
  IX_MY_DIVIDENDS: "my_dividends",
  IX_REINVEST: "reinvest",
  IX_SELL: "sell",
  IX_TRANSFER: "transfer",
  IX_WITHDRAW: "withdraw",
};
