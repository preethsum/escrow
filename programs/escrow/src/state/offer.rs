use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub offer_id: u64,
    pub maker: Pubkey,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub is_active: bool,
    pub bump: u8,
}
