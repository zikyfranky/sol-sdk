import { readFileSync } from "fs";

const seed = readFileSync("/Users/isaacfrank/.config/solana/id.json");

export default new Uint8Array(JSON.parse(seed.toString()));
