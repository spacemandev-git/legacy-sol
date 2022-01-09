import * as anchor from '@project-serum/anchor';
import { LegacySol } from '../target/types/legacy_sol';

export type Feature = {
    name: string,
    weight: anchor.BN,
    next_scan: anchor.BN
}

export type Troop = {
    name: string,
    link: string, //64  example is 63: //https://arweave.net/zt3-t8SHDSck0TLcSuC-hdQb2E0civ0DVMRgwf6sCz0
    class: any, //4
    power: anchor.BN, //8
    mod_inf: anchor.BN, //8
    mod_armor: anchor.BN, //8
    mod_air: anchor.BN, //8,
}

export enum TroopClass {
    Infantry,
    Armor,
    Aircraft
}

export interface Setup {
    contractadmin: PDA,
    gameacc: PDA,
    startLoc: PDA,
    gameId: String,
    program: anchor.Program<LegacySol>,
  }
  
  export interface PDA {
    account: anchor.web3.PublicKey,
    bump: number
  }
  
  