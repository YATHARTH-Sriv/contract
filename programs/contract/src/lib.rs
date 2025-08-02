use anchor_lang::prelude::*;

declare_id!("6K2rTd7Lmdhks4vDaSYDvbSG43HoVbtRX2tXj86rRG2r");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize_user(ctx:Context<UserInitialised>,user_name:String,user_username:String)->Result<()>{
        *ctx.accounts.UserCreated=User{
            user_pubkey:ctx.accounts.payer.key(),
            user_name,
            user_username,
            rooms_attended:0,
            rooms_created:0,
            space_attended_and_time:Vec::new(),
            spoken_time:0,
            bump:ctx.bumps.UserCreated
        };
        Ok(())
    }

    pub fn subscription_purchase(ctx:Context<Subscription>)->Result<()>{
        // msg!("programid",ctx.program_id);
        Ok(())
    }

    pub fn create_room(ctx:Context<RoomCreated>,room_title:String,room_id:u64,start_ts:i64,duration:u64)->Result<()>{
        *ctx.accounts.yap_room=YapRoom{
            room_id:room_id,
            room_title:room_title,
            start_time:start_ts,
            duration:duration,
            host_pubkey:ctx.accounts.payer.key(),
            active:false,
            bump:ctx.bumps.yap_room
        };
        Ok(())
    }

    pub fn start_room(ctx:Context<RoomStarted>)->Result<()>{
        let yaproom=&mut ctx.accounts.yap_room;
        yaproom.active=true;
        Ok(())
    }

    pub fn close_room(ctx: Context<CloseRoom>) -> Result<()> { 
        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        Ok(())
    }
    pub fn mint_badge_nft(ctx: Context<MintBadgeNft>) -> Result<()> { 
        Ok(())
    }   

}


#[derive(Accounts)]
#[instruction(user_username:String)]
pub struct UserInitialised<'info>{
    #[account(mut)]
    pub payer:Signer<'info>,

    #[account(
        init,
        payer = payer,
        space= 8 + User::INIT_SPACE,
        seeds=[b"newuserinthetown",user_username.as_bytes().as_ref()],
        bump
    )]
    pub UserCreated: Account<'info,User>,
    pub system_program:Program<'info,System>
}

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone)]
pub struct SpaceAttendance {
    pub room_id: u64,
    pub time: i64,
}

#[account]
#[derive(InitSpace)]
pub struct User{
    pub user_pubkey:Pubkey,
    #[max_len(32)]
    pub user_name:String,
    #[max_len(32)]
    pub user_username:String,
    pub rooms_created:u64,
    pub rooms_attended:u64,
    pub spoken_time:u64,
    #[max_len(100)]
    pub space_attended_and_time:Vec<SpaceAttendance>,
    pub bump:u8
}

#[derive(Accounts)]
pub struct Subscription{}

#[derive(Accounts)]
#[instruction(room_id:u64)]
pub struct RoomCreated<'info>{
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer=payer,
        space= 8 + YapRoom::INIT_SPACE,
        seeds=[b"Room_Creation",room_id.to_le_bytes().as_ref()],
        bump
    )]
    pub yap_room: Account<'info,YapRoom>,
    pub system_program:Program<'info,System>
}

#[account]
#[derive(InitSpace)]
pub struct YapRoom{
    pub host_pubkey:Pubkey,
    #[max_len(32)]
    pub room_title:String,
    pub room_id:u64,
    pub duration:u64,
    pub start_time:i64,
    pub active:bool,
    pub bump:u8

}


#[derive(Accounts)]
#[instruction(room_id:u64)]
pub struct RoomStarted<'info>{
    #[account(
        mut,
        seeds=[b"Room_Creation",room_id.to_le_bytes().as_ref()],
        bump = yap_room.bump,
    )]
    pub yap_room: Account<'info,YapRoom>,
    pub system_program: Program<'info,System>,
}

#[derive(Accounts)]
pub struct CloseRoom{}

#[derive(Accounts)]
pub struct ClaimReward{}

#[derive(Accounts)]
pub struct MintBadgeNft{}