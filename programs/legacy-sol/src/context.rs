use anchor_lang::prelude::*;
use crate::account::*;

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
        seeds=[id.as_ref(), 0_i8.to_be_bytes().as_ref(), 0_i8.to_be_bytes().as_ref()],
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
        space=8+10000
    )]
    pub player_account: Account<'info, Player>,
    pub player: Signer<'info>,
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(x:i8, y:i8, _bmp:u8)]
pub struct SpawnPlayer<'info>{
    pub game: Account<'info, Game>,
    pub player: Account<'info, Player>,
    #[account(init, 
        seeds=[game.id.as_ref(), x.to_be_bytes().as_ref(), y.to_be_bytes().as_ref()],
        bump=_bmp,
        payer=authority,
        space=8+512
    )]
    pub location: Account<'info, Location>,
    pub connecting_loc: Account<'info, Location>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyGame<'info> {
    #[account(mut, has_one=authority)]
    pub game: Account<'info, Game>,
    pub authority: Signer<'info>
}

#[derive(Accounts)]
#[instruction(x:i8, y:i8, _bmp:u8)]
pub struct InitLoc<'info>{
    #[account(mut)]
    pub game: Account<'info, Game>,
    #[account(init,
        seeds=[game.id.as_ref(), x.to_be_bytes().as_ref(), y.to_be_bytes().as_ref()],
        bump=_bmp,
        payer=authority,
        space=8+512        
    )]
    pub location: Account<'info, Location>,
    pub connecting_loc: Account<'info, Location>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct MoveOrAttack<'info> {
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub from: Account<'info, Location>,
    #[account(mut)]
    pub destination: Account<'info, Location>,
    #[account(has_one=authority)]
    pub player: Account<'info, Player>,
    pub authority: Signer<'info>
}

#[derive(Accounts)]
#[instruction(id:u64, _bmp:u8)]
pub struct InitCard<'info>{
    #[account(mut, has_one=authority)]
    pub game: Account<'info, Game>,
    #[account(init,
        seeds=[game.id.as_ref(), id.to_be_bytes().as_ref()],
        bump=_bmp,
        space=8+512,
        payer = authority
    )]
    pub card: Account<'info, CardTemplate>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Scan<'info> {
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub location: Account<'info, Location>,
    #[account(mut, has_one=authority)]
    pub player: Account<'info, Player>,
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct Redeem<'info>{
    #[account(mut, has_one=authority)]
    pub player: Account<'info, Player>,
    pub authority: Signer<'info>,
    pub card: Account<'info, CardTemplate>
}

#[derive(Accounts)]
pub struct PlayCard<'info>{
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub location: Account<'info, Location>,
    #[account(mut, has_one=authority)]
    pub player: Account<'info, Player>,
    pub authority: Signer<'info>
}

#[derive(Accounts)]
pub struct Debug {

}