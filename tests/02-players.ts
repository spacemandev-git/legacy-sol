import * as anchor from '@project-serum/anchor';
const {SystemProgram} = anchor.web3;

//Tests that players can be created
//Players can generate a start location (the map grows clockwise from max x,y)

import { getPDA } from './util';

export async function createPlayers(setup){
  const keys = [anchor.web3.Keypair.generate()];

  let players = [];
  for(let [i, key] of keys.entries()){
    const acc = await getPDA([Buffer.from(setup.gameId), key.publicKey.toBuffer()], setup.program.programId)
    await setup.program.rpc.initPlayer(acc.bump, `Player-${i}`, {
      accounts: {
        game: setup.gameacc.account,
        playerAccount: acc.account,
        player: key.publicKey,
        payer: setup.provider.wallet.publicKey,
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

export async function setupStartLocations(players: anchor.web3.Keypair[]){
  
}