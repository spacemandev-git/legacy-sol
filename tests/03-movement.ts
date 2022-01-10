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
  let locs:I.Locs = {}
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
        authority: setup.program.provider.wallet.publicKey,
        location: new_loc['pda']['account'],
        connectingLoc: conn_loc['pda']['account'],
        systemProgram: SystemProgram.programId
      }
    });

    locs[player] = {
      spawn: spawns[player],
      adjacent: {
        x: spawns[player].x +1,
        y: spawns[player].y,
        acc: new_loc['pda']['account'].toString()
      }
    }
    //Locs contains ALL posssible locations on the game board
    //locs.push({x:spawns[player].x, y:spawns[player].y, pda: conn_loc['pda']});
    //locs.push({x:spawns[player].x +1, y:spawns[player].y, pda: new_loc['pda']});
  }
  console.log("All Locations: ");
  console.log(locs);
  return locs;
}

export async function moveTroops(setup: I.Setup, locs:I.Locs){
  let movedLocs: I.LocationPlayers = {} 
  for(let player of Object.keys(locs)){
    console.log(`\n
    !!Before Move!!
    Spawn Loc: ${JSON.stringify(await setup.program.account.location.fetch(locs[player].spawn.acc))}
    Dest Loc: ${JSON.stringify(await setup.program.account.location.fetch(locs[player].adjacent.acc))}
    `)
    await setup.program.rpc.moveTroops({
      accounts: {
        game: setup.gameacc.account,
        player: player,
        authority: setup.program.provider.wallet.publicKey,
        from: locs[player].spawn.acc,
        destination: locs[player].adjacent.acc
      }
    })
    console.log(`\n
    !!After Move!!
    Spawn Loc: ${JSON.stringify(await setup.program.account.location.fetch(locs[player].spawn.acc))}
    Dest Loc: ${JSON.stringify(await setup.program.account.location.fetch(locs[player].adjacent.acc))}
    `)
    movedLocs[player] = {
      x: locs[player].adjacent.x,
      y: locs[player].adjacent.y,
      acc: locs[player].adjacent.acc
    }
  }
  return movedLocs;
}

export async function testCombatUntilWipe(setup:I.Setup, attackingPlayer: string, attackingLoc:string, defendingLoc:string){
  console.log(`
  !!Before Attack!!
  Attacking Loc: ${JSON.stringify(await setup.program.account.location.fetch(attackingLoc))}
  Defending Loc: ${JSON.stringify(await setup.program.account.location.fetch(defendingLoc))}
  `)
  await setup.program.rpc.attack({
    accounts: {
      game: setup.gameacc.account,
      from: attackingLoc,
      destination: defendingLoc,
      player: attackingPlayer,
      authority: setup.program.provider.wallet.publicKey,
    }
  })
  console.log(`
  !!After Attack!!
  Attacking Loc: ${JSON.stringify(await setup.program.account.location.fetch(attackingLoc))}
  Defending Loc: ${JSON.stringify(await setup.program.account.location.fetch(defendingLoc))}
  `)

  console.log(`
  !!Before Attack!!
  Attacking Loc: ${JSON.stringify(await setup.program.account.location.fetch(attackingLoc))}
  Defending Loc: ${JSON.stringify(await setup.program.account.location.fetch(defendingLoc))}
  `)
  await setup.program.rpc.attack({
    accounts: {
      game: setup.gameacc.account,
      from: attackingLoc,
      destination: defendingLoc,
      player: attackingPlayer,
      authority: setup.program.provider.wallet.publicKey,
    }
  })
  console.log(`
  !!After Attack!!
  Attacking Loc: ${JSON.stringify(await setup.program.account.location.fetch(attackingLoc))}
  Defending Loc: ${JSON.stringify(await setup.program.account.location.fetch(defendingLoc))}
  `)

  console.log(`
  !!Before Attack!!
  Attacking Loc: ${JSON.stringify(await setup.program.account.location.fetch(attackingLoc))}
  Defending Loc: ${JSON.stringify(await setup.program.account.location.fetch(defendingLoc))}
  `)
  await setup.program.rpc.attack({
    accounts: {
      game: setup.gameacc.account,
      from: attackingLoc,
      destination: defendingLoc,
      player: attackingPlayer,
      authority: setup.program.provider.wallet.publicKey,
    }
  })
  console.log(`
  !!After Attack!!
  Attacking Loc: ${JSON.stringify(await setup.program.account.location.fetch(attackingLoc))}
  Defending Loc: ${JSON.stringify(await setup.program.account.location.fetch(defendingLoc))}
  `)
}