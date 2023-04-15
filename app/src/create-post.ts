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

  // init blog
  const [blogAccount] = PublicKey.findProgramAddressSync(
    [Buffer.from('blog'), wallet.publicKey.toBuffer()],
    new PublicKey(PROGRAM_ID)
  );

  // create post
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
  const postSig = await program.methods
    .createPost(post.author, post.slug, post.title, post.content)
    .accounts({
      blogAccount,
      postAccount,
      authority: wallet.publicKey,
      systemProgram: new PublicKey('11111111111111111111111111111111'),
    })
    .rpc();
  console.log(postSig);
};

main();
