import { setupInitalState } from "./01-setup";
import { createPlayers } from "./02-players";

async function happyPath(){
  //setup
  const setup = await setupInitalState('happypath-game');
  //players
  const players = await createPlayers(setup);
  //map  
}

describe("Legacy Test Suite", () => {
  it("checks happypath", async () => {
    happyPath();
  })
})