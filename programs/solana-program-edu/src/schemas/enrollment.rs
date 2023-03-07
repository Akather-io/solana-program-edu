use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Enrollment {
    pub course: Pubkey,
    pub student: Pubkey,
    pub start_date: i64,
    pub completion_date: Option<i64>,
}

impl Enrollment {
    pub const LEN: usize =
        DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH + PUBLIC_KEY_LENGTH + U64_SIZE + U64_SIZE;
}
