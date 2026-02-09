use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::system_program::Transfer;

declare_id!("BHaQmbreJ3Axmib5mPGuNYDbQrSV54c4jUN5God3hipG");

#[program]
pub mod cpi_demo {
    use super::*;
    
    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_account = ctx.accounts.sender.to_account_info();
        let to_account = ctx.accounts.receiver.to_account_info();
        let program_program = ctx.accounts.system_program.to_account_info();

        let cpi_context = CpiContext::new(
            program_program,
            Transfer{
                from: from_account,
                to: to_account
            }
        );
        system_program::transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(mut)]
    receiver: SystemAccount<'info>,
    
    system_program: Program<'info, System>,
}