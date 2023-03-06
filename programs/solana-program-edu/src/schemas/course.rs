use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Course {
    pub creator: Pubkey,
    pub id: u64,
    pub name: String,
    pub description: String,
    pub instructor: Pubkey,
    pub created_at: i64,
}

impl Course {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + U64_SIZE
        + STRING_LENGTH_PREFIX
        + MAX_COURSE_NAME_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_COURSE_DESCRIPTION_LENGTH
        + PUBLIC_KEY_LENGTH
        + TIMESTAMP_LENGTH;
}
