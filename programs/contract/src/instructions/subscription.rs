use anchor_lang::prelude::*;
use crate::{error::ErrorCode, CreatorProfile};
use anchor_spl::token::{self, Mint,Token, TokenAccount, Transfer};

pub fn subscription_purchase(
        ctx: Context<Subscription>,
        subscription_type: u8, // 1 = monthly, 2 = yearly, 3 = lifetime
    ) -> Result<()> {
        require!(subscription_type >= 1 && subscription_type <= 3, ErrorCode::InvalidSubscriptionType);

        let creator_profile = &mut ctx.accounts.creator_profile;
        let clock = Clock::get()?;

        // Payment amounts (in lamports for SOL or smallest unit for token)
        let payment_amount = match subscription_type {
            1 => 50_000_000,   // Monthly
            2 => 500_000_000,  // Yearly (10 months price)
            3 => 2000_000_000, // Lifetime
            _ => return Err(ErrorCode::InvalidSubscriptionType.into()),
        };

        // Transfer payment to treasury
        let cpi_accounts = Transfer {
            from: ctx.accounts.payer_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, payment_amount)?;

        // Set subscription expiry
        let expiry = match subscription_type {
            1 => clock.unix_timestamp + (30 * 24 * 60 * 60), // 30 days
            2 => clock.unix_timestamp + (365 * 24 * 60 * 60), // 365 days
            3 => i64::MAX, // Lifetime
            _ => return Err(ErrorCode::InvalidSubscriptionType.into()),
        };

        // Initialize or update creator profile
        creator_profile.owner = ctx.accounts.payer.key();
        creator_profile.subscription_expires = expiry;
        creator_profile.subscription_type = subscription_type;
        creator_profile.total_yap_earned = 0;
        creator_profile.total_subscription_revenue += payment_amount;
        creator_profile.pending_rewards = 0;
        creator_profile.reputation_score = 750; // Creators start with higher rep
        creator_profile.total_rooms_hosted = 0;
        creator_profile.total_hosting_minutes = 0;
        creator_profile.average_room_rating = 0;
        creator_profile.hosted_rooms = Vec::new();
        creator_profile.attended_rooms = Vec::new();
        creator_profile.created_at = clock.unix_timestamp;
        creator_profile.bump = ctx.bumps.creator_profile;

        // emit!(SubscriptionPurchased {
        //     creator: ctx.accounts.payer.key(),
        //     subscription_type,
        //     expires_at: expiry,
        //     amount_paid: payment_amount,
        // });

        Ok(())
    }


#[derive(Accounts)]
pub struct Subscription<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + CreatorProfile::INIT_SPACE,
        seeds = [b"creator", payer.key().as_ref()],
        bump
    )]
    pub creator_profile: Account<'info, CreatorProfile>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = payment_mint,
        associated_token::authority = payer
    )]
    pub payer_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = payment_mint,
        associated_token::authority = treasury
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    /// CHECK: Treasury account
    pub treasury: AccountInfo<'info>,
    pub payment_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}