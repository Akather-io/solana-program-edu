use anchor_lang::prelude::*;

pub mod constants;
pub use constants::*;

pub mod instructions;
pub use instructions::*;

pub mod schemas;
pub use schemas::*;

pub mod errors;

declare_id!("3WnkE7WM2yM6fPL3FtV4geVXxFq6FLm86kMvP9mrwdg8");

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
    ) -> Result<()> {
        instructions::course::create_course::handler(ctx, id, name, description, instructor, price)
    }

    pub fn enroll(ctx: Context<CourseEnroll>, course_id: u64) -> Result<()> {
        instructions::course::enroll::handler(ctx, course_id)
    }
}
