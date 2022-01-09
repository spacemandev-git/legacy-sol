# Legacy

An in progress RTS game on Solana blockchain using the Anchor framework.


# Win Conditions
- Initialize the most Locations
- Farm the most IC cards by points
- Have the most power on board
- King of Distributed Hill
    - Control the most "Impact" Zones for the longest period of time

# Other Notes
- Every time you kill a unit, gain 1 XP, gain an IC card after N xp
- Alternativy, Kill Streak for Card, kill N troops without loosing one of your own to gain an Card

- Players can only move X every seconds, or have "Move Charges" that refresh every X seconds up to a max of X
- "Zombies Mode" with units controlled by Oracle and moved every X second
- Implement ranged attacks somehow ?

# IC Card Types
- Units
- Unit Mods (applied to deployed Units by you)
- Location Mods (Search for IC, Damage Unit on Location)
- Reset Move Charges/Next Move Timer


# Combat
- hash the timestamp to find which byte of the slot hash to use to calculate damage
