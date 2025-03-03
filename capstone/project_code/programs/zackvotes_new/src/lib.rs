#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;
// use states::*;

declare_id!("En82z6uBktq8s18hW8MEcUMkRaTfN3oSTJrLvaHPRrX8");

#[program]
pub mod zackvotes_new {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn create_poll(
        ctx: Context<CreatePoll>,
        description: String,
        start: u64,
        end: u64,
        option_names: Vec<String>,
    ) -> Result<()> {
        instructions::create_poll(ctx, description, start, end, option_names)
    }

    pub fn vote(ctx: Context<Vote>, poll_id: u64, option_id: u64) -> Result<()> {
        instructions::vote(ctx, poll_id, option_id)
    }
}















// #![allow(clippy::result_large_err)]
// use anchor_lang::prelude::*;

// pub mod constants;
// pub mod errors;
// pub mod instructions;
// pub mod states;

// use instructions::*;
// #[allow(unused_imports)]
// use states::*;

// declare_id!("En82z6uBktq8s18hW8MEcUMkRaTfN3oSTJrLvaHPRrX8");

// #[program]
// pub mod zackvotes_new {

//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         instructions::initialize(ctx)
//     }

//     pub fn create_poll(
//         ctx: Context<CreatePoll>,
//         description: String,
//         start: u64,
//         end: u64,
//     ) -> Result<()> {
//         instructions::create_poll(ctx, description, start, end)
//     }

//     pub fn vote(ctx: Context<Vote>, poll_id: u64, vote_option: u8) -> Result<()> {
//         instructions::vote(ctx, poll_id, vote_option)
//     }
// }