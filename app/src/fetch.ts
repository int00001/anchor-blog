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

  // PDAs
  const [blogAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from('blog'), wallet.publicKey.toBuffer()],
    new PublicKey(PROGRAM_ID)
  );
  const post = {
    author: wallet.publicKey,
    slug: '4s893-29',
    title: 'new post',
    content: 'long content for a post',
  };
  const [postAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from('post'), blogAccount.toBuffer(), Buffer.from(post.slug)],
    new PublicKey(PROGRAM_ID)
  );

  // fetch data from on-chain accounts
  const blogData = await program.account.blog.fetch(blogAccount);
  console.log(blogData);

  const postData = await program.account.post.fetch(postAccount);
  console.log(postData);
};

main();
