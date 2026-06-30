use anchor_lang::prelude::*;

use crate::{error::VotingError, Election, ELECTION_SEED};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + Election::INIT_SPACE,
        seeds = [ELECTION_SEED, id.to_le_bytes().as_ref()],
        bump
    )]
    election: Account<'info, Election>,

    system_program: Program<'info, System>,
}

pub fn handle_initialize(
    ctx: Context<Initialize>,
    id: u64,
    start_timestamp: i64,
    end_timestamp: i64,
    options: Vec<String>,
) -> Result<()> {
    require!(
        options.len() > 0 && options.len() <= 16,
        VotingError::InvalidOptionsCount
    );
    require!(
        options
            .iter()
            .all(|option| option.len() > 0 && option.len() <= 32),
        VotingError::InvalidOptionName
    );
    ctx.accounts.election.id = id;
    ctx.accounts.election.start_timestamp = start_timestamp;
    ctx.accounts.election.end_timestamp = end_timestamp;
    let len = options.len();
    ctx.accounts.election.options = options;
    ctx.accounts.election.tallies = vec![0; len];
    ctx.accounts.election.bump = ctx.bumps.election;
    Ok(())
}
