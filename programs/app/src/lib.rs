pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

use {
    crate::instructions::*,
    anchor_lang::{prelude::*, AnchorDeserialize, AnchorSerialize},
};

declare_id!("AHScsmJEzPyCPEpYqS66C3zkoEgoKscR6p4Rkp8xZoAN");

#[program]
pub mod app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }
}
