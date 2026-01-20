mod state;
mod error;
mod constants;
mod instructions;
pub use constants::*;
pub use state::*;
use anchor_lang::prelude::*;
use instructions::*;

declare_id!("11111111111111111111111111111111");


#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        instructions::send_offered_tokens_to_vault(&context, token_a_offered_amount)?;
        instructions::save_off(context, id, token_b_wanted_amount)
    }
}