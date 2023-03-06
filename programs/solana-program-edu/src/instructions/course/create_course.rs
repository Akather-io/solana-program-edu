use crate::{schemas::course::Course, COURSE_SEED};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateCourse<'info> {
    #[account(
        init,
        seeds = [COURSE_SEED.as_ref(), id.to_le_bytes().as_ref()],
        bump ,
        payer = payer,
        space = Course::LEN
    )]
    pub course: Account<'info, Course>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateCourse>,
    id: u64,
    name: String,
    description: String,
    instructor: Pubkey,
    created_at: i64,
) -> Result<()> {
    let course = &mut ctx.accounts.course;
    course.creator = *ctx.accounts.payer.key;
    course.id = id;
    course.name = name;
    course.description = description;
    course.instructor = instructor;
    course.created_at = created_at;
    Ok(())
}
