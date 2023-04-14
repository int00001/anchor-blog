import fs from 'fs';

import { Keypair } from '@solana/web3.js';

export const loadWallet = () => {
  const rawdata = fs.readFileSync(process.env.KEYPAIR_PATH!);
  const keypairData = JSON.parse(rawdata.toString());
  const secretKey = Uint8Array.from(keypairData);
  return Keypair.fromSecretKey(secretKey);
};
