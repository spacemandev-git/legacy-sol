import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { LegacySol } from '../target/types/legacy_sol';

import { setupInitalState } from "./01-setup";
import { createPlayers, spawnPlayers } from "./02-players";
import { initLocBySpawn, moveTroops } from './03-movement';

async function happyPath(){
  //setup
  let setup = await setupInitalState('happypath-game');

  //players
  const players = await createPlayers(setup, 2);

  //spawn player
  const spawnLocations = await spawnPlayers(setup, players);

  //initalize some more locations
  const locations = await initLocBySpawn(setup, spawnLocations);

  //Move units to adjacent tile
  await moveTroops(setup, locations);
}

describe("Legacy Test Suite", () => {
  it("checks happypath", async () => {
    await happyPath();
  })

  it('runs debug', async () => {
    //@ts-ignore
    const program = anchor.workspace.LegacySol as Program<LegacySol>;
    const provider = anchor.Provider.env();
    await program.rpc.debug([{x:new anchor.BN(-1)}], {accounts: {}});
  })
})
