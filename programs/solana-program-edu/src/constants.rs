use anchor_lang::prelude::Pubkey;

pub const DISCRIMINATOR_LENGTH: usize = std::mem::size_of::<u64>(); //8
pub const PUBLIC_KEY_LENGTH: usize = std::mem::size_of::<Pubkey>(); //32
pub const TIMESTAMP_LENGTH: usize = std::mem::size_of::<i64>(); //8

pub const STRING_LENGTH_PREFIX: usize = 4;
pub const BOOL_SIZE: usize = std::mem::size_of::<bool>();
pub const I64_SIZE: usize = std::mem::size_of::<i64>();
pub const U64_SIZE: usize = std::mem::size_of::<u64>();
pub const U8_SIZE: usize = std::mem::size_of::<u8>();

pub const MAX_COURSE_NAME_LENGTH: usize = 50 * 4;
pub const MAX_COURSE_DESCRIPTION_LENGTH: usize = 200 * 4;
pub const MAX_SYMBOL_LENGTH: usize = 4 * 4;
pub const MAX_URI_LENGTH: usize = 100 * 4;

pub const COURSE_SEED: &[u8] = b"course";
pub const ENROLLMENT_SEED: &[u8] = b"enrollment";
pub const CARD_SEED: &[u8] = b"card";
