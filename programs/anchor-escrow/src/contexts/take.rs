use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    // What other account are needed?
    // Do they need to be mutable? Initialized?
}

impl<'info> Take<'info> {
    // Cpi needed to transfer tokens from the escrow to the taker
    // The escrow is a PDA account, so we need to provide the seeds

    // let signer_seeds: [&[&[u8]]; 1] = [&[
    //     b"escrow",
    //     self.maker.to_account_info().key.as_ref(),
    //     &self.escrow.seed.to_le_bytes()[..],
    //     &[self.escrow.bump],
    // ]];
    
    
}
