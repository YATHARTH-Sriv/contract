use anchor_lang::prelude::*;
use crate::constant::*;
use crate::{error::ErrorCode, AttendanceRole, ListeningSession, SpaceAttendance, User, YapRoom};


#[derive(Accounts)]
#[instruction(room_id: u64)]
pub struct LeaveRoom<'info> {
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
        mut,
        seeds = [b"listening_session", room_id.to_le_bytes().as_ref(), payer.key().as_ref()],
        bump = listening_session.bump
    )]
    pub listening_session: Account<'info, ListeningSession>,

    #[account(mut)]
    pub payer: Signer<'info>,
}

 // Leave room as listener
    pub fn leave_room(ctx: Context<LeaveRoom>, room_id: u64) -> Result<()> {
        let room = &mut ctx.accounts.yap_room;
        let user = &mut ctx.accounts.user_profile;
        let listening_session = &mut ctx.accounts.listening_session;
        let clock = Clock::get()?;

        require!(listening_session.ended_at.is_none(), ErrorCode::SessionAlreadyEnded);
        require!(room.current_listeners > 0, ErrorCode::NoListenersToRemove);

        room.current_listeners -= 1;
        listening_session.ended_at = Some(clock.unix_timestamp);

        // Calculate listening time and rewards
        let listening_time_minutes = (clock.unix_timestamp - listening_session.started_at) / 60;
        let rewards = calculate_listening_rewards(
            listening_time_minutes as u64,
            listening_session.engagement_score,
            user.reputation_score,
            user.staked_amount,
        );

        listening_session.rewards_earned = rewards;
        user.total_listening_time += listening_time_minutes as u64;
        user.total_yap_earned += rewards;

        // Update user's space attendance record
        let attendance = SpaceAttendance {
            room_id,
            time: listening_time_minutes,
            role: AttendanceRole::Listener,
            engagement_score: listening_session.engagement_score,
        };

        if user.space_attended_and_time.len() >= 200 {
            user.space_attended_and_time.remove(0);
        }
        user.space_attended_and_time.push(attendance);

        // emit!(UserLeftRoom {
        //     user: ctx.accounts.payer.key(),
        //     room_id,
        //     left_at: clock.unix_timestamp,
        //     listening_time_minutes: listening_time_minutes as u64,
        //     rewards_earned: rewards,
        // });

        Ok(())
    }


// Helper function to calculate listening rewards
fn calculate_listening_rewards(
    listening_time_minutes: u64,
    engagement_score: u8,
    user_reputation: u64,
    staked_amount: u64,
) -> u64 {
    let base_reward = listening_time_minutes * BASE_REWARD_PER_MINUTE;
    
    // Engagement multiplier (50% to 200%)
    let engagement_multiplier = 50 + (engagement_score as u64 * 15);
    
    // Reputation bonus (up to 50% bonus)
    let reputation_bonus = std::cmp::min(user_reputation / 20, 50);
    
    // Staking bonus (up to 100% bonus)
    let staking_bonus = std::cmp::min(staked_amount / 10_000_000, 100); // 10 YAP staked = 1% bonus
    
    base_reward * engagement_multiplier / 100 * (100 + reputation_bonus + staking_bonus) / 100
}
