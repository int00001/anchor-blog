import { Connection, PublicKey } from '@solana/web3.js';
import * as dotenv from 'dotenv';

import { PROGRAM_ID } from 'config';
import { getProgram, getProvider, loadWallet } from 'utils';

dotenv.config();

const main = async () => {
  const connection = new Connection(process.env.ALCHEMY_SOL_DEV_HTTPS!);
  const wallet = loadWallet();

  const provider = getProvider(connection, wallet);
  const program = getProgram(provider, PROGRAM_ID);

  const [blogAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from('blog'), wallet.publicKey.toBuffer()],
    new PublicKey(PROGRAM_ID)
  );
  const sig = await program.methods
    .initializeBlog()
    .accounts({
      blogAccount,
      authority: wallet.publicKey,
      systemProgram: new PublicKey('11111111111111111111111111111111'),
    })
    .rpc();

  console.log(sig);
};

main();
