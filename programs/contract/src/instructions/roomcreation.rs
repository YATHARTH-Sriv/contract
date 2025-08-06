use anchor_lang::prelude::*;
use crate::{error::ErrorCode, CreatorProfile};
use crate::{constant::*, Platform, User, YapRoom};

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
        require!(room_title.len() <= 128, ErrorCode::TitleTooLong);
        require!(room_title.len() > 0, ErrorCode::TitleEmpty);
        require!(description.len() <= 500, ErrorCode::DescriptionTooLong);
        require!(category.len() <= 32, ErrorCode::CategoryTooLong);
        require!(max_speakers <= 50, ErrorCode::TooManySpeakers);
        require!(max_speakers > 0, ErrorCode::NoSpeakersAllowed);
        
        let clock = Clock::get()?;
        require!(start_ts > clock.unix_timestamp, ErrorCode::InvalidScheduleTime);
        require!(duration as i64 >= MIN_ROOM_DURATION, ErrorCode::RoomTooShort);
        require!(duration as i64 <= MAX_ROOM_DURATION, ErrorCode::RoomTooLong);

        // Check creator subscription is valid
        require!(
            ctx.accounts.creator_profile.subscription_expires > clock.unix_timestamp,
            ErrorCode::SubscriptionExpired
        );

        let room = &mut ctx.accounts.yap_room;
        let creator_profile = &mut ctx.accounts.creator_profile;
        let platform = &mut ctx.accounts.platform;
        let user = &mut ctx.accounts.user_profile;

        room.host_pubkey = ctx.accounts.payer.key();
        room.room_title = room_title.clone();
        room.room_id = room_id;
        room.duration = duration;
        room.start_time = start_ts;
        room.description = description;
        room.category = category.clone();
        room.max_speakers = max_speakers;
        room.active = false;
        room.is_live = false;
        room.current_listeners = 0;
        room.peak_listeners = 0;
        room.current_speakers = 0;
        room.total_rewards_distributed = 0;
        room.host_earnings = 0;
        room.live_started_at = None;
        room.ended_at = None;
        room.total_duration = 0;
        room.total_ratings = 0;
        room.rating_count = 0;
        room.average_rating = 0;
        room.created_at = clock.unix_timestamp;
        room.bump = ctx.bumps.yap_room;

        // Update stats
        creator_profile.total_rooms_hosted += 1;
        user.rooms_created += 1;
        platform.total_rooms += 1;

        // emit!(RoomCreated {
        //     room_id,
        //     host: ctx.accounts.payer.key(),
        //     title: room_title,
        //     scheduled_time: start_ts,
        //     duration,
        //     category,
        // });

        Ok(())
    }



#[derive(Accounts)]
#[instruction(room_id: u64)]
pub struct RoomCreated<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + YapRoom::INIT_SPACE,
        seeds = [b"room", room_id.to_le_bytes().as_ref()],
        bump
    )]
    pub yap_room: Account<'info, YapRoom>,

    #[account(
        mut,
        seeds = [b"creator", payer.key().as_ref()],
        bump = creator_profile.bump
    )]
    pub creator_profile: Account<'info, CreatorProfile>,

    #[account(
        mut,
        seeds = [b"user", user_profile.user_username.as_bytes().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, User>,

    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,

    pub system_program: Program<'info, System>,
}