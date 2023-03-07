use crate::constants::*;
use crate::errors::ErrorMessages;
use crate::{Course, Enrollment};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(course_id: u64)]
pub struct CourseEnroll<'info> {
    #[account(mut)]
    pub student: Signer<'info>,
    #[account(mut)]
    pub course: Account<'info, Course>,
    #[account(
        init_if_needed,
        seeds = [ENROLLMENT_SEED.as_ref(), course_id.to_le_bytes().as_ref(), student.key.as_ref()],
        bump,
        payer = student,
        space = Enrollment::LEN
    )]
    pub enrollment: Account<'info, Enrollment>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CourseEnroll>, course_id: u64) -> Result<()> {
    let (pda, _) = Pubkey::find_program_address(
        &[COURSE_SEED.as_ref(), course_id.to_le_bytes().as_ref()],
        ctx.program_id,
    );

    let course = &ctx.accounts.course;
    require!(
        course.to_account_info().key == &pda,
        ErrorMessages::InvalidCourseAccount
    );

    let enrollment = &mut ctx.accounts.enrollment;

    enrollment.student = *ctx.accounts.student.key;
    enrollment.course = *course.to_account_info().key;
    enrollment.start_date = Clock::get()?.unix_timestamp;
    // enrollment.completion_date = 0;
    Ok(())
}
