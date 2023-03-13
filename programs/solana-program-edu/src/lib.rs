use anchor_lang::prelude::*;

pub mod constants;
pub use constants::*;

pub mod instructions;
pub use instructions::*;

pub mod schemas;
pub use schemas::*;

pub mod errors;

declare_id!("E9B6Mxj284u6A66861qefD8zkMoxN3gF9krecLZSiNGi");

#[program]
pub mod solana_program_edu {

    use super::*;

    pub fn create_course(
        ctx: Context<CreateCourse>,
        id: u64,
        name: String,
        description: String,
        instructor: Pubkey,
        price: u64,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        instructions::course::create_course::handler(
            ctx,
            id,
            name,
            description,
            instructor,
            price,
            symbol,
            uri,
        )
    }

    pub fn enroll(ctx: Context<CourseEnroll>) -> Result<()> {
        instructions::course::enroll::handler(ctx)
    }
}
