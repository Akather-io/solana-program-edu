use anchor_lang::prelude::*;

pub mod constants;
pub use constants::*;

pub mod instructions;
pub use instructions::*;

pub mod schemas;
pub use schemas::*;

pub mod errors;

declare_id!("2qFu7vVAqBQLNu4QchkeprJUBRzmVVF6v2ESv48tJkFL");

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

    pub fn update_student(ctx: Context<UpdateStudent>) -> Result<()> {
        instructions::course::update_student::handler(ctx)
    }

    pub fn issue_cert(ctx: Context<IssueCert>, uri: String) -> Result<()> {
        instructions::course::issue_cert::handler(ctx, uri)
    }
}
