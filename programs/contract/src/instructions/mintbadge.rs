use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, MintTo, Token, TokenAccount}};
use crate::{error::ErrorCode, Platform, User};

    // Mint badge NFT for achievements
    pub fn mint_badge_nft(ctx: Context<MintBadgeNft>, badge_type: u8) -> Result<()> {
        let user = &ctx.accounts.user_profile;
        
        // Check eligibility based on badge type
        let eligible = match badge_type {
            1 => user.rooms_attended >= 10,           // Frequent Listener
            2 => user.total_listening_time >= 1000,   // Marathon Listener (1000+ minutes)
            3 => user.rooms_created >= 5,             // Active Creator
            4 => user.reputation_score >= 900,        // High Reputation
            _ => return Err(ErrorCode::InvalidBadgeType.into()),
        };

        require!(eligible, ErrorCode::NotEligibleForBadge);

        // Mint NFT
        let cpi_accounts = MintTo {
            mint: ctx.accounts.badge_mint.to_account_info(),
            to: ctx.accounts.user_badge_account.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
        };

        let seeds = &[b"platform".as_ref(), &[ctx.accounts.platform.bump]];
        let signer = &[&seeds[..]];

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::mint_to(cpi_ctx, 1)?;

        // emit!(BadgeMinted {
        //     user: ctx.accounts.payer.key(),
        //     badge_type,
        //     minted_at: Clock::get()?.unix_timestamp,
        // });

        Ok(())
    }


#[derive(Accounts)]
pub struct MintBadgeNft<'info> {
    #[account(
        seeds = [b"user", user_profile.user_username.as_bytes().as_ref()],
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, User>,

    #[account(
        seeds = [b"platform"],
        bump = platform.bump
    )]
    pub platform: Account<'info, Platform>,

    #[account(mut)]
    pub badge_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = badge_mint,
        associated_token::authority = payer
    )]
    pub user_badge_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}