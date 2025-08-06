
pub mod instructions;
pub mod error;
pub mod state;
pub mod constant;

use anchor_lang::prelude::*;
pub use instructions::*;
pub use state::*;
pub use constant::*;

declare_id!("6K2rTd7Lmdhks4vDaSYDvbSG43HoVbtRX2tXj86rRG2r");


#[program]
pub mod contract {
    use super::*;

    // Initialize platform (one-time setup)
    pub fn initialize_platform(ctx: Context<InitializePlatform>) -> Result<()> {
        instructions::initialize::handler_initialize_platform(ctx)
    }

    // User creates account
    pub fn initialize_user(
        ctx: Context<UserInitialised>,
        user_name: String,
        user_username: String,
    ) -> Result<()> {
        instructions::userinitilaize::initialize_user(ctx, user_name, user_username)
    }

    // Purchase creator subscription
    pub fn subscription_purchase(
        ctx: Context<Subscription>,
        subscription_type: u8, // 1 = monthly, 2 = yearly, 3 = lifetime
    ) -> Result<()> {
       instructions::subscription::subscription_purchase(ctx, subscription_type)
    }

    // Create room (schedule)
    pub fn create_room(
        ctx: Context<RoomCreated>,
        room_title: String,
        room_id: u64,
        start_ts: i64,
        duration: u64,
        description: String,
        category: String,
        max_speakers: u8,
    ) -> Result<()> {
       instructions::roomcreation::create_room(ctx, room_title, room_id, start_ts, duration, description, category, max_speakers)
    }

    // Start room (go live)
    pub fn start_room(ctx: Context<RoomStarted>, room_id: u64) -> Result<()> {
        instructions::startroom::start_room(ctx, room_id)
    }

    // End room
    pub fn end_room(ctx: Context<CloseRoom>, room_id: u64) -> Result<()> {
        instructions::closeroom::end_room(ctx, room_id)
    }

    // Join room as listener
    pub fn join_room(ctx: Context<JoinRoom>, room_id: u64) -> Result<()> {
       instructions::joinroom::join_room(ctx)
    }

    // Leave room as listener
    pub fn leave_room(ctx: Context<LeaveRoom>, room_id: u64) -> Result<()> {
        instructions::leaveroom::leave_room(ctx, room_id)
    }

    // Claim accumulated rewards
    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claimreward::claim_reward(ctx)
    }

    // Mint badge NFT for achievements
    pub fn mint_badge_nft(ctx: Context<MintBadgeNft>, badge_type: u8) -> Result<()> {
       instructions::mintbadge::mint_badge_nft(ctx, badge_type)
    }
}
