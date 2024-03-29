use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Enrollment {
    pub course: Pubkey,
    pub student: Pubkey,
    pub start_date: i64,
    pub completion_date: i64,
    pub issued_at: i64,
}

impl Enrollment {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + PUBLIC_KEY_LENGTH
        + TIMESTAMP_LENGTH
        + TIMESTAMP_LENGTH
        + TIMESTAMP_LENGTH;
}
