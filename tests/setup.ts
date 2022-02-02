import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import { LegacySol } from '../target/types/legacy_sol';
import fs from 'fs/promises';

describe("Legacy Test Suite", () => {
    it("setups game", async () => {
        //@ts-ignore
        const program = anchor.workspace.LegacySol as Program<LegacySol>;
        const provider = anchor.Provider.env();
        const name = "testgame";
        const features = JSON.parse((await fs.readFile('migrations/assets/features.json')).toString())
        const units = JSON.parse((await fs.readFile('migrations/assets/units.json')).toString())
        const mods = JSON.parse((await fs.readFile('migrations/assets/unit_mods.json')).toString())
      
        //RPC New Game
        const [game_acc, game_bmp] = findProgramAddressSync([Buffer.from(name)], program.programId);
        const [start_loc, start_loc_bmp] = findProgramAddressSync([Buffer.from(name),new anchor.BN(0).toArrayLike(Buffer, "be", 1),new anchor.BN(0).toArrayLike(Buffer, "be", 1)], program.programId)
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
        await program.rpc.createGame(name, new anchor.BN(game_bmp), new anchor.BN(start_loc_bmp), starting_card, {
            accounts: {
            authority: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            gameAccount: game_acc,
            startLocation: start_loc
            }
        })
        console.log("Initalized Game");
        console.log(JSON.stringify(await program.account.game.fetch(game_acc)));
    })
  
    it('runs debug', async () => {
      //@ts-ignore
      const program = anchor.workspace.LegacySol as Program<LegacySol>;
      const provider = anchor.Provider.env();
      //await program.rpc.debug([{y:new anchor.BN(-1), x: new anchor.BN(5)}], {accounts: {}});
    })
  })