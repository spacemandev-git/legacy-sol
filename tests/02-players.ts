import * as anchor from '@project-serum/anchor';
const {SystemProgram} = anchor.web3;

//Tests that players can be created
//Players can generate a start location (the map grows clockwise from max x,y)

import { getPDA } from './util';
import * as I from './interfaces';

export async function createPlayers(setup:I.Setup, amtPlayers:number){
  const keys = [];
  
  for(let i=0; i<amtPlayers;i++){
    keys.push(anchor.web3.Keypair.generate());
  }

  let players:I.PDA[] = [];
  for(let [i, key] of keys.entries()){
    const acc = await getPDA([Buffer.from(setup.gameId), key.publicKey.toBuffer()], setup.program.programId)
    await setup.program.rpc.initPlayer(acc.bump, `Player-${i}`, {
      accounts: {
        game: setup.gameacc.account,
        playerAccount: acc.account,
        player: key.publicKey,
        payer: setup.program.provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [key]
    })
    players.push(acc);
  }
  console.log("Player Accounts Created: ");
  for(let account of players) {
    console.log("\n", account.account.toString());
  }

  return players;
}

export async function spawnPlayers(setup:I.Setup, players: I.PDA[]){
    let player_spawn_locations = {}

    for(let i=0; i<players.length; i++){
      const x = new anchor.BN(0)//.toArrayLike(Buffer, "be", 8);
      const y = new anchor.BN(i+1)//.toArrayLike(Buffer, "be", 8);
      const cX = new anchor.BN(0)//.toArrayLike(Buffer, "be", 8);
      const cY = new anchor.BN(i)//.toArrayLike(Buffer, "be", 8)
      const spawn_loc = await getPDA([Buffer.from(setup.gameId),x.toArrayLike(Buffer, "be", 8),y.toArrayLike(Buffer, "be", 8)], setup.program.programId)
      const connecting_location = await getPDA([Buffer.from(setup.gameId), cX.toArrayLike(Buffer, "be", 8), cY.toArrayLike(Buffer, "be", 8)], setup.program.programId)
      await setup.program.rpc.spawn(new anchor.BN(0), new anchor.BN(i+1), spawn_loc.bump, {
        accounts: {
          game: setup.gameacc.account,
          player: players[i].account,
          location: spawn_loc.account,
          connectingLoc: connecting_location.account,
          payer: setup.program.provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: []
      })
      player_spawn_locations[players[i].account.toString()] = {
        x: 0,
        y: i+1,
        acc: spawn_loc.account.toString()

      }
      //console.log(`Spawning Player: %s at Location (0,${i}) %s`, players[i].account.toString(), spawn_loc.account.toString());
    }
    console.log("Player Spawns: ");
    console.log(player_spawn_locations);
    return player_spawn_locations;
}