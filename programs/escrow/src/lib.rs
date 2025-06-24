pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("H4spQCJhKeoNEn8naUATKgsPTJqXRSBYp6bXMcCdePte");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        offer_id: u64,
        token_a_amount: u64,
        token_b_amount: u64,
    ) -> Result<()> {
        make_offer::transfer_tokens_to_vault(&ctx, token_a_amount)?;
        make_offer::save_offer(ctx, offer_id, token_a_amount, token_b_amount)
    }

    pub fn take_offer(ctx: Context<TakeOffer>, offer_id: u64) -> Result<()> {
        take_offer::transfer_tokens_to_maker(&ctx)?;
        take_offer::transfer_tokens_to_taker(ctx, offer_id)
    }
}
