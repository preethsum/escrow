use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{transfer_tokens, Offer, ANCHOR_DISCRIMINATION};

#[derive(Accounts)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
    mint::token_program = token_program,
    )]
    pub token_a_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub token_b_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_a_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_token_a_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATION + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        init,
        payer = maker,
        associated_token::mint = token_a_mint,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    pub token_a_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn transfer_tokens_to_vault(ctx: Context<MakeOffer>) -> Result<()> {
    transfer_tokens(&ctx.accounts., to, authority, token_program, mint, amount)
    Ok(())
}
