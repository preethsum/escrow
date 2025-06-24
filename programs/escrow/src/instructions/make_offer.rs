use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{transfer_tokens, Offer, ANCHOR_DISCRIMINATION};

#[derive(Accounts)]
#[instruction(offer_id: u64)]
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
        seeds = [b"offer", maker.key().as_ref(), offer_id.to_le_bytes().as_ref()],
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

pub fn transfer_tokens_to_vault(ctx: &Context<MakeOffer>, amount: u64) -> Result<()> {
    transfer_tokens(
        &ctx.accounts.maker_token_a_account,
        &ctx.accounts.token_a_vault,
        &ctx.accounts.maker,
        &ctx.accounts.token_program,
        &ctx.accounts.token_a_mint,
        amount,
    )
}

pub fn save_offer(
    ctx: Context<MakeOffer>,
    offer_id: u64,
    token_a_amount: u64,
    token_b_amount: u64,
) -> Result<()> {
    let offer = &mut ctx.accounts.offer;
    offer.offer_id = offer_id;
    offer.maker = ctx.accounts.maker.key();
    offer.token_a_mint = ctx.accounts.token_a_mint.key();
    offer.token_b_mint = ctx.accounts.token_b_mint.key();
    offer.token_a_amount = token_a_amount;
    offer.token_b_amount = token_b_amount;
    offer.is_active = true;
    offer.bump = ctx.bumps.offer;

    Ok(())
}
