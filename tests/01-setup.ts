//game can be initalized
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { LegacySol } from '../target/types/legacy_sol';
import * as I from './interfaces';
import { getPDA } from './util';
import * as fs from 'fs/promises';

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

  await program.rpc.createGame(gameId, gameacc.bump, provider.wallet.publicKey, startLoc.bump,
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


  //upload Features and Troop Lists
  await program.rpc.addFeatures(await getFeatures(), {
    accounts: {
      game: gameacc.account,
      authority: provider.wallet.publicKey
    }
  })

  await program.rpc.addTroopTemplates(await getTroopList(), {
    accounts: {
      game: gameacc.account,
      authority: provider.wallet.publicKey
    }
  })
  console.log("Game Account: ", await program.account.game.fetch(gameacc.account));

  //state variables are all outputs that can be used in tests
  const setup:I.Setup = {
    contractadmin: contractadmin,
    gameacc: gameacc,
    gameId: gameId,
    program: program,
    startLoc: startLoc,
  }
  return setup;
}

async function getFeatures(){
  let raw = JSON.parse((await fs.readFile('tests/data/features.json')).toString());
  let features: I.Feature[] = []
  for(let f of raw){
    features.push( {
      weight: new anchor.BN(f.weight),
      name: f.name,
      next_scan: new anchor.BN(f.next_scan)
    })
  }

  return features;
}

async function getTroopList(){
  let templates = [];
  return templates;
}