import * as anchor from '@project-serum/anchor';
import { setupInitalState } from "./01-setup";
import { createPlayers, spawnPlayers } from "./02-players";
import { initLocBySpawn } from './03-movement';

async function happyPath(){
  //setup
  let setup = await setupInitalState('happypath-game');

  //players
  const players = await createPlayers(setup, 2);

  //spawn player
  const spawnLocations = await spawnPlayers(setup, players);

  //initalize some more locations
  const locations = await initLocBySpawn(setup, spawnLocations);
}

describe("Legacy Test Suite", () => {
  it("checks happypath", async () => {
    await happyPath();
  })
})