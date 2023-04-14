import * as anchor from '@coral-xyz/anchor';

export const getProvider = (
  connection: anchor.web3.Connection,
  keypair: anchor.web3.Keypair
) => {
  const wallet = new anchor.Wallet(keypair);
  return new anchor.AnchorProvider(
    connection,
    wallet,
    anchor.AnchorProvider.defaultOptions()
  );
};
