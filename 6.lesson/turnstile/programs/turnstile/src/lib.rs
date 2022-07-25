use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// write business logic here
#[program]
pub mod turnstile {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state: &mut Account<State> = &mut ctx.accounts.state;
        state.locked = true;
        Ok(())
    }

    pub fn coin(ctx: Context<UpdateState>) -> Result<()> {
        let state: &mut Account<State> = &mut ctx.accounts.state;
        state.locked = false;
        Ok(())
    }

    pub fn push(ctx: Context<UpdateState>) -> Result<()> {
        let state: &mut Account<State> = &mut ctx.accounts.state;
        state.locked = true;
        Ok(())
    }
}

// validate incoming account here
#[derive(Accounts)]
pub struct Initialize<'info> {
    // initialize account
    #[account(
        init,
        payer = user,
        space = 8 + 1
    )]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
// for each instruction there is exactly one context
pub struct UpdateState<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
}

#[account]
pub struct State {
    pub locked: bool,
}
