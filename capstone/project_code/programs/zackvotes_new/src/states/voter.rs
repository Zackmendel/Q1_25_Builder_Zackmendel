use anchor_lang::prelude::*;

#[account]
pub struct Voter {
    pub poll_id: u64,
    pub has_voted: bool,
    pub vote_option: u8, // 0: Yes, 1: No, 2: abstain
}

// For space calculation you might add an INIT_SPACE constant:
impl Voter {
    pub const INIT_SPACE: usize = 8  // poll_id
        + 1                        // has_voted (bool as u8)
        + 1;                       // vote_option
}






// use anchor_lang::prelude::*;

// #[account]
// pub struct Voter {
//     pub cid: u64,
//     pub poll_id: u64,
//     pub has_voted: bool,
// }
