use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PollOption {
    pub option_id: u64,
    pub name: String,
    pub vote_count: u64,
}
