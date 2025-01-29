use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // Implemens the InitSpace trait without the anchor discriminator, so you still need pass that in
pub struct Escrow {
    pub seed: u64, // seed so that the user can have more than one escrow account.
    pub maker: Pubkey, // Maker pubkey
    pub mint_a: Pubkey, // mint that the maker wants to trade
    pub mint_b: Pubkey, // mint that the maker wants to receive
    pub receive: u64, // amount of mint_b that the maker wants to receive
    pub bump: u8, // bump of our escrow account
}