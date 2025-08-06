use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct YapRoom {
    pub host_pubkey: Pubkey,
    #[max_len(128)]
    pub room_title: String,
    pub room_id: u64,
    pub duration: u64,
    pub start_time: i64,
    #[max_len(500)]
    pub description: String,
    #[max_len(32)]
    pub category: String,
    pub max_speakers: u8,
    
    // Status
    pub active: bool,
    pub is_live: bool,
    
    // Participants
    pub current_listeners: u32,
    pub peak_listeners: u32,
    pub current_speakers: u32,
    
    // Economics
    pub total_rewards_distributed: u64,
    pub host_earnings: u64,
    
    // Timing
    pub live_started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub total_duration: i64,
    
    // Quality Metrics
    pub total_ratings: u32,
    pub rating_count: u32,
    pub average_rating: u32,
    
    pub created_at: i64,
    pub bump: u8,
}
