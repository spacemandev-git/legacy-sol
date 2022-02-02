import {parse} from 'csv-parse/sync';
import fs from 'fs/promises';
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import { LegacySol } from '../target/types/legacy_sol';
import NodeWallet from './NodeWallet';
import { bs58 } from '@project-serum/anchor/dist/cjs/utils/bytes';
// Make assumption that program has been deployed using Solana program deploy

async function createGame(name:string){
  if(!name){
    console.log("Please pass in a Game Name");
    return;
  }
  console.log(`Setting up New Game: ${name}`);

  //Fetch Features & Cards
  const features:Feature[] = JSON.parse((await fs.readFile('migrations/assets/features.json')).toString())
  const units:TroopAndMod[] = JSON.parse((await fs.readFile('migrations/assets/units.json')).toString())
  const mods:TroopAndMod[] = JSON.parse((await fs.readFile('migrations/assets/unit_mods.json')).toString())
  const idl = JSON.parse((await fs.readFile('target/idl/legacy_sol.json')).toString())
  const CONTRACT_ADDRESS = "Cz4TVYSDxwobuiKdtZY8ejp3hWL7WfCbPNYGUqnNBVSe";
  // Localhost Deploy
  const connection = new anchor.web3.Connection('http://127.0.0.1:8899');
  const keypair = anchor.web3.Keypair.generate();
  await connection.requestAirdrop(keypair.publicKey, (1e9*1000));
  console.log("Starting 25s sleep to confirm the airdrop went through");
  await new Promise(f => setTimeout(f, 25000)); //wait for airdrop to go through
  await fs.writeFile('migrations/game_admin.key', bs58.encode(keypair.secretKey));
  
  //console.log(`Provider Address: ${provider.wallet.publicKey.toBase58()}\nBalance ${(await connection.getBalance(keypair.publicKey, "finalized"))/1e9} SOL`)
  const provider = new anchor.Provider(connection, new NodeWallet(keypair), {});
  const game:Program<LegacySol> = new anchor.Program<LegacySol>(idl, CONTRACT_ADDRESS, provider);


  //Devnet Deploy
  /*
  const connection = new anchor.web3.Connection('http://api.devnet.solana.com');
  const provider = new anchor.Provider(connection, new NodeWallet(anchor.web3.Keypair.fromSecretKey(bs58.decode('2Q1ComiijcAgk5ZhrzkXB3qffFhK23TMV1tw9ZzUcHo3f3QN4q5erd2SVaq12kuX23YU6KKtnyKt53N8kVNULBVn'))), {});

  */


  //RPC New Game
  const [game_acc, game_bmp] = findProgramAddressSync([Buffer.from(name)], game.programId);
  const [start_loc, start_loc_bmp] = findProgramAddressSync([Buffer.from(name),new anchor.BN(0).toArrayLike(Buffer, "be", 1),new anchor.BN(0).toArrayLike(Buffer, "be", 1)], game.programId)
  const unit = units.find(x => x.name == "Scout")
  const rust_starting_unit = {
    name: unit.name,
    link: unit.link,
    class: {'infantry': {}},
    power: new anchor.BN(unit.power),
    range: new anchor.BN(unit.range),
    recovery: new anchor.BN(unit.recovery),
    modInf: new anchor.BN(unit.mod_inf),
    modArmor: new anchor.BN(unit.mod_armor),
    modAir: new anchor.BN(unit.mod_air)
  }
  const starting_card = {
    dropTable: {"basic": {}},
    id: new anchor.BN(0),
    cardType: {"unit": {"unit": rust_starting_unit}}
  }
  console.log(starting_card);
  await game.rpc.createGame(name, new anchor.BN(game_bmp), new anchor.BN(start_loc_bmp), starting_card, {
    accounts: {
      authority: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      gameAccount: game_acc,
      startLocation: start_loc
    }
  })
  console.log("Initalized Game");
  console.log(JSON.stringify(await game.account.game.fetch(game_acc)));

  //RPC Features

  //Need to cast it to Rust Enum
  const rust_features = features.map(f => {
    const rust_f:any = {};
    if(f.drop_table == "basic"){
      rust_f.dropTable = {"basic":{}}
    } else if(f.drop_table == "rare"){
      rust_f.dropTable = {"rare": {}}
    } else if(f.drop_table == "legendary"){
      rust_f.dropTable = {"legendary": {}}
    } else if(f.drop_table == "none"){
      rust_f.dropTable = {none:{}}
    }
    rust_f.name = f.name;
    rust_f.link = f.link;
    rust_f.scanRecovery = new anchor.BN(f.scan_recovery);
    rust_f.weight = new anchor.BN(f.weight);

    return rust_f;
  })

  await game.rpc.addFeatures(rust_features, {
    accounts: {
      game: game_acc,
      authority: provider.wallet.publicKey
    }
  })
  console.log("Added Features");
  
  let id = 1;

  let unit_promises = []
  //RPC Cards
  for(let unit of units){
    const rust_unit = {
      name: unit.name,
      link: unit.link,
      class: {[unit.class.toLowerCase()]: {}}, //{[unit.class]: {}}, //getClass(unit.class),
      power: new anchor.BN(unit.power),
      range: new anchor.BN(unit.range),
      recovery: new anchor.BN(unit.recovery),
      modInf: new anchor.BN(unit.mod_inf),
      modArmor: new anchor.BN(unit.mod_armor),
      modAir: new anchor.BN(unit.mod_air)
    }

    const card = {
        dropTable: {[unit.drop_table]: {}},//{basic: {}},// //getDropTable(unit.drop_table),
        id: new anchor.BN(id),
        cardType: {
          unit: {
            unit: rust_unit
          }
        }
    }

    const [card_acc, card_bmp] = findProgramAddressSync([Buffer.from(name), new anchor.BN(id).toArrayLike(Buffer, "be", 8)], game.programId)
    
    //@ts-ignore
    unit_promises.push(game.rpc.initCard(card, card_bmp, {
      accounts: {
        game: game_acc,
        cardAcc: card_acc,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      }
    }))
    
    id++;
  }
  await Promise.all(unit_promises);
  console.log("Units Instantiated");


  let mod_promises = [];
  for(let mod of mods){
    const rust_mod = {
      name: mod.name,
      link: mod.link,
      class: {[mod.class.toLowerCase()]: {}},
      power: new anchor.BN(mod.power),
      range: new anchor.BN(mod.range),
      recovery: new anchor.BN(mod.recovery),
      modInf: new anchor.BN(mod.mod_inf),
      modArmor: new anchor.BN(mod.mod_armor),
      modAir: new anchor.BN(mod.mod_air)
    }
    const card = {
        dropTable: {[mod.drop_table]: {}},
        id: new anchor.BN(id),
        cardType: {
          unitMod: {
            umod: rust_mod
          }
        }
    }
    const [card_acc, card_bmp] = findProgramAddressSync([Buffer.from(name), new anchor.BN(id).toArrayLike(Buffer, "be", 8)],game.programId)
    //@ts-ignore
    mod_promises.push(game.rpc.initCard(card, card_bmp, {
      accounts :{
        game: game_acc,
        cardAcc: card_acc,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      }
    }))
    id++;
  }
  await Promise.all(mod_promises);
  console.log("Uploaded Mods");
 
  //Print Game Acc and Total Cards Uploaded
  console.log(`Uploaded ${id} Cards`);
  
  const account = await game.account.game.fetch(game_acc);
  console.log("Game Account: ");
  console.log(JSON.stringify(account));
  await fs.writeFile('migrations/game_acc.json', JSON.stringify(account));
  
}

