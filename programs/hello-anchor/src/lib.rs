use anchor_lang::prelude::*;

declare_id!("9c1XXAfxTqVkoR9TDvfMxmZeDvSGArSca25nCY1c8rek");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
