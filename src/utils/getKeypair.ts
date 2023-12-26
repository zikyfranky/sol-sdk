import { readFileSync } from "fs";
/**
 * This gets keypair secretKey from an ABSOLUTE file path
 * @param path absolute path to the file, relative path is not supported
 */
export default (path = `${__dirname}/../tests/id.json`) =>
  new Uint8Array(JSON.parse(readFileSync(path).toString()));
