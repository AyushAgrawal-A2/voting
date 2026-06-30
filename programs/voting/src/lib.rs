pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HBWJpCrBbyVdmYTNfd7nN5p2Xd4P64acszPJTE8Sktw7");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        id: u64,
        start_timestamp: i64,
        end_timestamp: i64,
        options: Vec<String>,
    ) -> Result<()> {
        crate::instructions::initialize::handle_initialize(
            ctx,
            id,
            start_timestamp,
            end_timestamp,
            options,
        )
    }

    pub fn vote(ctx: Context<Vote>, _id: u64, option: u8) -> Result<()> {
        crate::instructions::vote::handle_vote(ctx, option)
    }
}
