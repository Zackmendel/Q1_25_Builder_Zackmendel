use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::ErrorCode::*;
use crate::states::{poll::Poll, voter::Voter};
use anchor_lang::prelude::*;

pub fn vote(
    ctx: Context<Vote>,
    poll_id: u64,
    option_id: u64
) -> Result<()> {
    let voter = &mut ctx.accounts.voter;
    let poll = &mut ctx.accounts.poll;

    if voter.has_voted {
        return Err(VoterAlreadyVoted.into());
    }

    let current_timestamp = Clock::get()?.unix_timestamp as u64;
    if current_timestamp < poll.start || current_timestamp > poll.end {
        return Err(PollNotActive.into());
    }

    // Validate the option id and increment vote count.
    let mut valid_option = false;
    for opt in poll.options.iter_mut() {
        if opt.option_id == option_id {
            opt.vote_count += 1;
            valid_option = true;
            break;
        }
    }
    if !valid_option {
        return Err(InvalidVoteOption.into());
    }

    // Record vote details.
    voter.poll_id = poll_id;
    voter.has_voted = true;
    voter.vote_option = option_id as u8;

    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,

    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Voter::INIT_SPACE,
        seeds = [b"voter", poll_id.to_le_bytes().as_ref(), user.key().as_ref()],
        bump
    )]
    pub voter: Account<'info, Voter>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}






























// use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
// use crate::errors::ErrorCode::*;
// use crate::states::*;
// use anchor_lang::prelude::*;

// pub fn vote(
//     ctx: Context<Vote>, 
//     poll_id: u64, 
//     vote_option: u8
//     ) -> Result<()> {
//     let voter = &mut ctx.accounts.voter;
//     let poll = &mut ctx.accounts.poll;

//     if voter.has_voted {
//         return Err(VoterAlreadyVoted.into());
//     }

//     let current_timestamp = Clock::get()?.unix_timestamp as u64;
//     if current_timestamp < poll.start || current_timestamp > poll.end {
//         return Err(PollNotActive.into());
//     }

//     // Record the vote
//     voter.poll_id = poll_id;
//     voter.has_voted = true;
//     voter.vote_option = vote_option;

//     match vote_option {
//         0 => poll.yes_votes += 1,
//         1 => poll.no_votes += 1,
//         2 => poll.abstain += 1,
//         _ => return Err(InvalidVoteOption.into()),
//     }


//     Ok(())
// }

// #[derive(Accounts)]
// #[instruction(poll_id: u64, cid: u64)]
// pub struct Vote<'info> {
//     #[account(
//         mut,
//         seeds = [poll_id.to_le_bytes().as_ref()],
//         bump
//     )]
//     pub poll: Account<'info, Poll>, // Poll to be voted in

//     #[account(
//         init, // Create the voter account if it doesn't exist
//         payer = user,
//         space = ANCHOR_DISCRIMINATOR_SIZE + 25, // Account size
//         seeds = [b"voter", poll_id.to_le_bytes().as_ref(), user.key().as_ref()],
//         bump
//     )]
//     pub voter: Account<'info, Voter>, // Unique per poll and user

//     #[account(mut)]
//     pub user: Signer<'info>, // Voter's signer account

//     pub system_program: Program<'info, System>,
// }