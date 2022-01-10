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
    name: "Scout", 
    link: "scout.json",
    class: {infantry: {}},
    power: new anchor.BN(6),
    range: new anchor.BN(1),
    modInf: new anchor.BN(0),
    modArmor: new anchor.BN(0),
    modAir: new anchor.BN(0)
  },
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
  const features = await getFeatures();

  await program.rpc.addFeatures(features, {
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
      nextScan: new anchor.BN(f.next_scan)
    })

  }
  return features;
}

async function getTroopList(){
  let templates: I.Troop[] = [];
  let raw = JSON.parse((await fs.readFile('tests/data/troop_templates.json')).toString())
  const getClass = (cls:string) => {
    let tClass = {};
    tClass[cls.toLowerCase()] = {};
    return tClass;
  }
  
  for (let t of raw){
    templates.push({
      name: t.name, 
      link: t.link,
      class: getClass(t['class']),
      power: new anchor.BN(t.power),
      range: new anchor.BN(t.range),
      modInf: new anchor.BN(t.mod_inf),
      modArmor: new anchor.BN(t.mod_armor),
      modAir: new anchor.BN(t.mod_air)
    })
  }

  return templates;
}