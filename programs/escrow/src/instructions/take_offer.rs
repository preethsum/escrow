use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{transfer_tokens, Offer};

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_a_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program

    )]
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(mut,
    associated_token::mint = token_b_mint,
    associated_token::authority = taker,
     )]
    pub taker_token_b_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_a_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_token_a_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_b_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_b_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"offer", maker.key().as_ref(), offer.offer_id.to_le_bytes().as_ref()],
        bump = offer.bump,
        has_one = maker,
        has_one = token_a_mint,
        has_one = token_b_mint,
        constraint = offer.is_active == true,
        close = maker,
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
        // cannot close this
        // close = maker,
    )]
    pub token_a_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn transfer_tokens_to_taker(ctx: Context<TakeOffer>, offer_id: u64) -> Result<()> {
    let seeds = &[
        b"offer",
        ctx.accounts.maker.to_account_info().key.as_ref(),
        &offer_id.to_le_bytes()[..],
        &[ctx.accounts.offer.bump],
    ];

    let signer = [&seeds[..]];

    let cpi_accounts = TransferChecked {
        from: ctx.accounts.token_a_vault.to_account_info(),
        to: ctx.accounts.taker_token_b_account.to_account_info(),
        mint: ctx.accounts.token_b_mint.to_account_info(),
        authority: ctx.accounts.offer.to_account_info(),
    };

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        &signer,
    );

    transfer_checked(
        cpi_context,
        ctx.accounts.offer.token_b_amount,
        ctx.accounts.token_b_mint.decimals,
    )?;

    close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.token_a_vault.to_account_info(),
            destination: ctx.accounts.maker.to_account_info(),
            authority: ctx.accounts.offer.to_account_info(),
        },
        &signer,
    ))
}

pub fn transfer_tokens_to_maker(ctx: &Context<TakeOffer>) -> Result<()> {
    transfer_tokens(
        &ctx.accounts.taker_token_b_account,
        &ctx.accounts.maker_token_b_account,
        &ctx.accounts.taker,
        &ctx.accounts.token_program,
        &ctx.accounts.token_b_mint,
        ctx.accounts.offer.token_b_amount,
    )
}
