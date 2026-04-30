use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
 
declare_id!("Dti1smuB44VyBRcBgfhiUKtjT8BtQcSZaWBvadCdXtmP");

#[program]
pub mod my_project {
    use super::*;
    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.sender.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();
 
        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        );


        transfer(cpi_context, amount)?;
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)] //sino pongo esto Solana rechazará la transacción porque el balance del sender no se puede modificar
    sender: Signer<'info>, //obliga a que la cuenta haya firmado la transacción
    #[account(mut)]
    recipient: SystemAccount<'info>, //define el receptor de sistema
    system_program: Program<'info, System>, //valida que el programa de sistema esté incluido en la transacción
}