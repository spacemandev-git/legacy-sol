import * as anchor from '@project-serum/anchor';

export const getPDA = async (accBuf: Buffer[], programId: anchor.web3.PublicKey) => {
  const [acc, accbmp] = await anchor.web3.PublicKey.findProgramAddress(accBuf, programId)
  return {account: acc, bump: accbmp};
}