async function transformCSV(){
  const getJSON = async (fileName:string) => {
    const file = (await fs.readFile(fileName)).toString()
    const json = parse(file, {
      columns: true,
      cast: (val, ctx) => {
        if(isNaN(parseInt(val))){
          return val;
        } else {
          return parseInt(val)
        }
      }
    });
    return json;
  }  

  //Features
  const features = await getJSON('migrations/assets/features.csv');
  await fs.writeFile('migrations/assets/features.json', JSON.stringify(features));
  //Units
  const units = await getJSON('migrations/assets/units.csv');
  await fs.writeFile('migrations/assets/units.json', JSON.stringify(units));
  //UnitMods
  const mods = await getJSON('migrations/assets/unit_mods.csv');
  await fs.writeFile('migrations/assets/unit_mods.json', JSON.stringify(mods));
}

async function debug(){
  
}

//transformCSV();
createGame(process.argv[2]);

export interface TroopAndMod{
  drop_table: "none" | "basic" | "rare" | "legendary",
  name: string,
  link: string,
  class: "Infantry" | "Armor" | "Aircraft",
  power: number,
  range: number,
  recovery: number,
  mod_inf: number,
  mod_armor: number,
  mod_air: number  
}

export interface Feature {
  name: string,
  scan_recovery: number,
  weight: number,
  link: string,
  drop_table: "basic" | "rare" | "legendary"
}


//BEFORE DEPLOY SCRIPT
//rm -rf solana test validator
//solana-test-validator
//solana deploy target/deploy/legacy_sol.so target/deploy/legacy_sol-keypair.json
//Deploy with local id.json THEN use a different address for managing the game deployment