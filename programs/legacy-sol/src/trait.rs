/*
use crate::account::*;

pub trait Combat {
  pub fn attack(&self, defending_troop:Troop) -> u8;
} 

impl Combat for Troop {
  pub fn attack(&self, defending_troop:Troop) -> u8 {
    let mut attack:i16 = self.power;
    if defending_troop.class == TroopClass::Infantry {
      attack += self.mod_inf
    } else if defending_troop.class == TroopClass::Armor {
      attack += self.mod_armor
    } else if defending_troop.class == TroopClass::Aircraft {
      attack += self.mod_air
    }

    if attack < 0 {
      attack = 0
    }

    return attack;
  }
}
*/