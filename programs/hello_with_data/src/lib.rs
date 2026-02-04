use anchor_lang::prelude::*;

declare_id!("9A7RG9zq2b3qzGvWdo28qbhBUEe3C54a8PiajBQwLQQ2");

#[program]
pub mod hello_with_data {
    
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        let new_account = &mut ctx.accounts.new_account;
        new_account.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>, 
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NewAccount {
    data: u64
}
