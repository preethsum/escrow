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
        token_a_amount: u64,
        token_b_amount: u64,
    ) -> Result<()> {
        make_offer::transfer_tokens_to_vault(&ctx, token_a_amount)?;
        make_offer::save_offer(ctx, token_a_amount, token_b_amount)
    }
}
