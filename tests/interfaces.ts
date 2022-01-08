import * as anchor from '@project-serum/anchor';

export type Feature = {
    name: String,
    rarity: anchor.BN,
    next_scan: anchor.BN
}

export type Troop = {
    id: String, //64  example is 63: //https://arweave.net/zt3-t8SHDSck0TLcSuC-hdQb2E0civ0DVMRgwf6sCz0
    class: TroopClass, //4
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