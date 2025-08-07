use anchor_lang::prelude::*;
use crate::{error::ErrorCode, Platform, User,event::UserCreated};


pub fn initialize_user(
        ctx: Context<UserInitialised>,
        user_name: String,
        user_username: String,
    ) -> Result<()> {
        require!(user_name.len() <= 32, ErrorCode::NameTooLong);
        require!(user_username.len() <= 32, ErrorCode::UsernameTooLong);
        require!(user_name.len() > 0, ErrorCode::NameEmpty);
        require!(user_username.len() > 0, ErrorCode::UsernameEmpty);

        let user = &mut ctx.accounts.user_created;
        let platform = &mut ctx.accounts.platform;
        let clock = Clock::get()?;

        user.user_pubkey = ctx.accounts.payer.key();
        user.user_name = user_name;
        user.user_username = user_username;
        user.rooms_created = 0;
        user.rooms_attended = 0;
        user.spoken_time = 0;
        user.total_listening_time = 0;
        user.total_yap_earned = 0;
        user.reputation_score = 500; 
        user.staked_amount = 0;
        user.is_verified = false;
        user.space_attended_and_time = Vec::new();
        user.created_at = clock.unix_timestamp;
        user.bump = ctx.bumps.user_created;

        platform.total_users += 1;

        emit!(UserCreated {
            user: ctx.accounts.payer.key(),
            username: user.user_username.clone(),
            created_at: clock.unix_timestamp,
        });

        Ok(())
    }


#[derive(Accounts)]
#[instruction(user_username: String)]
pub struct UserInitialised<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", user_username.as_bytes().as_ref()],
        bump
    )]
    pub user_created: Account<'info, User>,

    #[account(
        mut,
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,

    pub system_program: Program<'info, System>,
}
