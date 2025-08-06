use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ListeningSession {
    pub user: Pubkey,
    pub room: Pubkey,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub rewards_earned: u64,
    pub engagement_score: u8,
    pub bump: u8,
}