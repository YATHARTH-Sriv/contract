use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::Platform;




pub fn handler_initialize_platform(ctx: Context<InitializePlatform>) -> Result<()> {
        let platform = &mut ctx.accounts.platform;
        
        platform.authority = ctx.accounts.authority.key();
        platform.total_rooms = 0;
        platform.total_users = 0;
        platform.treasury = ctx.accounts.treasury.key();
        platform.yap_mint = ctx.accounts.yap_mint.key();
        platform.bump = ctx.bumps.platform;
        
        Ok(())
    }


// Account structs
#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Platform::INIT_SPACE,
        seeds = [b"platform"],
        bump
    )]
    pub platform: Account<'info, Platform>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// CHECK: Treasury account for collecting payments
    pub treasury: AccountInfo<'info>,
    
    pub yap_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

