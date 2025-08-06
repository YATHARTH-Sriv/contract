use anchor_lang::prelude::*;


// Account Data Structures
#[account]
#[derive(InitSpace)]
pub struct Platform {
    pub authority: Pubkey,
    pub total_rooms: u64,
    pub total_users: u64,
    pub treasury: Pubkey,
    pub yap_mint: Pubkey,
    pub bump: u8,
}