use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RewardPool {
    pub room: Pubkey,
    pub total_allocated: u64,
    pub total_distributed: u64,
    pub created_at: i64,
    pub finalized_at: Option<i64>,
    pub bump: u8,
}

