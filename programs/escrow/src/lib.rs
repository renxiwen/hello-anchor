mod state;
mod error;
mod constants;
mod instructions;
use anchor_lang::prelude::*;
pub use constants::*;
use instructions::take_offer::TakeOffer;
pub use state::*;
use instructions::make_offer::MakeOffer;

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
        instructions::make_offer::send_offered_tokens_to_vault(&context, token_a_offered_amount)?;
        instructions::make_offer::save_off(context, id, token_b_wanted_amount)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::send_wanted_tokens_to_maker(&context)?;
        instructions::take_offer::withdraw_and_close_vault(context)
    }}