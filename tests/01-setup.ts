//game can be initalized
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { LegacySol } from '../target/types/legacy_sol';
import { Feature, Troop } from './interfaces';
import { getPDA, Setup } from './util';

const {SystemProgram} = anchor.web3;

export async function setupInitalState(_gid:string){
  anchor.setProvider(anchor.Provider.env());
  //@ts-ignore
  const program = anchor.workspace.LegacySol as Program<LegacySol>;
  const provider = anchor.Provider.env();

  //initalize the program
  const contractadmin = await getPDA([provider.wallet.publicKey.toBuffer()], program.programId);
  await program.rpc.initialize(contractadmin.bump, {
    accounts: {
      adminAccount: contractadmin.account,
      admin: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId
    }
  });
  console.log("Initalized System!")
  //initalize the game
  const gameId=_gid;
  const gameacc = await getPDA([Buffer.from(gameId)], program.programId);
  const startLoc = await getPDA([Buffer.from(gameId), new anchor.BN(0).toArrayLike(Buffer, "be", 8),new anchor.BN(0).toArrayLike(Buffer, "be", 8)], program.programId);

  await program.rpc.createGame(gameId, gameacc.bump, provider.wallet.publicKey, startLoc.bump, await getFeatures(), await getTroopList(),
    {
      accounts: {
        adminAccount: contractadmin.account,
        admin: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        gameAccount: gameacc.account,
        startLocation: startLoc.account,
      }
  })
  console.log("Initalized Game!");
  //state variables are all outputs that can be used in tests
  const setup:Setup = {
    contractadmin: contractadmin,
    gameacc: gameacc,
    gameId: gameId,
    program: program,
    startLoc: startLoc,
  }
  return setup;
}

async function getFeatures(){
  let features = [];
  return features;
}

async function getTroopList(){
  let templates = [];
  return templates;
}