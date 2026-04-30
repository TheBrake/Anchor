use anchor_lang::prelude::*;

declare_id!("Dti1smuB44VyBRcBgfhiUKtjT8BtQcSZaWBvadCdXtmP");

#[program] // este modulo contiene la logica de nuestro programa
mod hello_anchor { //este define el modulo del programa, el nombre es el mismo que el del proyecto.
    use super::*; //Importa todo lo que esta en el modulo padre, en este caso el prelude de anchor y el declare_id.
    pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> { 
        ctx.accounts.new_account.data = data; //accede a la cuenta new_account y le asigna el valor de data
        msg!("Changed data to: {}!", data); //es como el println!
        Ok(()) //indica que la funcion terminó con exito.
    }
}
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)] //define los requisitos para que la funcion sirva.
    pub new_account: Account<'info, NewAccount>, //es el espacio donde se guardara la informacion de la cuenta.
    #[account(mut)] //debe ser muteable porque se le va a pagar la cuenta new_account.
    pub signer: Signer<'info>, //es la persona que ejecuta la funcion.
    pub system_program: Program<'info, System>, //este es un programa necesario para crear cuentas en Solana, es el programa del sistema.
}
 
#[account] //un atributo de anchor que indica que esta estructura es una cuenta de Solana.
pub struct NewAccount {
    data: u64, //fue el campo que se definió en la función initialize, es el espacio donde se guardará el valor de data.
}
