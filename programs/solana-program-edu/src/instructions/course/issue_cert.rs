use anchor_lang::{prelude::*, solana_program::program::invoke};
use anchor_spl::{associated_token, token};
use mpl_token_metadata::{instruction, ID as METADATA_PROGRAM_ID};

use crate::{errors::ErrorMessages, Course, Enrollment, CERT_SEED};

#[derive(Accounts)]
pub struct IssueCert<'info> {
    #[account(mut)]
    pub course: Account<'info, Course>,
    #[account(mut)]
    pub enrollment: Account<'info, Enrollment>,
    #[account(
        init_if_needed,
        seeds = [CERT_SEED.as_ref(), course.key().as_ref(), enrollment.key().as_ref()],
        bump,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority
    )]
    pub certificate: Account<'info, token::Mint>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = certificate,
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

pub fn handler(ctx: Context<IssueCert>, uri: String) -> Result<()> {
    let enrollment = &mut ctx.accounts.enrollment;
    let course = &mut ctx.accounts.course;

    require!(
        !enrollment.completion_date.eq(&0),
        ErrorMessages::EnrollmentNotCompleted
    );

    require!(
        enrollment.issued_at.eq(&0),
        ErrorMessages::CertificateAlreadyIssued
    );

    enrollment.issued_at = Clock::get()?.unix_timestamp;
    msg!(
        "Enrollment marked as completed and certificate issued at {}",
        enrollment.issued_at
    );

    msg!(
        "Mint: {}",
        &ctx.accounts.certificate.to_account_info().key()
    );
    msg!("Token Address: {}", &ctx.accounts.token_account.key());

    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.certificate.to_account_info(),
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
        ctx.accounts.certificate.to_account_info(),
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
            ctx.accounts.certificate.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.authority.key(),
            ctx.accounts.authority.key(),
            course.name.clone(),
            course.symbol.clone(),
            uri.clone(),
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
        ctx.accounts.certificate.to_account_info(),
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
            ctx.accounts.certificate.key(),
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
