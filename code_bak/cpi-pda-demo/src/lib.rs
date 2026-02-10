use anchor_lang::prelude::*;

declare_id!("DiA1UmunmR3kZJidoXwvsSyAyBEuR647EvYubXboQRED");

#[program]
pub mod cpi_pda_demo {
    use anchor_lang::system_program::{Transfer, transfer};

    use super::*;
    
    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.pda_account.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();
        
        //
        let seed = to_pubkey.key();
        let bump_seed = ctx.bumps.pda_account;
        let singner_seeds: &[&[&[u8]]] = &[&[b"pda",seed.as_ref(),&[bump_seed]]];
        
        let cpi_context = CpiContext::new(
            system_program,
            Transfer{
                from: from_pubkey,
                to: to_pubkey,
            }
        ).with_signer(singner_seeds);
        transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(
        mut,
        seeds = [b"pda", recipient.key().as_ref()],
        bump
    )]
    pda_account: SystemAccount<'info>,
    
    #[account(mut)]
    recipient: SystemAccount<'info>,
    
    system_program: Program<'info, System>,
}
