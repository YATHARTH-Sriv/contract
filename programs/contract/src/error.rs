use anchor_lang::prelude::*;
// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Name is too long")]
    NameTooLong,
    
    #[msg("Username is too long")]
    UsernameTooLong,
    
    #[msg("Name cannot be empty")]
    NameEmpty,
    
    #[msg("Username cannot be empty")]
    UsernameEmpty,
    
    #[msg("Invalid subscription type")]
    InvalidSubscriptionType,
    
    #[msg("Title is too long")]
    TitleTooLong,
    
    #[msg("Title cannot be empty")]
    TitleEmpty,
    
    #[msg("Description is too long")]
    DescriptionTooLong,
    
    #[msg("Category is too long")]
    CategoryTooLong,
    
    #[msg("Too many speakers allowed")]
    TooManySpeakers,
    
    #[msg("Must allow at least one speaker")]
    NoSpeakersAllowed,
    
    #[msg("Invalid schedule time")]
    InvalidScheduleTime,
    
    #[msg("Room duration too short")]
    RoomTooShort,
    
    #[msg("Room duration too long")]
    RoomTooLong,
    
    #[msg("Subscription has expired")]
    SubscriptionExpired,
    
    #[msg("Room is already live")]
    RoomAlreadyLive,
    
    #[msg("Not authorized to host this room")]
    UnauthorizedHost,
    
    #[msg("Not time to start the room yet")]
    NotTimeToStart,
    
    #[msg("Room is not live")]
    RoomNotLive,
    
    #[msg("Host cannot join as listener")]
    HostCannotJoinAsListener,
    
    #[msg("Session already ended")]
    SessionAlreadyEnded,
    
    #[msg("No listeners to remove")]
    NoListenersToRemove,
    
    #[msg("No rewards to claim")]
    NoRewardsToClaim,
    
    #[msg("Invalid badge type")]
    InvalidBadgeType,
    
    #[msg("Not eligible for this badge")]
    NotEligibleForBadge,
}