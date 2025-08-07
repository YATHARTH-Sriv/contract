use anchor_lang::prelude::*;
use crate::{constant::*, CreatorProfile, RewardPool, RoomRecord, YapRoom};
use crate::error::ErrorCode;

// End room
pub fn end_room(ctx: Context<CloseRoom>, room_id: u64) -> Result<()> {
    let room = &mut ctx.accounts.yap_room;
    let creator_profile = &mut ctx.accounts.creator_profile;
    let reward_pool = &mut ctx.accounts.reward_pool;
    let clock = Clock::get()?;

    require!(room.is_live, ErrorCode::RoomNotLive);
    require!(room.host_pubkey == ctx.accounts.payer.key(), ErrorCode::UnauthorizedHost);

    room.is_live = false;
    room.active = false;
    room.ended_at = Some(clock.unix_timestamp);

    // Calculate total room duration in minutes
    let duration_minutes = if let Some(started_at) = room.live_started_at {
        let duration_seconds = clock.unix_timestamp - started_at;
        room.total_duration = duration_seconds;
        (duration_seconds / 60) as u64
    } else {
        0
    };

    // Calculate host earnings
    let base_host_reward = duration_minutes * HOST_REWARD_PER_MINUTE;
    let engagement_bonus = (room.peak_listeners as u64) * LISTENER_BONUS_PER_HOST;
    let quality_bonus = if room.rating_count > 0 {
        (room.average_rating as u64 * duration_minutes) / 1000
    } else {
        0
    };

    let total_host_earnings = base_host_reward + engagement_bonus + quality_bonus;
    room.host_earnings = total_host_earnings;

    // Update creator profile
    creator_profile.total_yap_earned += total_host_earnings;
    creator_profile.pending_rewards += total_host_earnings;
    creator_profile.total_hosting_minutes += duration_minutes;

    // Add room record to creator's history (keep last 100)
    let room_record = RoomRecord {
        room_id,
        hosted_at: room.live_started_at.unwrap_or(clock.unix_timestamp),
        duration_minutes,
        peak_listeners: room.peak_listeners,
        total_speakers: room.current_speakers,
        yap_earned: total_host_earnings,
        room_rating: room.average_rating,
    };

    if creator_profile.hosted_rooms.len() >= 100 {
        creator_profile.hosted_rooms.remove(0);
    }
    creator_profile.hosted_rooms.push(room_record);

    // Update creator reputation
    let room_quality_score = if room.rating_count > 0 {
        room.average_rating as u64
    } else {
        500 // Neutral if no ratings
    };

    let engagement_score = std::cmp::min(1000, room.peak_listeners as u64 * 10);
    let total_rooms = creator_profile.total_rooms_hosted;
    
    if total_rooms <= 1 {
        creator_profile.reputation_score = (room_quality_score + engagement_score) / 2;
    } else {
        let existing_weight = std::cmp::min(total_rooms - 1, 10);
        let new_score = (room_quality_score + engagement_score) / 2;
        creator_profile.reputation_score = 
            (creator_profile.reputation_score * existing_weight + new_score) / (existing_weight + 1);
    }

    // Finalize reward pool
    reward_pool.finalized_at = Some(clock.unix_timestamp);

    // emit!(RoomEnded {
    //     room_id,
    //     host: ctx.accounts.payer.key(),
    //     ended_at: clock.unix_timestamp,
    //     duration_minutes,
    //     host_earnings: total_host_earnings,
    //     peak_listeners: room.peak_listeners,
    // });

    Ok(())
}


#[derive(Accounts)]
#[instruction(room_id: u64)]
pub struct CloseRoom<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"room", room_id.to_le_bytes().as_ref()],
        bump = yap_room.bump,
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
        seeds = [b"reward_pool", room_id.to_le_bytes().as_ref()],
        bump = reward_pool.bump
    )]
    pub reward_pool: Account<'info, RewardPool>,

}