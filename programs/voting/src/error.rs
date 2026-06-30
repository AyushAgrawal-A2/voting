use anchor_lang::prelude::*;

#[error_code]
pub enum VotingError {
    #[msg("Options must be <= 16")]
    InvalidOptionsCount,
    #[msg("Option size must be <= 32")]
    InvalidOptionName,
    #[msg("Invalid vote option")]
    InvalidVoteOption,
    #[msg("Voting not started")]
    VotingNotStarted,
    #[msg("Voting ended")]
    VotingEnded,
    #[msg("Overflow")]
    Overflow,
}
