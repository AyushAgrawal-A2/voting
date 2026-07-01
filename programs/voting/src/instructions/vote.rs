use anchor_lang::prelude::*;

use crate::{error::VotingError, Election, Voter, ELECTION_SEED, VOTER_SEED};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Vote<'info> {
    #[account(mut)]
    voter: Signer<'info>,

    #[account(
        init,
        payer = voter,
        space = 8 + Voter::INIT_SPACE,
        seeds = [VOTER_SEED, id.to_le_bytes().as_ref(), voter.key().as_ref()],
        bump
    )]
    voter_pda: Account<'info, Voter>,

    #[account(
        mut,
        seeds = [ELECTION_SEED, id.to_le_bytes().as_ref()],
        bump = election.bump
    )]
    election: Account<'info, Election>,

    system_program: Program<'info, System>,
}

pub fn handle_vote(ctx: Context<Vote>, option: u8) -> Result<()> {
    let current_timestamp = Clock::get()?.unix_timestamp;
    require!(
        current_timestamp >= ctx.accounts.election.start_timestamp,
        VotingError::VotingNotStarted
    );
    require!(
        current_timestamp <= ctx.accounts.election.end_timestamp,
        VotingError::VotingEnded
    );
    require!(
        option < ctx.accounts.election.options.len() as u8,
        VotingError::InvalidVoteOption
    );
    ctx.accounts.election.tallies[option as usize] = ctx.accounts.election.tallies[option as usize]
        .checked_add(1)
        .ok_or(VotingError::Overflow)?;
    Ok(())
}
