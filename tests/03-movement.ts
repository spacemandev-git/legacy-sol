//anyone can create a new map location as long as it's within max X,Y grid
//submitting a new location creates a new PDA account for the location which computes the features on it
//players start with troops on their start location
//players can move troops from one tile to another
//when moving troops into a tile with occupying troops combat occurs
  //test melee & ranged combat
  //test modifiers work against proper units

import * as anchor from '@project-serum/anchor';
const {SystemProgram} = anchor.web3;
  
import { getPDA } from './util';
import * as I from './interfaces';

  
export async function initLocBySpawn(setup:I.Setup, spawns:I.SpawnedPlayers){
  let locs = []
  for(let player of Object.keys(spawns)){
    let conn_loc = {
      x: new anchor.BN(spawns[player].x).toArrayLike(Buffer, "be", 8),
      y: new anchor.BN(spawns[player].y).toArrayLike(Buffer, "be", 8),
    }
    let new_loc = {
      x: new anchor.BN(spawns[player].x + 1).toArrayLike(Buffer, "be", 8),
      y: new anchor.BN(spawns[player].y).toArrayLike(Buffer, "be", 8),
    }
    conn_loc['pda'] = await getPDA([Buffer.from(setup.gameId), conn_loc.x, conn_loc.y], setup.program.programId);
    new_loc['pda'] = await getPDA([Buffer.from(setup.gameId), new_loc.x, new_loc.y], setup.program.programId);
    await setup.program.rpc.initalizeLocation(new anchor.BN(spawns[player].x + 1), new anchor.BN(spawns[player].y), new_loc['pda']['bump'], {
      accounts: {
        game: setup.gameacc.account,
        player: setup.program.provider.wallet.publicKey,
        location: new_loc['pda']['account'],
        connectingLoc: conn_loc['pda']['account'],
        systemProgram: SystemProgram.programId
      }
    });

    //Locs contains ALL posssible locations on the game board
    locs.push({x:spawns[player].x, y:spawns[player].y});
    locs.push({x:spawns[player].x +1, y:spawns[player].y});
  }
  console.log("All Locations: ");
  console.log(locs);
  return locs;
}