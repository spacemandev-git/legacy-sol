import * as anchor from '@project-serum/anchor';
import { setupInitalState } from "./01-setup";
import { createPlayers, spawnPlayers } from "./02-players";

async function happyPath(){
  //setup
  let setup = await setupInitalState('happypath-game');

  //players
  const players = await createPlayers(setup, 2);

  //spawn player
  const spawnLocations = await spawnPlayers(setup, players);
}

describe("Legacy Test Suite", () => {
  it("checks happypath", async () => {
    await happyPath();
  })
})