use anchor_lang::prelude::*;

use crate::AttendanceRole;

#[account]
#[derive(InitSpace)]
pub struct CreatorProfile {
    pub owner: Pubkey,
    pub subscription_expires: i64,
    pub subscription_type: u8,
    
    // Earnings & Rewards
    pub total_yap_earned: u64,
    pub total_subscription_revenue: u64,
    pub pending_rewards: u64,
    
    // Creator Statistics
    pub reputation_score: u64,
    pub total_rooms_hosted: u64,
    pub total_hosting_minutes: u64,
    pub average_room_rating: u32,
    
    // Room History
    #[max_len(100)]
    pub hosted_rooms: Vec<RoomRecord>,
    #[max_len(200)]
    pub attended_rooms: Vec<AttendanceRecord>,
    
    pub created_at: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone)]
pub struct RoomRecord {
    pub room_id: u64,
    pub hosted_at: i64,
    pub duration_minutes: u64,
    pub peak_listeners: u32,
    pub total_speakers: u32,
    pub yap_earned: u64,
    pub room_rating: u32,
}


#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone)]
pub struct AttendanceRecord {
    pub room_id: u64,
    pub room_host: Pubkey,
    pub attended_at: i64,
    pub role: AttendanceRole,
    pub minutes_present: u64,
    pub minutes_speaking: u64,
    pub engagement_score: u8,
}

