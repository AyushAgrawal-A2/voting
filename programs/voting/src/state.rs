use anchor_lang::prelude::*;

use crate::{MAX_OPTIONS, MAX_OPTION_LEN};

#[account]
#[derive(InitSpace)]
pub struct Election {
    pub id: u64,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    #[max_len(MAX_OPTIONS, MAX_OPTION_LEN)]
    pub options: Vec<String>,
    #[max_len(MAX_OPTIONS)]
    pub tallies: Vec<u64>,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Voter {}
