use anchor_lang::prelude::*;

#[constant]
pub const ELECTION_SEED: &[u8] = b"election";

#[constant]
pub const VOTER_SEED: &[u8] = b"vote";

pub const MAX_OPTIONS: u8 = 16;

pub const MAX_OPTION_LEN: u8 = 32;
