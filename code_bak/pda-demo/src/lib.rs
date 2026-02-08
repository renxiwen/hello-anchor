use anchor_lang::prelude::*;

declare_id!("DvAiFhGXVzqySJaUVKgJcHrS9GzN6iFVJx81m2gVTZJd");

#[program]
pub mod pda_demo {
    use super::*;
    pub fn initialize(ctx: Context<InstructionAccounts>) -> Result<()> {
        msg!("PDA: {}", ctx.accounts.pda_account.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    pub signer: Signer<'info>,
    #[account(
        seeds = [b"hello_word",signer.key().as_ref()],
        bump,
    )]
    pub pda_account: SystemAccount<'info>,
    // 因为 pda_account 是 SystemAccount，所以不需要 system_program，系统账户在存在 sol 时会自动创建
    // pub system_program: Program<'info, System>,
}
