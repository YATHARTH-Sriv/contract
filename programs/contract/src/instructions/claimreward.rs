use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

use crate::{error::ErrorCode, Platform, User};
    // Claim accumulated rewards
    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        let user = &mut ctx.accounts.user_profile;
        require!(user.total_yap_earned > 0, ErrorCode::NoRewardsToClaim);

        // Mint YAP tokens to user
        let cpi_accounts = MintTo {
            mint: ctx.accounts.yap_mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.platform.to_account_info(),
        };

        let seeds = &[b"platform".as_ref(), &[ctx.accounts.platform.bump]];
        let signer = &[&seeds[..]];

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::mint_to(cpi_ctx, user.total_yap_earned)?;

        let claimed_amount = user.total_yap_earned;
        user.total_yap_earned = 0; // Reset after claim

        // emit!(RewardsClaimed {
        //     user: ctx.accounts.payer.key(),
        //     amount: claimed_amount,
        //     claimed_at: Clock::get()?.unix_timestamp,
        // });

        Ok(())
    }


#[derive(Accounts)]
pub struct ClaimReward<'info> {
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

    #[account(mut)]
    pub yap_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = yap_mint,
        associated_token::authority = payer
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
