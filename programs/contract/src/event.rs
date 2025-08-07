use anchor_lang::prelude::*;

// Events
#[event]
pub struct UserCreated {
    pub user: Pubkey,
    pub username: String,
    pub created_at: i64,
}

// #[event]
// pub struct SubscriptionPurchased {
//     pub creator: Pubkey,
//     pub subscription_type: u8,
//     pub expires_at: i64,
//     pub amount_paid: u64,
// }

// #[event]
// pub struct RoomCreated {
//     pub room_id: u64,
//     pub host: Pubkey,
//     pub title: String,
//     pub scheduled_time: i64,
//     pub duration: u64,
//     pub category: String,
// }

// #[event]
// pub struct RoomStarted {
//     pub room_id: u64,
//     pub host: Pubkey,
//     pub started_at: i64,
// }

// #[event]
// pub struct RoomEnded {
//     pub room_id: u64,
//     pub host: Pubkey,
//     pub ended_at: i64,
//     pub duration_minutes: u64,
//     pub host_earnings: u64,
//     pub peak_listeners: u32,
// }

// #[event]
// pub struct UserJoinedRoom {
//     pub user: Pubkey,
//     pub room_id: u64,
//     pub joined_at: i64,
// }

// #[event]
// pub struct UserLeftRoom {
//     pub user: Pubkey,
//     pub room_id: u64,
//     pub left_at: i64,
//     pub listening_time_minutes: u64,
//     pub rewards_earned: u64,
// }

// #[event]
// pub struct RewardsClaimed {
//     pub user: Pubkey,
//     pub amount: u64,
//     pub claimed_at: i64,
// }

// #[event]
// pub struct BadgeMinted {
//     pub user: Pubkey,
//     pub badge_type: u8,
//     pub minted_at: i64,
// }
