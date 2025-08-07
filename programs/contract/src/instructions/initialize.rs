use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use crate::Platform;

pub fn handler_initialize_platform(ctx: Context<InitializePlatform>) -> Result<()> {
        let platform = &mut ctx.accounts.platform;
        
        platform.authority = ctx.accounts.authority.key();
        platform.total_rooms = 0;
        platform.total_users = 0;
        platform.treasury = ctx.accounts.treasury.key();
        platform.yap_mint = ctx.accounts.yap_mint.key();
        platform.bump = ctx.bumps.platform;
        platform.treasury_bump=ctx.bumps.treasury;
        
        Ok(())
    }


// Account structs
#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Platform::INIT_SPACE,
        seeds = [b"platform"],
        bump
    )]
    pub platform: Account<'info, Platform>,
    pub yap_mint: InterfaceAccount<'info, Mint>,
     #[account(
        init,
        token::mint = yap_mint,
        token::authority = authority,
        payer = authority,
        seeds = [b"yaphouse_treasury"],
        bump
    )]
    
    pub treasury: InterfaceAccount<'info,TokenAccount>,   
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

