use anchor_lang::prelude::*;

use crate::{errors::ErrorMessages, Course, Enrollment};

#[derive(Accounts)]
pub struct UpdateStudent<'info> {
    #[account(mut)]
    pub course: Account<'info, Course>,
    #[account(mut)]
    pub enrollment: Account<'info, Enrollment>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<UpdateStudent>) -> Result<()> {
    let enrollment = &mut ctx.accounts.enrollment;
    let course = &mut ctx.accounts.course;

    require!(
        course.instructor.eq(&ctx.accounts.authority.key()),
        ErrorMessages::Unauthorized
    );

    enrollment.completion_date = Clock::get()?.unix_timestamp;
    msg!("Student completion date updated");

    Ok(())
}
