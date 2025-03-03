use anchor_lang::prelude::*;
use crate::states::poll_option::PollOption;

#[account]
pub struct Poll {
    pub id: u64,
    pub description: String,
    pub start: u64,
    pub end: u64,
    pub options: Vec<PollOption>,
}

impl Poll {
    pub const INIT_SPACE: usize = 672;
}


// // Update the INIT_SPACE constant accordingly if you have one defined (for example):
// impl Poll {
//     pub const INIT_SPACE: usize = 8   // id
//         + 4 + 280                  // description (4 bytes for length + 280 bytes max)
//         + 8                        // start
//         + 8                        // end
//         + 8                        // yes_votes
//         + 8                        // no_votes
//         + 8;                       // other_votes
// }






// use anchor_lang::prelude::*;

// #[account]
// #[derive(InitSpace)]
// pub struct Poll {
//     pub id: u64,
//     #[max_len(280)]
//     pub description: String,
//     pub start: u64,
//     pub end: u64,
//     pub candidates: u64,
// }
