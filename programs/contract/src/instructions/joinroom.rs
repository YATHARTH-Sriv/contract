use anchor_lang::prelude::*;

use crate::{error::ErrorCode, ListeningSession, User, YapRoom};


    // Join room as listener
    // pub fn join_room(ctx: Context<JoinRoom>, room_id: u64) -> Result<()> {
    pub fn join_room(ctx: Context<JoinRoom>) -> Result<()> {
        let room = &mut ctx.accounts.yap_room;
        let user = &mut ctx.accounts.user_profile;
        let listening_session = &mut ctx.accounts.listening_session;
        let clock = Clock::get()?;

        require!(room.is_live, ErrorCode::RoomNotLive);
        require!(room.host_pubkey != ctx.accounts.payer.key(), ErrorCode::HostCannotJoinAsListener);

        room.current_listeners += 1;
        if room.current_listeners > room.peak_listeners {
            room.peak_listeners = room.current_listeners;
        }

        user.rooms_attended += 1;

        // Initialize listening session
        listening_session.user = ctx.accounts.payer.key();
        listening_session.room = room.key();
        listening_session.started_at = clock.unix_timestamp;
        listening_session.ended_at = None;
        listening_session.rewards_earned = 0;
        listening_session.engagement_score = 5; // Start with neutral engagement
        listening_session.bump = ctx.bumps.listening_session;

        // emit!(UserJoinedRoom {
        //     user: ctx.accounts.payer.key(),
        //     room_id,
        //     joined_at: clock.unix_timestamp,
        // });

        Ok(())
    }


#[derive(Accounts)]
#[instruction(room_id: u64)]
pub struct JoinRoom<'info> {
    #[account(
        mut,
        seeds = [b"room", room_id.to_le_bytes().as_ref()],
        bump = yap_room.bump,
    )]
    pub yap_room: Account<'info, YapRoom>,

    #[account(
        mut,
        seeds = [b"user", user_profile.user_username.as_bytes().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, User>,

    #[account(
        init,
        payer = payer,
        space = 8 + ListeningSession::INIT_SPACE,
        seeds = [b"listening_session", room_id.to_le_bytes().as_ref(), payer.key().as_ref()],
        bump
    )]
    pub listening_session: Account<'info, ListeningSession>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}