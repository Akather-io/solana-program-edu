use crate::constants::*;
use crate::errors::ErrorMessages;
use crate::{Course, Enrollment};
use anchor_lang::solana_program::program::invoke;
use anchor_lang::{prelude::*, system_program};
use anchor_spl::{associated_token, token};
use mpl_token_metadata::{instruction, ID as METADATA_PROGRAM_ID};

#[derive(Accounts)]
pub struct CourseEnroll<'info> {
    #[account(mut)]
    pub course: Account<'info, Course>,
    #[account(
        init_if_needed,
        seeds = [ENROLLMENT_SEED.as_ref(), course.key().as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = Enrollment::LEN
    )]
    pub enrollment: Account<'info, Enrollment>,
    #[account(mut,seeds = [TREASURER_SEED.as_ref(), course.key().as_ref(), course.creator.as_ref()], bump)]
    /// CHECK: Just a pure account
    pub treasurer: AccountInfo<'info>,
    #[account(
        init_if_needed,
        seeds = [CARD_SEED.as_ref(), course.key().as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority
    )]
    pub card: Account<'info, token::Mint>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = card,
        associated_token::authority = authority,
    )]
    pub token_account: Account<'info, token::TokenAccount>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: We're about to create this with Metaplex
    pub token_metadata_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CourseEnroll>) -> Result<()> {
    let course = &ctx.accounts.course;
    let enrollment = &mut ctx.accounts.enrollment;

    let user_balance = ctx.accounts.authority.to_account_info().lamports();
    require!(
        user_balance.gt(&course.price),
        ErrorMessages::NotEnoughFunds
    );

    enrollment.student = *ctx.accounts.authority.key;
    enrollment.course = *course.to_account_info().key;
    enrollment.start_date = Clock::get()?.unix_timestamp;
    enrollment.completion_date = 0;
    enrollment.issued_at = 0;
    // enrollment.completion_date = 0;

    msg!("Transferring funds to treasurer");
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.treasurer.to_account_info(),
        },
    );
    system_program::transfer(cpi_context, course.price)?;
    msg!(
        "Funds transferred to treasurer: {}",
        &ctx.accounts.treasurer.key()
    );

    msg!("Mint: {}", &ctx.accounts.card.to_account_info().key());
    msg!("Token Address: {}", &ctx.accounts.token_account.key());

    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.card.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    token::mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1,
    )?;
    msg!("Minted NFT card student");

    msg!(
        "Creating metadata account: {}",
        &ctx.accounts.metadata.to_account_info().key()
    );

    let account_info = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.card.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    ];
    // msg!("Account Info for metadata account: {:?}", account_info);

    let creator = vec![
        mpl_token_metadata::state::Creator {
            address: course.key(),
            verified: false,
            share: 100,
        },
        mpl_token_metadata::state::Creator {
            address: ctx.accounts.authority.key(),
            verified: false,
            share: 0,
        },
    ];

    msg!("Creator Assigned to metadata account");
    invoke(
        &instruction::create_metadata_accounts_v3(
            METADATA_PROGRAM_ID,
            ctx.accounts.metadata.key(),
            ctx.accounts.card.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.authority.key(),
            course.name.clone(),
            course.symbol.clone(),
            course.uri.clone(),
            Some(creator),
            1,
            true,
            false,
            None,
            None,
            None,
        ),
        account_info.as_slice(),
    )?;
    msg!("Metadata Account Created !!!");
    let master_edition_infos = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.card.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    msg!("Master Edition Account Infos Assigned");
    invoke(
        &instruction::create_master_edition_v3(
            METADATA_PROGRAM_ID,
            ctx.accounts.master_edition.key(),
            ctx.accounts.card.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.authority.key(),
            Some(0),
        ),
        master_edition_infos.as_slice(),
    )?;
    msg!("Token mint process completed successfully.");

    Ok(())
}
