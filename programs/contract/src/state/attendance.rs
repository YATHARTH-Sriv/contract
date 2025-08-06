use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Clone)]
pub enum AttendanceRole {
    Listener,
    Speaker,
    CoHost,
}