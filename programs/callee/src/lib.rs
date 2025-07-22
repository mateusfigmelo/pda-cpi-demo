use anchor_lang::prelude::*;

declare_id!("8bQrHKxa2NkbnDnYtSSckJ1TLzitPsLg61c61RyFNSui");

#[program]
pub mod callee {
    use super::*;

    pub fn hello(ctx: Context<Hello>) -> Result<()> {
        msg!(
            "Hello from callee! PDA signed this CPI {}",
            ctx.accounts.signer.key()
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello<'info> {
    // Just requires any signer (PDA or regular keypair)
    pub signer: Signer<'info>,
}
