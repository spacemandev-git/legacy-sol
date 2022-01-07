import * as anchor from '@project-serum/anchor';
import { LegacySol } from '../target/types/legacy_sol';

export const getPDA = async (accBuf: Buffer[], programId: anchor.web3.PublicKey) => {
  const [acc, accbmp] = await anchor.web3.PublicKey.findProgramAddress(accBuf, programId)
  return {account: acc, bump: accbmp};
}

export interface Setup {
  contractadmin: PDA,
  gameacc: PDA,
  startLoc: PDA,
  gameId: String,
  program: anchor.Program<LegacySol>,
}

export interface PDA {
  account: anchor.web3.PublicKey,
  bump: number
}