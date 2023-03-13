use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Course {
    pub creator: Pubkey,
    pub id: u64,
    pub instructor: Pubkey,
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub uri: String,
    pub created_at: i64,
    pub price: u64,
}

impl Course {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + U64_SIZE
        + PUBLIC_KEY_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_COURSE_NAME_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_COURSE_DESCRIPTION_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_SYMBOL_LENGTH
        + STRING_LENGTH_PREFIX
        + MAX_URI_LENGTH
        + TIMESTAMP_LENGTH
        + U64_SIZE;
}
