use anchor_lang::prelude::*;

use crate::{ AttendanceRole};

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user_pubkey: Pubkey,
    #[max_len(32)]
    pub user_name: String,
    #[max_len(32)]
    pub user_username: String,
    pub rooms_created: u64,
    pub rooms_attended: u64,
    pub spoken_time: u64,
    pub total_listening_time: u64,
    pub total_yap_earned: u64,
    pub reputation_score: u64,
    pub staked_amount: u64,
    pub is_verified: bool,
    #[max_len(200)]
    pub space_attended_and_time: Vec<SpaceAttendance>,
    pub created_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone)]
pub struct SpaceAttendance {
    pub room_id: u64,
    pub time: i64,
    pub role: AttendanceRole,
    pub engagement_score: u8,
}
