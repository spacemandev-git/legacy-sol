use anchor_lang::prelude::*;
use crate::account::*;

#[derive(Accounts)]
#[instruction(_bump: u8)]
pub struct Initialize<'info> {
    #[account(init, 
        seeds=[admin.key().as_ref()], bump=_bump, 
        payer=admin,
        space=8+32
    )]
    pub admin_account: Account<'info, Admin>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(id: String, _bump:u8, admin_pk: Pubkey, _0_loc_bump:u8)]
pub struct InitGame<'info> {
    pub admin_account: Account<'info, Admin>,
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(init,
        seeds=[id.as_ref()],
        bump=_bump, 
        payer=admin, 
        space=8+10000
    )]
    pub game_account: Account<'info, Game>,
    #[account(init,
        seeds=[id.as_ref(), 0_i64.to_be_bytes().as_ref(), 0_i64.to_be_bytes().as_ref()],
        bump=_0_loc_bump,
        payer=admin,
        space=8+512
    )]
    pub start_location: Account<'info, Location>,
}

#[derive(Accounts)]
#[instruction(_bump:u8)]
pub struct InitPlayer<'info> {
    pub game: Account<'info, Game>,                                                                                     
    #[account(init,
        seeds=[game.id.as_ref(), player.key().as_ref()], bump=_bump,
        payer=payer,
        space=8+512
    )]
    pub player_account: Account<'info, Player>,
    pub player: Signer<'info>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(x:i64, y:i64, _bmp:u8)]
pub struct SpawnPlayer<'info>{
    pub game: Account<'info, Game>,
    pub player: Account<'info, Player>,
    #[account(init, 
        seeds=[game.id.as_ref(), x.to_be_bytes().as_ref(), y.to_be_bytes().as_ref()],
        bump=_bmp,
        payer=payer,
        space=8+512
    )]
    pub location: Account<'info, Location>,
    pub connecting_loc: Account<'info, Location>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyGame<'info> {
    #[account(mut, has_one=authority)]
    pub game: Account<'info, Game>,
    pub authority: Signer<'info>
}

#[derive(Accounts)]
#[instruction(x:i64, y:i64, _bmp:u8)]
pub struct InitLoc<'info>{
    pub game: Account<'info, Game>,
    pub player: Signer<'info>,
    #[account(init,
        seeds=[game.id.as_ref(), x.to_be_bytes().as_ref(), y.to_be_bytes().as_ref()],
        bump=_bmp,
        payer=player,
        space=8+512        
    )]
    pub location: Account<'info, Location>,
    pub connecting_loc: Account<'info, Location>,
    pub system_program: Program<'info, System>
}