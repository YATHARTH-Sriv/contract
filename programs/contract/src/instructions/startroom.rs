use anchor_lang::prelude::*;

use crate::{error::ErrorCode, RewardPool, YapRoom};
    
// Start room (go live)
pub fn start_room(ctx: Context<RoomStarted>, _room_id: u64) -> Result<()> {
    let room = &mut ctx.accounts.yap_room;
    let clock = Clock::get()?;

    require!(!room.is_live, ErrorCode::RoomAlreadyLive);
    require!(room.host_pubkey == ctx.accounts.payer.key(), ErrorCode::UnauthorizedHost);

    // Check if it's time to start (within 1 hour of scheduled time)
    let time_diff = (clock.unix_timestamp - room.start_time).abs();
    require!(time_diff <= 3600, ErrorCode::NotTimeToStart);

    room.is_live = true;
    room.active = true;
    room.live_started_at = Some(clock.unix_timestamp);

    // Initialize reward pool
    let reward_pool = &mut ctx.accounts.reward_pool;
    reward_pool.room = room.key();
    reward_pool.total_allocated = 0;
    reward_pool.total_distributed = 0;
    reward_pool.created_at = clock.unix_timestamp;
    reward_pool.finalized_at = None;
    reward_pool.bump = ctx.bumps.reward_pool;

    // emit!(RoomStarted {
    //     room_id,
    //     host: ctx.accounts.payer.key(),
    //     started_at: clock.unix_timestamp,
    // });

    Ok(())
}



#[derive(Accounts)]
#[instruction(room_id: u64)]
pub struct RoomStarted<'info> {
    #[account(
        mut,
        seeds = [b"room", room_id.to_le_bytes().as_ref()],
        bump = yap_room.bump,
    )]
    pub yap_room: Account<'info, YapRoom>,

    #[account(
        init,
        payer = payer,
        space = 8 + RewardPool::INIT_SPACE,
        seeds = [b"reward_pool", room_id.to_le_bytes().as_ref()],
        bump
    )]
    pub reward_pool: Account<'info, RewardPool>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}