use anchor_lang::prelude::*;
use callee::program::Callee;

declare_id!("7LgEuEgLU4y9iPYP9iVZM7rpkA4FyfddcghPSYgJdez5");

#[program]
pub mod caller {
    use super::*;

    pub fn call_hello(ctx: Context<CallHello>) -> Result<()> {
        // Use the caller's PDA as signer for the CPI
        let seeds = &[b"caller_pda".as_ref(), &[ctx.bumps.caller_pda]];
        let signer = &[&seeds[..]];

        let cpi_program = ctx.accounts.callee_program.to_account_info();
        let cpi_accounts = callee::cpi::accounts::Hello {
            signer: ctx.accounts.caller_pda.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        callee::cpi::hello(cpi_ctx)
    }
}

#[derive(Accounts)]
pub struct CallHello<'info> {
    /// CHECK: Caller's PDA that will sign the CPI
    #[account(seeds = [b"caller_pda"], bump)]
    pub caller_pda: UncheckedAccount<'info>,
    pub callee_program: Program<'info, Callee>,
}
